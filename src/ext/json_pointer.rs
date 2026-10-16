//! Implementation of JSON Pointers according to [[RFC 6901](https://www.rfc-editor.org/info/rfc6901/)].

use std::borrow::Cow;

use nom::{
    IResult, Parser as _,
    branch::alt,
    bytes::complete::{is_not, tag, take_till},
    sequence::preceded,
};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error<'p> {
    /// This key was not found in the target object.
    KeyNotFound(Cow<'p, str>),
    /// This index was outside of the bounds of the target array.
    IndexNotFound(usize),
    /// Malformed index (not a non-zero integer, or `-`).
    InvalidIndex(Cow<'p, str>),
    /// The target value is not indexable by a key nor an index (not an array or object).
    NonIndexable(&'static str),
}

impl<'p> Error<'p> {
    pub fn into_static(self) -> Error<'static> {
        match self {
            Error::KeyNotFound(cow) => Error::KeyNotFound(Cow::Owned(cow.into_owned())),
            Error::IndexNotFound(index) => Error::IndexNotFound(index),
            Error::InvalidIndex(cow) => Error::InvalidIndex(Cow::Owned(cow.into_owned())),
            Error::NonIndexable(s) => Error::NonIndexable(s),
        }
    }
}

macro_rules! follow {
    ($id: ident, $get: ident $(, $mut: tt)?) => {
        /// Attempts to follow a [JSON Pointer](https://www.rfc-editor.org/info/rfc6901/),
        /// parsing lazily as required (zero-copy for non-escaped strings!).
        ///
        /// **NOTE*: syntax errors cause the parser to stop at the current value.
        pub fn $id<'r, 'p>(root: &'r $($mut)? Value, ptr: &'p str) -> Result<&'r $($mut)? Value, Error<'p>> {
            let mut current = root;
            for mut segment in pointer(ptr) {
                current = match (current, segment.next(), segment.next(), segment) {
                    // Impossible
                    (Value::Object(_), None, Some(_), _) => {
                        unreachable!("iter.next() cannot return None, then return Some(_)!")
                    }

                    // This is equivalent to indexing by an empty string "". Notice that iter.next() => iterator will forever be empty
                    (Value::Object(obj), None, None, _) => {
                        obj.$get("").ok_or(Error::KeyNotFound("".into()))
                    }
                    // Simplest case of indexing: a single non-empty verbatim string, or single escaped character
                    (Value::Object(obj), Some(single), None, _) => obj
                        .$get(single.as_str())
                        .ok_or(Error::KeyNotFound(single.as_str().into())),

                    // 1+ Tokens: string allocation required
                    (Value::Object(obj), Some(first), Some(second), rest) => {
                        // Allocate for the actual key.
                        let key = {
                            let mut buf = String::new();
                            buf.push_str(first.as_str());
                            buf.push_str(second.as_str());
                            rest.for_each(|token| buf.push_str(token.as_str()));
                            buf
                        };
                        obj.$get(&key).ok_or(Error::KeyNotFound(key.into()))
                    }

                    // `-` is always out of bounds per the spec
                    (Value::Array(arr), Some(Atom::Unescaped("-")), None, _) => {
                        Err(Error::IndexNotFound(arr.len()))
                    }

                    // Attempt to parse the index, then index into the array
                    (Value::Array(arr), Some(Atom::Unescaped(index)), None, _) => index
                        .parse::<usize>()
                        .map_err(|_| Error::InvalidIndex(Cow::Borrowed(index)))
                        .and_then(|index| arr.$get(index).ok_or(Error::IndexNotFound(index))),

                    // We know that the alternate: (None | Some(Escaped), Some(_) | None) is not a valid index, since indexes can only be digits (i.e. subset of a singular verbatim)
                    (Value::Array(_), None, _, _) => Err(Error::InvalidIndex(Cow::Borrowed(""))),
                    // The only downside of this is that the error message is technically inaccurate here, but we can say index "starting with"...
                    // I don't want to allocate another string.
                    (Value::Array(_), Some(index), _, _) => {
                        Err(Error::InvalidIndex(Cow::Borrowed(index.as_str())))
                    }

                    // Non-indexable
                    (Value::Null, ..) => Err(Error::NonIndexable("null")),
                    (Value::Bool(_), ..) => Err(Error::NonIndexable("boolean")),
                    (Value::String(_), ..) => Err(Error::NonIndexable("string")),
                    (Value::Number(_), ..) => Err(Error::NonIndexable("number")),
                }?;
            }

            Ok(current)
        }
    };
}

follow!(follow, get);
follow!(follow_mut, get_mut, mut);

/// Smallest unit for the parser: a string of verbatim characters, or an escaped character.
///
/// Makes use of two separate Escaped* variants to allow us to
/// avoid allocation for [Atom::as_str].
#[derive(Debug, Clone, Copy)]
enum Atom<'a> {
    /// A sequence of `unescaped` characters
    Unescaped(&'a str),
    /// `escaped`: `~0`
    EscapedTilde,
    /// `escaped`: `~1`
    EscapedSlash,
}

impl<'a> Atom<'a> {
    /// This atom's underlying string.
    fn as_str(self) -> &'a str {
        match self {
            Atom::Unescaped(string) => string,
            // Exploiting that 'static: 'a for any 'a:
            Atom::EscapedTilde => "~",
            Atom::EscapedSlash => "/",
        }
    }
}

/// `unescaped / escaped`
fn atom(i: &str) -> IResult<&str, Atom<'_>> {
    (alt((
        // unescaped
        is_not("/~").map(Atom::Unescaped),
        // escaped
        preceded(
            tag("~"),
            alt((
                tag("0").map(|_| Atom::EscapedTilde),
                tag("1").map(|_| Atom::EscapedSlash),
            )),
        ),
    )))
    .parse(i)
}

/// `"/" reference-token`
fn segment(i: &str) -> IResult<&str, &str> {
    preceded(tag("/"), take_till(|c| c == '/')).parse(i)
}

/// Turn any single-item nom parser into a lazy iterator.
fn iter_parser<'a, O>(
    mut input: &'a str,
    mut parser: impl FnMut(&'a str) -> IResult<&'a str, O> + 'a,
) -> impl Iterator<Item = O> + 'a {
    std::iter::from_fn(move || {
        let (rest, out) = parser(input).ok()?;
        input = rest;
        Some(out)
    })
}

/// `json_pointer = *("/" reference-token)`
fn pointer<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = Atom<'a>>> {
    iter_parser(input, segment).map(|seg| iter_parser(seg, atom))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::ext::json_pointer::follow;

    /// Examples from [RFC 6901, Chapter 5](https://www.rfc-editor.org/info/rfc6901/#section-5).
    #[test]
    fn official_examples() {
        let root = json!({ "foo": ["bar", "baz"], "": 0, "a/b": 1, "c%d": 2, "e^f": 3, "g|h": 4, "i\\j": 5, "k\"l": 6, " ": 7, "m~n": 8 });

        assert_eq!(follow(&root, ""), Ok(&root)); // The whole document
        assert_eq!(follow(&root, "/foo"), Ok(&json!(["bar", "baz"])));
        assert_eq!(follow(&root, "/foo/0"), Ok(&json!("bar")));
        assert_eq!(follow(&root, "/"), Ok(&json!(0)));
        assert_eq!(follow(&root, "/a~1b"), Ok(&json!(1)));
        assert_eq!(follow(&root, "/c%d"), Ok(&json!(2)));
        assert_eq!(follow(&root, "/e^f"), Ok(&json!(3)));
        assert_eq!(follow(&root, "/g|h"), Ok(&json!(4)));
        assert_eq!(follow(&root, "/i\\j"), Ok(&json!(5)));
        assert_eq!(follow(&root, "/k\"l"), Ok(&json!(6)));
        assert_eq!(follow(&root, "/ "), Ok(&json!(7)));
        assert_eq!(follow(&root, "/m~0n"), Ok(&json!(8)));
    }
}
