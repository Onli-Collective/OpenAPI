use std::ops::Deref;
use std::{marker::PhantomData, sync::LazyLock};

use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Deserializer, Serialize};

/// This is the root document object of the [OpenAPI document](https://spec.openapis.org/oas/v3.0.3.html#openapi-document).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenApi {
    // TODO: maybe make this the semver thingy...
    /// **REQUIRED**. This String **MUST** be the [semantic version number](https://semver.org/spec/v2.0.0.html) of the [OpenAPI Specification version](https://spec.openapis.org/oas/v3.0.3.html#versions)
    /// that the OpenAPI document uses.
    ///
    /// The [`openapi`](Self::openapi) field **SHOULD** be used by tooling specifications and clients to interpret the OpenAPI document. This is *not* related to the API [info.version](Info::version) String.
    #[serde(rename = "openapi")]
    pub openapi: String,
    ///**REQUIRED**. Provides metadata about the API. The metadata **MAY** be used by tooling as required.
    #[serde(rename = "info")]
    pub info: Info,
    /// An array of Server Objects, which provide connectivity information to a target server. If the [`servers`](Self::servers) property is not provided, or is an empty array, the default value would be a [Server Object](Server) with a [`url`](Server::url) value of `/`.
    #[serde(default, skip_serializing_if = "is_default", rename = "servers")]
    pub servers: Option<Vec<Server>>,
    ///**REQUIRED**. The available paths and operations for the API.
    #[serde(rename = "paths")]
    pub paths: Paths,
    /// An element to hold various schemas for the specification.
    #[serde(default, skip_serializing_if = "is_default", rename = "components")]
    pub components: Option<Components>,
    /// A declaration of which security mechanisms can be used across the API. The list of values includes alternative security requirement objects that can be used. Only one of the security requirement objects need to be satisfied to authorize a request. Individual operations can override this definition. To make security optional, an empty security requirement (`{}`) can be included in the array.
    #[serde(default, skip_serializing_if = "is_default", rename = "security")]
    pub security: Option<Vec<SecurityRequirement>>,
    /// A list of tags used by the specification with additional metadata. The order of the tags can be used to reflect on their order by the parsing tools. Not all tags that are used by the [Operation Object](Operation) must be declared. The tags that are not declared **MAY** be organized randomly or based on the tools’ logic. Each tag name in the list **MUST** be unique.
    #[serde(default, skip_serializing_if = "is_default", rename = "tags")]
    pub tags: Option<Vec<Tag>>,
    /// Additional external documentation.
    #[serde(default, skip_serializing_if = "is_default", rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// The object provides metadata about the API.
///
/// The metadata **MAY** be used by the clients if needed, and **MAY** be presented in editing or documentation generation tools for convenience.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Info {
    /// **REQUIRED**. The title of the API.
    #[serde(rename = "title")]
    pub title: String,
    /// A short description of the API. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API. **MUST** be in the format of a URL.
    #[serde(default, skip_serializing_if = "is_default", rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    #[serde(default, skip_serializing_if = "is_default", rename = "contact")]
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    #[serde(default, skip_serializing_if = "is_default", rename = "license")]
    pub license: Option<License>,
    ///**REQUIRED**. The version of the OpenAPI document (which is distinct from the [OpenAPI Specification version](OpenApi::openapi) or the API implementation version).
    #[serde(rename = "version")]
    pub version: String,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Contact information for the exposed API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    #[serde(default, skip_serializing_if = "is_default", rename = "name")]
    pub name: Option<String>,
    /// The URL pointing to the contact information. **MUST** be in the format of a URL.
    #[serde(default, skip_serializing_if = "is_default", rename = "url")]
    pub url: Option<Url>,
    /// The email address of the contact person/organization. **MUST** be in the format of an email address.
    #[serde(default, skip_serializing_if = "is_default", rename = "email")]
    pub email: Option<Email>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// License information for the exposed API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct License {
    ///**REQUIRED**. The license name used for the API.
    #[serde(rename = "name")]
    pub name: String,
    /// A URL to the license used for the API. **MUST** be in the format of a URL.
    #[serde(default, skip_serializing_if = "is_default", rename = "url")]
    pub url: Option<String>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// An object representing a Server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Server {
    ///**REQUIRED**. A URL to the target host.  This URL supports Server Variables and **MAY** be relative, to indicate that the host location is relative to the location where the OpenAPI document is being served. Variable substitutions will be made when a variable is named in `{`brackets`}`.
    #[serde(rename = "url")]
    pub url: String,
    /// An optional String describing the host designated by the URL. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    /// A map between a variable name and its value.  The value is used for substitution in the server’s URL template.
    #[serde(default, skip_serializing_if = "is_default", rename = "variables")]
    pub variables: Option<IndexMap<String, ServerVariable>>,
    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// An object representing a Server Variable for server URL template substitution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerVariable {
    /// An enumeration of String values to be used if the substitution options are from a limited set. The array **SHOULD NOT** be empty.
    #[serde(default, skip_serializing_if = "is_default", rename = "enum")]
    pub enum_: Option<Vec<String>>,
    ///**REQUIRED**. The default value to use for substitution, which **SHALL** be sent if an alternate value is *not* supplied. Note this behavior is different than the [Schema Object’s](Schema) treatment of default values, because in those cases parameter values are optional. If the [`enum`](Self::enum_) is defined, the value **SHOULD** exist in the enum’s values.
    #[serde(rename = "default")]
    pub default: String,
    /// An optional description for the server variable. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Holds a set of reusable objects for different aspects of the OpenAPI specification.
///
/// All objects defined within the components object will have no effect on the API unless they are explicitly referenced from properties outside the components object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Components {
    /// An object to hold reusable [Schema Objects](Schema).
    #[serde(default, skip_serializing_if = "is_default", rename = "schemas")]
    pub schemas: Option<IndexMap<ComponentKey, ReferenceOr<Schema>>>,
    /// An object to hold reusable [Response Objects](Response).
    #[serde(default, skip_serializing_if = "is_default", rename = "responses")]
    pub responses: Option<IndexMap<ComponentKey, ReferenceOr<Response>>>,
    /// An object to hold reusable [Parameter Objects](Parameter).
    #[serde(default, skip_serializing_if = "is_default", rename = "parameters")]
    pub parameters: Option<IndexMap<ComponentKey, ReferenceOr<Parameter>>>,
    /// An object to hold reusable [Example Objects](Example).
    #[serde(default, skip_serializing_if = "is_default", rename = "examples")]
    pub examples: Option<IndexMap<ComponentKey, ReferenceOr<Example>>>,
    /// An object to hold reusable [Request Body Objects](RequestBody).
    #[serde(default, skip_serializing_if = "is_default", rename = "requestBodies")]
    pub request_bodies: Option<IndexMap<ComponentKey, ReferenceOr<RequestBody>>>,
    /// An object to hold reusable [Header Objects](Header).
    #[serde(default, skip_serializing_if = "is_default", rename = "headers")]
    pub headers: Option<IndexMap<ComponentKey, ReferenceOr<Header>>>,
    /// An object to hold reusable [Security Scheme Objects](SecurityScheme).
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "securitySchemes"
    )]
    pub security_schemes: Option<IndexMap<ComponentKey, ReferenceOr<SecurityScheme>>>,
    /// An object to hold reusable [Link Objects](Link).
    #[serde(default, skip_serializing_if = "is_default", rename = "links")]
    pub links: Option<IndexMap<ComponentKey, ReferenceOr<Link>>>,
    /// An object to hold reusable [Callback Objects](Callback).
    #[serde(default, skip_serializing_if = "is_default", rename = "callbacks")]
    pub callbacks: Option<IndexMap<ComponentKey, ReferenceOr<Callback>>>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// All the fixed fields declared in [Components] are objects that **MUST** use keys that match the regular expression: `^[a-zA-Z0-9\.\-_]+$`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct ComponentKey(String);

impl From<ComponentKey> for String {
    fn from(value: ComponentKey) -> String {
        value.0
    }
}

impl TryFrom<String> for ComponentKey {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        const REGEX_RAW: &str = r"^[a-zA-Z0-9\.\-_]+$";
        static REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
            regex::Regex::new(REGEX_RAW).expect("Valid regex from the OpenAPI Spec S 4.7.7.1!")
        });

        if REGEX.is_match(&value) {
            Ok(Self(value))
        } else {
            Err(format!(
                "Invalid component key. Must match the pattern `{REGEX_RAW}`."
            ))
        }
    }
}

/// Holds the relative paths to the individual endpoints and their operations.
///
/// The path is appended to the URL from the [Server Object](Server) in order to construct the full URL.
/// The Paths **MAY** be empty, due to [ACL constraints](https://spec.openapis.org/oas/v3.0.3.html#security-filtering).
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Paths {
    // TODO: replace with path template...
    #[serde(default, flatten)]
    pub paths: IndexMap<String, PathItem>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

impl<'de> Deserialize<'de> for Paths {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut paths = IndexMap::default();
        let extensions = deserializer.deserialize_map(&mut MixedPatternVisitor::new(
            "Paths",
            |path: String, next_value| {
                if path.starts_with("/") {
                    paths
                        .insert(path.clone(), next_value()?)
                        .map_or(Ok(()), |_| Err(format!("Duplicate key `{path}`")))
                } else {
                    Err(format!(
                        "Unexpected field `{path}`. Paths must start with `/`."
                    ))
                }
            },
        ))?;

        Ok(Self { paths, extensions })
    }
}

/// Describes the operations available on a single path.
///
/// A Path Item **MAY** be empty, due to [ACL constraints](https://spec.openapis.org/oas/v3.0.3.html#security-filtering).
/// The path itself is still exposed to the documentation viewer but they will not know which operations and parameters are available.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PathItem {
    /// Allows for an external definition of this path item. The referenced structure **MUST** be in the format of a [Path Item Object](PathItem).  In case a Path Item Object field appears both in the defined object and the referenced object, the behavior is undefined.
    #[serde(default, skip_serializing_if = "is_default", rename = "$ref")]
    pub ref_: Option<String>,
    /// An optional, String summary, intended to apply to all operations in this path.
    #[serde(default, skip_serializing_if = "is_default", rename = "summary")]
    pub summary: Option<String>,
    /// An optional, String description, intended to apply to all operations in this path. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    /// A definition of a GET operation on this path.
    #[serde(default, skip_serializing_if = "is_default", rename = "get")]
    pub get: Option<Operation>,
    /// A definition of a PUT operation on this path.
    #[serde(default, skip_serializing_if = "is_default", rename = "put")]
    pub put: Option<Operation>,
    /// A definition of a POST operation on this path.
    #[serde(default, skip_serializing_if = "is_default", rename = "post")]
    pub post: Option<Operation>,
    /// A definition of a DELETE operation on this path.
    #[serde(default, skip_serializing_if = "is_default", rename = "delete")]
    pub delete: Option<Operation>,
    /// A definition of a OPTIONS operation on this path.
    #[serde(default, skip_serializing_if = "is_default", rename = "options")]
    pub options: Option<Operation>,
    /// A definition of a HEAD operation on this path.
    #[serde(default, skip_serializing_if = "is_default", rename = "head")]
    pub head: Option<Operation>,
    /// A definition of a PATCH operation on this path.
    #[serde(default, skip_serializing_if = "is_default", rename = "patch")]
    pub patch: Option<Operation>,
    /// A definition of a TRACE operation on this path.
    #[serde(default, skip_serializing_if = "is_default", rename = "trace")]
    pub trace: Option<Operation>,
    /// An alternative [server](Server) array to service all operations in this path.
    #[serde(default, skip_serializing_if = "is_default", rename = "servers")]
    pub servers: Option<Vec<Server>>,
    /// A list of parameters that are applicable for all the operations described under this path. These parameters can be overridden at the operation level, but cannot be removed there. The list **MUST NOT** include duplicated parameters. A unique parameter is defined by a combination of a [name](Parameter::name) and [location](Parameter::in_). The list can use the [Reference Object](Reference) to link to parameters that are defined at the [OpenAPI Object’s components/parameters](Components::parameters).
    #[serde(default, skip_serializing_if = "is_default", rename = "parameters")]
    pub parameters: Option<Vec<ReferenceOr<Parameter>>>,
    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Describes a single API operation on a path.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Operation {
    /// A list of tags for API documentation control. Tags can be used for logical grouping of operations by resources or any other qualifier.
    #[serde(default, skip_serializing_if = "is_default", rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// A short summary of what the operation does.
    #[serde(default, skip_serializing_if = "is_default", rename = "summary")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation behavior. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    /// Additional external documentation for this operation.
    #[serde(default, skip_serializing_if = "is_default", rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Unique String used to identify the operation. The id **MUST** be unique among all operations described in the API. The operationId value is *case-sensitive*. Tools and libraries **MAY** use the operationId to uniquely identify an operation, therefore, it is **RECOMMENDED** to follow common programming naming conventions.
    #[serde(default, skip_serializing_if = "is_default", rename = "operationId")]
    pub operation_id: Option<String>,
    /// A list of parameters that are applicable for this operation. If a parameter is already defined at the [Path Item](PathItem::parameters), the new definition will override it but can never remove it. The list **MUST NOT** include duplicated parameters. A unique parameter is defined by a combination of a [name](Parameter::name) and [location](Parameter::in_). The list can use the [Reference Object](Reference) to link to parameters that are defined at the [OpenAPI Object’s components/parameters](Components::parameters).
    #[serde(default, skip_serializing_if = "is_default", rename = "parameters")]
    pub parameters: Option<Vec<ReferenceOr<Parameter>>>,
    /// The request body applicable for this operation.  The [requestBody](Self::request_body) is only supported in HTTP methods where the HTTP 1.1 specification [[RFC7231](https://httpwg.org/specs/rfc7231.html)] [Section 4.3.1](https://datatracker.ietf.org/doc/html/rfc7231#section-4.3.1) has explicitly defined semantics for request bodies.  In other cases where the HTTP spec is vague, [requestBody](Self::request_body) **SHALL** be ignored by consumers.
    #[serde(default, skip_serializing_if = "is_default", rename = "requestBody")]
    pub request_body: Option<ReferenceOr<RequestBody>>,
    ///**REQUIRED**. The list of possible responses as they are returned from executing this operation.
    #[serde(rename = "responses")]
    pub responses: Responses,
    /// A map of possible out-of band callbacks related to the parent operation. The key is a unique identifier for the Callback Object. Each value in the map is a [Callback Object](Callback) that describes a request that may be initiated by the API provider and the expected responses.
    #[serde(default, skip_serializing_if = "is_default", rename = "callbacks")]
    pub callbacks: Option<IndexMap<String, ReferenceOr<Callback>>>,
    /// Declares this operation to be deprecated. Consumers **SHOULD** refrain from usage of the declared operation. Default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "deprecated")]
    pub deprecated: bool,
    /// A declaration of which security mechanisms can be used for this operation. The list of values includes alternative security requirement objects that can be used.
    ///
    /// Only one of the security requirement objects need to be satisfied to authorize a request.
    /// To make security optional, an empty security requirement (`{}`) can be included in the array.
    /// This definition overrides any declared top-level [`security`](OpenApi::security). To remove a top-level security declaration, an empty array can be used.
    #[serde(default, skip_serializing_if = "is_default", rename = "security")]
    pub security: Option<Vec<SecurityRequirement>>,
    /// An alternative [Server] array to service this operation. If an alternative server object is specified at the [Path Item Object](PathItem::servers) or [Root level](OpenApi::servers), it will be overridden by this value.
    #[serde(default, skip_serializing_if = "is_default", rename = "servers")]
    pub servers: Option<Vec<Server>>,
    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Allows referencing an external resource for extended documentation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExternalDocumentation {
    /// A short description of the target documentation. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    ///**REQUIRED**. The URL for the target documentation. Value **MUST** be in the format of a URL.
    #[serde(rename = "url")]
    pub url: Url,
    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Describes a single operation parameter.
///
/// A unique parameter is defined by a combination of a [name](Self::name) and [location](Self::in_).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    ///**REQUIRED**. The name of the parameter. Parameter names are *case sensitive*.
    /// - If [in](Parameter::in_) is ["path"](ParameterLocation::Path), the [name](Parameter::name) field **MUST** correspond to a template expression occurring within the [path](Paths::paths) field in the [Paths Object](Paths). See [Path Templating](https://spec.openapis.org/oas/v3.0.3.html#path-templating) for further information.
    /// - If [in](Parameter::in_) is ["header"](ParameterLocation::Header) and the [name](Parameter::name) field is `"Accept"` or `"Content-Type"` or `"Authorization"`, the parameter definition **SHALL** be ignored.
    /// - For all other cases, the [name](Parameter::name) field corresponds to the parameter name used by the [`in`](Parameter::in_) property.
    #[serde(rename = "name")]
    pub name: String,
    ///**REQUIRED**. The location of the parameter.
    #[serde(rename = "in")]
    pub in_: ParameterLocation,
    /// A brief description of the parameter. This could contain examples of use. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory. If the [parameter location](Parameter::in_) is ["path"](ParameterLocation::Path), this property is**REQUIRED** and its value **MUST** be `true`.
    /// - Otherwise, the property **MAY** be included and its default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "required")]
    pub required: Option<bool>,
    /// Specifies that a parameter is deprecated and **SHOULD** be transitioned out of usage. Default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "deprecated")]
    pub deprecated: bool,
    /// Sets the ability to pass empty-valued parameters. This is valid only for [query](ParameterLocation::Query) parameters and allows sending a parameter with an empty value. Default value is `false`.
    ///
    /// If [`style`](Self::style) is used, and if behavior is `n/a` (cannot be serialized), the value of [`allowEmptyValue`](Self::allow_empty_value) **SHALL** be ignored.
    ///
    /// Use of this property is **NOT RECOMMENDED**, as it is likely to be removed in a later revision.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "allowEmptyValue"
    )]
    pub allow_empty_value: bool,
    /// Describes how the parameter value will be serialized depending on the type of the parameter value. Default values (based on value of [in](Self::in_)):
    /// - for [`query`](ParameterLocation::Query) - [`form`](ParameterStyle::Form);
    /// - for [`path`](ParameterLocation::Query) - [`simple`](ParameterStyle::Form);
    /// - for [`header`](ParameterLocation::Query) - [`simple`](ParameterStyle::Form);
    /// - for [`cookie`](ParameterLocation::Query) - [`form`](ParameterStyle::Form).
    #[serde(default, skip_serializing_if = "is_default", rename = "style")]
    pub style: Option<ParameterStyle>,
    pub explode: Option<bool>,
    /// Determines whether the parameter value **SHOULD** allow reserved characters, as defined by [[RFC3886](https://www.rfc-editor.org/info/rfc3986/)] [Section 2.2](https://datatracker.ietf.org/doc/html/rfc3986#section-2.2) `:/?#[]@!$&amp;'()*+,;=` to be included without percent-encoding. This property only applies to parameters with an `in` value of `query`. The default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "allowReserved")]
    pub allow_reserved: bool,
    /// The schema defining the type used for the parameter.
    #[serde(default, skip_serializing_if = "is_default", rename = "schema")]
    pub schema: Option<ReferenceOr<Schema>>,
    /// Example of the parameter’s potential value.
    ///
    /// The example **SHOULD** match the specified schema and encoding properties if present. The [`example`](Self::example) field is mutually exclusive of the [`examples`](Self::examples) field. Furthermore, if referencing a schema that contains an example, the example value **SHALL** override the example provided by the schema. To represent examples of media types that cannot naturally be represented in JSON or YAML, a string value can contain the example with escaping where necessary.
    #[serde(default, skip_serializing_if = "is_default", rename = "example")]
    pub example: Option<serde_json::Value>,
    /// Examples of the parameter’s potential value.
    ///
    /// Each example **SHOULD** contain a value in the correct format as specified in the parameter encoding.
    ///
    /// The [`examples`](Self::examples) field is mutually exclusive of the [`example`](Self::example) field.
    /// Furthermore, if referencing a [`schema`](Self::schema) that contains an example, the [`examples`](Self::examples) value **SHALL** *override* the example provided by the schema.
    #[serde(default, skip_serializing_if = "is_default", rename = "examples")]
    pub examples: Option<IndexMap<String, ReferenceOr<Example>>>,
    /// A map containing the representations for the parameter. The key is the media type and the value describes it. The map **MUST** only contain one entry.
    #[serde(default, skip_serializing_if = "is_default", rename = "content")]
    pub content: Option<IndexMap<String, MediaType>>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// There are four possible parameter locations specified by the [`in`](Parameter::in_) field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParameterLocation {
    /// Used together with [Path Templating](https://spec.openapis.org/oas/v3.0.3.html#path-templating),
    /// where the parameter value is actually part of the operation’s URL.
    ///
    /// This does not include the host or base path of the API. For example, in `/items/{itemId}`, the path parameter is `itemId`.
    #[serde(rename = "path")]
    Path,
    /// Parameters that are appended to the URL. For example, in `/items?id=###`, the query parameter is `id`.
    #[serde(rename = "query")]
    Query,
    /// Custom headers that are expected as part of the request. Note that [[RFC7230](https://datatracker.ietf.org/doc/html/rfc7230)] [Page 22](https://datatracker.ietf.org/doc/html/rfc7230#page-22) states header names are case insensitive.
    #[serde(rename = "header")]
    Header,
    /// Used to pass a specific cookie value to the API.
    #[serde(rename = "cookie")]
    Cookie,
}

/// In order to support common ways of serializing simple parameters, a set of [`style`](Parameter::style) values are defined.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParameterStyle {
    /// Path-style parameters defined by [[RFC6570](https://www.rfc-editor.org/info/rfc6570/)] [Section 3.2.7](https://www.rfc-editor.org/info/rfc6570/#section-3.2.7)
    #[serde(rename = "matrix")]
    Matrix,
    /// Label style parameters defined by [[RFC6570](https://www.rfc-editor.org/info/rfc6570/)] [Section 3.2.5](https://www.rfc-editor.org/info/rfc6570/#section-3.2.5)
    #[serde(rename = "label")]
    Label,
    /// Form style parameters defined by [[RFC6570](https://www.rfc-editor.org/info/rfc6570/)] [Section 3.2.8](https://www.rfc-editor.org/info/rfc6570/#section-3.2.8).
    ///
    /// This option replaces `collectionFormat` with a `csv` (when explode is `false`) or `multi` (when explode is `true`) value from OpenAPI 2.0.
    #[serde(rename = "form")]
    Form,
    /// Simple style parameters defined by [[RFC6570](https://www.rfc-editor.org/info/rfc6570/)] [Section 3.2.2](https://www.rfc-editor.org/info/rfc6570/#section-3.2.2).
    ///
    /// This option replaces `collectionFormat` with a `csv` value from OpenAPI 2.0.
    #[serde(rename = "simple")]
    Simple,
    /// Space separated array values. This option replaces `collectionFormat` equal to `ssv` from OpenAPI 2.0.
    #[serde(rename = "spaceDelimited")]
    SpaceDelimited,
    /// Pipe separated array values. This option replaces `collectionFormat` equal to `pipes` from OpenAPI 2.0.
    #[serde(rename = "pipeDelimited")]
    PipeDelimited,
    /// Provides a simple way of rendering nested objects using form parameters.
    #[serde(rename = "deepObject")]
    DeepObject,
}

/// Describes a single request body.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestBody {
    /// A brief description of the request body. This could contain examples of use.  [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    ///**REQUIRED**. The content of the request body. The key is a media type or media type range, see [[RFC7231](https://httpwg.org/specs/rfc7231.html)] [Appendix D](https://datatracker.ietf.org/doc/html/rfc7231#appendix-D), and the value describes it.  For requests that match multiple keys, only the most specific key is applicable. e.g. text/plain overrides text/*
    #[serde(rename = "content")]
    pub content: IndexMap<String, MediaType>,
    /// Determines if the request body is required in the request. Defaults to `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "required")]
    pub required: bool,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Each Media Type Object provides schema and examples for the media type identified by its key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MediaType {
    /// The schema defining the content of the request, response, or parameter.
    #[serde(default, skip_serializing_if = "is_default", rename = "schema")]
    pub schema: Option<ReferenceOr<Schema>>,
    /// Example of the media type.  The example object **SHOULD** be in the correct format as specified by the media type.  The `example` field is mutually exclusive of the `examples` field.  Furthermore, if referencing a `schema` which contains an example, the `example` value **SHALL** *override* the example provided by the schema.
    #[serde(default, skip_serializing_if = "is_default", rename = "example")]
    pub example: Option<serde_json::Value>,
    /// Examples of the media type.  Each example object **SHOULD**  match the media type and specified schema if present.  The `examples` field is mutually exclusive of the `example` field.  Furthermore, if referencing a `schema` which contains an example, the `examples` value **SHALL** *override* the example provided by the schema.
    #[serde(default, skip_serializing_if = "is_default", rename = "examples")]
    pub examples: Option<IndexMap<String, ReferenceOr<Example>>>,
    /// A map between a property name and its encoding information. The key, being the property name, **MUST** exist in the schema as a property. The encoding object **SHALL** only apply to `requestBody` objects when the media type is `multipart` or `application/x-www-form-urlencoded`.
    #[serde(default, skip_serializing_if = "is_default", rename = "encoding")]
    pub encoding: Option<IndexMap<String, Encoding>>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// A single encoding definition applied to a single schema property.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Encoding {
    #[serde(default, skip_serializing_if = "is_default", rename = "contentType")]
    pub content_type: Option<String>,
    /// A map allowing additional information to be provided as headers, for example `Content-Disposition`.  `Content-Type` is described separately and **SHALL** be ignored in this section. This property **SHALL** be ignored if the request body media type is not a `multipart`.
    #[serde(default, skip_serializing_if = "is_default", rename = "headers")]
    pub headers: Option<IndexMap<String, ReferenceOr<Header>>>,
    /// Describes how a specific property value will be serialized depending on its type.  See [Parameter Object](Parameter) for details on the [`style`](Parameter::style) property. The behavior follows the same values as [`query`](ParameterLocation::Query) parameters, including default values. This property **SHALL** be ignored if the request body media type is not `application/x-www-form-urlencoded`.
    #[serde(default, skip_serializing_if = "is_default", rename = "style")]
    pub style: Option<ParameterStyle>,
    /// When this is true, property values of type `array` or `object` generate separate parameters for each value of the array, or key-value-pair of the map. For other types of properties this property has no effect.
    ///
    /// When [`style`](Self::style) is [`form`](ParameterStyle::Form), the default value is `true`. For all other styles, the default value is `false`.
    ///
    /// This property **SHALL** be ignored if the request body media type is not `application/x-www-form-urlencoded`.
    pub explode: Option<bool>,
    /// Determines whether the parameter value **SHOULD** allow reserved characters, as defined by [[RFC3886](https://www.rfc-editor.org/info/rfc3986/)] [Section 2.2](https://datatracker.ietf.org/doc/html/rfc3986#section-2.2) `:/?#[]@!$&'()*+,;=` to be included without percent-encoding. The default value is `false`. This property **SHALL** be ignored if the request body media type is not `application/x-www-form-urlencoded`.
    #[serde(default, skip_serializing_if = "is_default", rename = "allowReserved")]
    pub allow_reserved: bool,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// A container for the expected responses of an operation. The container maps a HTTP response code to the expected response.
///
/// The documentation is not necessarily expected to cover all possible HTTP response codes because they may not be known in advance. However, documentation is expected to cover a successful operation response and any known errors.
///
/// The `default` field **MAY** be used as a default response object for all HTTP codes that are not covered individually by the specification.
///
/// The [Responses Object](Response) **MUST** contain at least one response code, and it **SHOULD** be the response for a successful operation call.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Responses {
    /// The documentation of responses other than the ones declared for specific HTTP response codes. Use this field to cover undeclared responses.
    /// A [Reference Object](Reference) can link to a response that the [OpenAPI Object’s components/responses](Components::responses) section defines.
    #[serde(default, skip_serializing_if = "is_default", rename = "default")]
    pub default: Option<ReferenceOr<Response>>,

    /// Any HTTP status code can be used as the property name, but only one property per code, to describe the expected response for that HTTP status code.
    ///
    /// A [Reference Object](Reference<Response>) can link to a response that is defined in the [OpenAPI Object’s components/responses](Components::responses) section.
    /// This field **MUST** be enclosed in quotation marks (for example, `"200"``) for compatibility between JSON and YAML.
    ///
    /// To define a range of response codes, this field **MAY** contain the uppercase wildcard character `X`. For example, `2XX` represents all response codes between `[200-299]`. Only the following range definitions are allowed: `1XX`, `2XX`, `3XX`, `4XX`, and `5XX`.
    /// If a response is defined using an explicit code, the explicit code definition takes precedence over the range definition for that code.
    #[serde(default, flatten)]
    pub responses: IndexMap<HttpStatusCode, ReferenceOr<Response>>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

impl<'de> Deserialize<'de> for Responses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut default = None;
        let mut responses = IndexMap::new();

        let extensions = deserializer.deserialize_map(&mut MixedPatternVisitor::new(
            "Responses",
            |key: String, next_value| match key {
                key if key == "default" => default
                    .replace(next_value()?)
                    .map_or(Ok(()), |_| Err("duplicate field default".to_string())),

                key => responses
                    .insert(HttpStatusCode::try_from(key.clone())?, next_value()?)
                    .map_or(Ok(()), |_| Err(format!("duplicate responses key: {key}"))),
            },
        ))?;

        Ok(Responses {
            default,
            responses,
            extensions,
        })
    }
}

/// Describes a single response from an API Operation, including design-time, static `links` to operations based on the response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    ///**REQUIRED**. A short description of the response. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(rename = "description")]
    pub description: String,
    /// Maps a header name to its definition. [[RFC7230](https://httpwg.org/specs/rfc7230.html)] [Page 22](https://datatracker.ietf.org/doc/html/rfc7230#page-22) states header names are case insensitive. If a response header is defined with the name `"Content-Type"`, it **SHALL** be ignored.
    #[serde(default, skip_serializing_if = "is_default", rename = "headers")]
    pub headers: Option<IndexMap<String, ReferenceOr<Header>>>,
    /// A map containing descriptions of potential response payloads. The key is a media type or media type range, see [[RFC7231](https://httpwg.org/specs/rfc7231.html)] [Appendix D](https://datatracker.ietf.org/doc/html/rfc7231#appendix-D), and the value describes it.  For responses that match multiple keys, only the most specific key is applicable. e.g. text/plain overrides text/*
    #[serde(default, skip_serializing_if = "is_default", rename = "content")]
    pub content: Option<IndexMap<String, MediaType>>,
    /// A map of operations links that can be followed from the response. The key of the map is a short name for the link, following the naming constraints of the names for [Component Objects](Components).
    #[serde(default, skip_serializing_if = "is_default", rename = "links")]
    pub links: Option<IndexMap<String, ReferenceOr<Link>>>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// A map of possible out-of band callbacks related to the parent operation.
///
/// Each value in the map is a [Path Item Object](PathItem) that describes a set of requests that may be initiated by the API provider
/// and the expected responses. The key value used to identify the path item object is an [expression](Expression), evaluated at runtime,
/// that identifies a URL to use for the callback operation.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Callback {
    /// A [Path Item Object](PathItem) used to define a callback request and expected responses. A [complete example](https://learn.openapis.org/examples/v3.0/callback-example.html) is available.
    #[serde(default, flatten)]
    pub callbacks: IndexMap<Expression, PathItem>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

impl<'de> serde::Deserialize<'de> for Callback {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut callbacks = IndexMap::new();
        let extensions = deserializer.deserialize_map(&mut MixedPatternVisitor::new(
            "Callback",
            |key: Expression, next_value| {
                callbacks
                    .insert(key.clone(), next_value()?)
                    .map_or(Ok(()), |_| {
                        Err(format!("duplicate field `{}`", key.deref()))
                    })
            },
        ))?;

        Ok(Self {
            callbacks,
            extensions,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Example {
    /// Short description for the example.
    #[serde(default, skip_serializing_if = "is_default", rename = "summary")]
    pub summary: Option<String>,
    /// Long description for the example. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    pub value: Option<serde_json::Value>,
    /// A URL that points to the literal example. This provides the capability to reference examples that cannot easily be included in JSON or YAML documents.  The `value` field and `externalValue` field are mutually exclusive.
    #[serde(default, skip_serializing_if = "is_default", rename = "externalValue")]
    pub external_value: Option<String>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// The Link object represents a possible design-time link for a response.
///
/// The presence of a link does not guarantee the caller’s ability to
/// successfully invoke it, rather it provides a known relationship and traversal mechanism between responses and other operations.
///
/// Unlike dynamic links (i.e. links provided in the response payload), the OAS linking mechanism does not require link information in the
/// runtime response.
///
/// For computing links, and providing instructions to execute them, a [runtime expression](Expression) is used for accessing values in an operation
/// and using them as parameters while invoking the linked operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Link {
    /// A relative or absolute URI reference to an OAS operation. This field is mutually exclusive of the [`operationId`](Link::operation_id) field, and **MUST** point to an [Operation Object](Operation). Relative `operationRef` values **MAY** be used to locate an existing [Operation Object](Operation) in the OpenAPI definition.
    #[serde(default, skip_serializing_if = "is_default", rename = "operationRef")]
    pub operation_ref: Option<String>,
    /// The name of an *existing*, resolvable OAS operation, as defined with a unique `operationId`.  This field is mutually exclusive of the [`operationRef`](Link::operation_ref) field.
    #[serde(default, skip_serializing_if = "is_default", rename = "operationId")]
    pub operation_id: Option<String>,
    /// A map representing parameters to pass to an operation as specified with [`operationId`](Link::operation_id) or identified via [`operationRef`](Link::operation_ref). The key is the parameter name to be used, whereas the value can be a constant or an expression to be evaluated and passed to the linked operation.  The parameter name can be qualified using the [parameter location](Parameter::in_) `[{in}.]{name}` for operations that use the same parameter name in different locations (e.g. path.id).
    #[serde(default, skip_serializing_if = "is_default", rename = "parameters")]
    pub parameters: Option<IndexMap<String, Either<serde_json::Value, Expression>>>,
    /// A literal value or [{expression}](Expression) to use as a request body when calling the target operation.
    #[serde(default, skip_serializing_if = "is_default", rename = "requestBody")]
    pub request_body: Option<Either<serde_json::Value, Expression>>,
    /// A description of the link. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    /// A server object to be used by the target operation.
    #[serde(default, skip_serializing_if = "is_default", rename = "server")]
    pub server: Option<Server>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Runtime expressions allow defining values based on information that will only be available within the HTTP message in an
/// actual API call.
///
/// This mechanism is used by [Link Objects](Link) and [Callback Objects](Callback).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Expression(String);

impl std::ops::Deref for Expression {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Header {
    /// A brief description of the parameter. This could contain examples of use. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    /// Determines whether this parameter is mandatory. Default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "required")]
    pub required: bool,
    /// Specifies that a parameter is deprecated and **SHOULD** be transitioned out of usage. Default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "deprecated")]
    pub deprecated: bool,
    /// Sets the ability to pass empty-valued parameters. This is valid only for `query` parameters and allows sending a parameter with an empty value. Default value is `false`. If [`style`](Parameter::style) is used, and if behavior is `n/a` (cannot be serialized), the value of `allowEmptyValue` **SHALL** be ignored. Use of this property is **NOT RECOMMENDED**, as it is likely to be removed in a later revision.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "allowEmptyValue"
    )]
    pub allow_empty_value: bool,
    /// Describes how the parameter value will be serialized depending on the type of the parameter value. Default value: [`simple`](ParameterStyle::Simple).
    #[serde(default, skip_serializing_if = "is_default", rename = "style")]
    pub style: Option<ParameterStyle>,
    /// When this is true, parameter values of type array or object generate separate parameters for each value of the array or key-value pair of the map.
    ///
    /// For other types of parameters this property has no effect. The default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "explode")]
    pub explode: bool,
    /// Determines whether the parameter value **SHOULD** allow reserved characters, as defined by [[RFC3886](https://www.rfc-editor.org/info/rfc3986/)] [Section 2.2](https://datatracker.ietf.org/doc/html/rfc3986#section-2.2) `:/?#[]@!$&amp;'()*+,;=` to be included without percent-encoding. This property only applies to parameters with an `in` value of `query`. The default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "allowReserved")]
    pub allow_reserved: bool,
    /// The schema defining the type used for the parameter.
    #[serde(default, skip_serializing_if = "is_default", rename = "schema")]
    pub schema: Option<ReferenceOr<Schema>>,
    /// Example of the parameter’s potential value.
    ///
    /// The example **SHOULD** match the specified schema and encoding properties if present. The [`example`](Self::example) field is mutually exclusive of the [`examples`](Self::examples) field. Furthermore, if referencing a schema that contains an example, the example value **SHALL** override the example provided by the schema. To represent examples of media types that cannot naturally be represented in JSON or YAML, a string value can contain the example with escaping where necessary.
    #[serde(default, skip_serializing_if = "is_default", rename = "example")]
    pub example: Option<serde_json::Value>,
    /// Examples of the parameter’s potential value. Each example **SHOULD** contain a value in the correct format as specified in the parameter encoding. The `examples` field is mutually exclusive of the [`example`](Self::example) field. Furthermore, if referencing a `schema` that contains an example, the `examples` value **SHALL** *override* the example provided by the schema.
    #[serde(default, skip_serializing_if = "is_default", rename = "examples")]
    pub examples: Option<IndexMap<String, ReferenceOr<Example>>>,
    /// A map containing the representations for the parameter. The key is the media type and the value describes it. The map **MUST** only contain one entry.
    #[serde(default, skip_serializing_if = "is_default", rename = "content")]
    pub content: Option<IndexMap<String, MediaType>>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Adds metadata to a single tag that is used by the [Operation Object](Operation).
///
/// It is not mandatory to have a [Tag Object](Tag) per tag defined in the Operation Object instances.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    ///**REQUIRED**. The name of the tag.
    #[serde(rename = "name")]
    pub name: String,
    /// A short description for the tag. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(default, skip_serializing_if = "is_default", rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// A simple object to allow referencing other components in the specification, internally and externally.
///
/// The Reference Object is defined by [JSON Reference](https://datatracker.ietf.org/doc/html/draft-pbryan-zyp-json-ref-03) and follows the same structure, behavior and rules.
///
/// For this specification, reference resolution is accomplished as defined by the JSON Reference specification and not by the JSON Schema specification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reference<T, S = String> {
    ///**REQUIRED**. The reference String.
    #[serde(rename = "$ref")]
    pub ref_: S,

    #[serde(skip)]
    _type: PhantomData<T>,
}

impl<T, S> Reference<T, S>
where
    S: Deref<Target = str>,
{
    const FRAGMENT: &str = "#";

    pub fn new(ref_: S) -> Self {
        Self {
            ref_,
            _type: PhantomData,
        }
    }

    pub fn as_str(&self) -> &str {
        self.ref_.as_ref()
    }

    /// Get the path to the file referenced.
    pub fn path(&self) -> &str {
        self.as_str()
            .split_once(Self::FRAGMENT)
            .map(|(path, _)| path)
            .unwrap_or(self.as_str())
    }

    /// Get the JSON Pointer (see [[RFC 6901](https://www.rfc-editor.org/info/rfc6901/)]), if present, in this reference.
    ///
    /// Use this in combination with [json_pointer::follow](crate::ext::json_pointer::follow) to evaluate the pointer on
    /// the referenced object at [Self::path].
    pub fn pointer(&self) -> Option<&str> {
        self.as_str().split_once(Self::FRAGMENT).map(|(_, ptr)| ptr)
    }
}

/// The Schema Object allows the definition of input and output data types.
///
/// These types can be objects, but also primitives and arrays. This object is an extended subset of the [JSON Schema Specification Wright Draft 00](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-00).
///
/// For more information about the properties, see [JSON Schema Core](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-00) and [JSON Schema Validation](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00).
/// Unless stated otherwise, the property definitions follow the JSON Schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Schema {
    /// A title will preferably be short.
    ///
    /// Can be used to decorate a user interface with information about the data produced by this user interface.
    ///
    /// [JSON Schema Validation, Section 6.1](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-6.1)
    #[serde(default, skip_serializing_if = "is_default", rename = "title")]
    pub title: Option<String>,
    /// Provides explanation about the purpose of the instance described by this schema.
    ///
    /// Can be used to decorate a user interface with information about the data produced by this user interface.
    ///
    ///  [JSON Schema Validation, Section 6.1](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-6.1)
    #[serde(default, skip_serializing_if = "is_default", rename = "description")]
    pub description: Option<String>,
    /// A numeric instance is valid only if division by this keyword's value results in an integer.
    ///
    /// The value of "multipleOf" MUST be a number, strictly greater than 0.
    ///
    /// [JSON Schema Validation, Section 5.1](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.1)
    #[serde(default, skip_serializing_if = "is_default", rename = "multipleOf")]
    pub multiple_of: Option<f64>,
    /// Represents an inclusive upper limit for a numeric instance.
    ///
    /// [JSON Schema Validation, Section 5.2](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.2)
    #[serde(default, skip_serializing_if = "is_default", rename = "maximum")]
    pub maximum: Option<f64>,
    /// Represents an exclusive upper limit for a numeric instance.
    ///
    /// [JSON Schema Validation, Section 5.3](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.3)
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "exclusiveMaximum"
    )]
    pub exclusive_maximum: bool,
    /// Represents an inclusive lower limit for a numeric instance.
    ///
    /// [JSON Schema Validation, Section 5.4](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.4)
    #[serde(default, skip_serializing_if = "is_default", rename = "minimum")]
    pub minimum: Option<f64>,
    /// Represents an exclusive lower limit for a numeric instance.
    ///
    /// [JSON Schema Validation, Section 5.5](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.5)
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "exclusiveMinimum"
    )]
    pub exclusive_minimum: bool,
    /// A string instance is valid against this keyword if its length is less than, or equal to, the value of this keyword.
    ///
    /// [JSON Schema Validation, Section 5.6](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.6)
    #[serde(default, skip_serializing_if = "is_default", rename = "maxLength")]
    pub max_length: Option<usize>,
    /// A string instance is valid against this keyword if its length is greater than, or equal to, the value of this keyword.
    ///
    /// Omitting this keyword has the same behavior as a value of 0.
    ///
    /// [JSON Schema Validation, Section 5.7](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.7)
    #[serde(default, skip_serializing_if = "is_default", rename = "minLength")]
    pub min_length: usize,
    /// This string SHOULD be a valid regular expression, according to the ECMA 262 regular expression dialect.
    ///
    /// A string instance is considered valid if the regular expression matches the instance successfully.
    ///
    /// [JSON Schema Validation, Section 5.8](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.8)
    #[serde(default, skip_serializing_if = "is_default", rename = "pattern")]
    pub pattern: Option<String>,

    /// An array instance is valid against ["max_items"](Self::max_items) if its size is less than, or equal to, the value of this keyword.
    ///
    /// [JSON Schema Validation, Section 5.10](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.10)
    #[serde(default, skip_serializing_if = "is_default", rename = "maxItems")]
    pub max_items: Option<usize>,

    /// An array instance is valid against ["minItems"](Self::min_items) if its size is greater than, or equal to, the value of this keyword.
    ///
    /// Omitting this keyword has the same behavior as a value of 0.
    ///
    /// [JSON Schema Validation, Section 5.11](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.11)
    #[serde(default, skip_serializing_if = "is_default", rename = "minItems")]
    pub min_items: usize,

    /// If this keyword has boolean value `true`, the instance validates successfully if all of its elements are unique.
    ///
    /// If it has boolean value `false`, the instance validates successfully.
    ///
    /// Omitting this keyword has the same behavior as a value of `false`.
    ///
    /// [JSON Schema Validation, Section 5.12](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.12)
    #[serde(default, skip_serializing_if = "is_default", rename = "uniqueItems")]
    pub unique_items: bool,

    /// An object instance is valid against ["maxProperties"](Self::max_properties) if its number of properties is less than, or equal to, the value of this keyword.
    ///
    /// [JSON Schema Validation, Section 5.13](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.13)
    #[serde(default, skip_serializing_if = "is_default", rename = "maxProperties")]
    pub max_properties: Option<usize>,

    /// An object instance is valid against ["minProperties"](Self::min_properties) if its number of properties is greater than, or equal to, the value of this keyword.
    ///
    /// Omitting this keyword has the same behavior as a value of 0.
    ///
    /// [JSON Schema Validation, Section 5.14](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.14)
    #[serde(default, skip_serializing_if = "is_default", rename = "minProperties")]
    pub min_properties: usize,

    /// An object instance is valid against this keyword if every item in the array is the name of a property in the instance.
    ///
    /// Omitting this keyword has the same behavior as an empty array.
    ///
    /// [JSON Schema Validation, Section 5.15](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.15)
    #[serde(default, skip_serializing_if = "is_default", rename = "required")]
    pub required: IndexSet<String>,

    /// An instance validates successfully against this keyword if its value is equal to one of the elements in this keyword's array value.
    ///
    /// Elements in the array might be of any value, including null.
    ///
    /// [JSON Schema Validation, Section 5.20](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.20)
    #[serde(default, skip_serializing_if = "is_default", rename = "enum")]
    pub enum_: IndexSet<serde_json::Value>,

    /// An instance validates if and only if the instance is in any of the sets listed for this keyword ([Type]).
    ///
    /// [JSON Schema Validation, Section 5.21](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.21)
    #[serde(default, skip_serializing_if = "is_default", rename = "type")]
    pub type_: Option<Type>,

    /// An instance validates successfully against this keyword if it validates successfully against all schemas defined by this keyword's value.
    ///
    /// This keyword's value MUST be a non-empty array.
    ///
    /// [JSON Schema Validation, Section 5.22](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.22)
    #[serde(default, skip_serializing_if = "is_default", rename = "allOf")]
    pub all_of: Option<Vec<ReferenceOr<Self>>>,

    /// An instance validates successfully against this keyword if it validates successfully against at least one schema defined by this keyword's value.
    ///
    /// This keyword's value MUST be a non-empty array.
    ///
    /// [JSON Schema Validation, Section 5.24](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.24)
    #[serde(default, skip_serializing_if = "is_default", rename = "oneOf")]
    pub one_of: Option<Vec<ReferenceOr<Self>>>,

    /// An instance validates successfully against this keyword if it validates successfully against exactly one schema defined by this keyword's value.
    ///
    /// This keyword's value MUST be a non-empty array.
    ///
    /// [JSON Schema Validation, Section 5.23](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.23)
    #[serde(default, skip_serializing_if = "is_default", rename = "anyOf")]
    pub any_of: Option<Vec<ReferenceOr<Self>>>,

    /// An instance is valid against this keyword if it fails to validate successfully against the schema defined by this keyword.
    ///
    /// [JSON Schema Validation, Section 5.25](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.25)
    #[serde(default, skip_serializing_if = "is_default", rename = "not")]
    pub not: Option<ReferenceOr<Box<Self>>>,

    /// Validation succeeds if all elements in the array successfully validate against that schema.
    ///
    /// Omitting this keyword has the same behavior as an empty schema.
    ///
    /// [JSON Schema Validation, Section 5.9](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.9)
    #[serde(default, skip_serializing_if = "is_default", rename = "items")]
    pub items: Option<ReferenceOr<Box<Self>>>,

    /// Validation succeeds if, for each name that appears in both the instance and as a name within this keyword's value, the child instance for that name successfully validates against the corresponding schema.
    ///
    /// Omitting this keyword has the same behavior as an empty object.
    ///
    /// [JSON Schema Validation, Section 5.16](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.16)
    #[serde(default, skip_serializing_if = "is_default", rename = "properties")]
    pub properties: Option<IndexMap<String, ReferenceOr<Self>>>,

    /// This keyword determines how child instances validate for objects, and does not directly validate the immediate instance itself.
    ///
    /// Validation with ["additionalProperties"](Self::additional_properties) applies only to the child values of instance names that do not match any names in ["properties"](Self::properties).
    ///
    /// For all such properties, validation succeeds if the child instance validates against the  ["additionalProperties"](Self::additional_properties) schema.
    ///
    /// Omitting this keyword has the same behavior as having ["additionalProperties"](Self::additional_properties) set to true.
    ///
    /// [JSON Schema Validation, Section 5.18](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.18)
    #[serde(
        default = "default_additional_properties",
        rename = "additionalProperties",
        skip_serializing_if = "is_default_additional_properties"
    )]
    pub additional_properties: Either<bool, ReferenceOr<Box<Self>>>,

    /// Allows interoperable semantic validation for a fixed subset of values which are accurately described by authoritative resources, be they RFCs or other external specifications.
    ///
    /// [JSON Schema Validation, Section 7](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-7)
    #[serde(default, skip_serializing_if = "is_default", rename = "format")]
    pub format: Option<Format>,
    /// This keyword can be used to supply a default JSON value associated with a particular schema.  A default value MUST be valid against the associated schema.
    ///
    /// [Modified](https://spec.openapis.org/oas/v3.0.3.html#properties) from [JSON Schema Validation, Section 6.2](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-6.2)
    #[serde(default, skip_serializing_if = "is_default", rename = "default")]
    pub default: Option<serde_json::Value>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,

    /// A `true` value adds "null" to the allowed type specified by the type keyword, only if type is explicitly defined within the same Schema Object.
    ///
    /// Other Schema Object constraints retain their defined behavior, and therefore may disallow the use of null as a value.
    /// A `false` value leaves the specified or default type unmodified. The default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "nullable")]
    pub nullable: bool,
    /// Adds support for polymorphism.
    ///
    /// The [discriminator](Discriminator) is an object name that is used to differentiate between other schemas which may satisfy the payload description.
    /// See [Composition and Inheritance](https://spec.openapis.org/oas/v3.0.3.html#composition-and-inheritance-polymorphism) for more details.
    #[serde(default, skip_serializing_if = "is_default", rename = "discriminator")]
    pub discriminator: Option<Discriminator>,
    /// Relevant only for Schema [`"properties"`](Self::properties) definitions.
    ///
    /// Declares the property as “read only”.
    ///
    /// This means that it **MAY** be sent as part of a response but **SHOULD NOT** be sent as part of
    /// the request.
    ///
    /// If the property is marked as [`readOnly`](Self::read_only) being `true`
    /// and is in the [required](Self::required) list, the required will take effect on the response only.
    /// A property **MUST NOT** be marked as both [`readOnly`](Self::read_only) and [`writeOnly`](Self::write_only) being `true`.
    ///
    /// Default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "readOnly")]
    pub read_only: bool,
    /// Relevant only for Schema [`"properties"`](Self::properties) definitions.
    ///
    /// Declares the property as “write only”.
    /// Therefore, it **MAY** be sent as part of a request but **SHOULD NOT** be sent as part of the response.
    /// If the property is marked as [`writeOnly`](Self::write_only) being `true` and is in the required list,
    /// the required will take effect on the request only.
    ///
    /// A property **MUST NOT** be marked as both [`readOnly`](Self::read_only) and [`writeOnly`](Self::write_only)
    /// being true.
    ///
    /// Default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "writeOnly")]
    pub write_only: bool,
    /// This **MAY** be used only on properties schemas.
    ///
    /// It has no effect on root schemas.
    /// Adds additional metadata to describe the XML representation of this property.
    #[serde(default, skip_serializing_if = "is_default", rename = "xml")]
    pub xml: Option<Xml>,
    /// Additional external documentation for this schema.
    #[serde(default, skip_serializing_if = "is_default", rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    /// A free-form property to include an example of an instance for this schema.
    /// To represent examples that cannot be naturally represented in JSON or YAML,
    /// a string value can be used to contain the example with escaping where necessary.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "example",
        deserialize_with = "allow_null"
    )]
    pub example: Option<serde_json::Value>,
    /// Specifies that a schema is deprecated and **SHOULD** be transitioned out of usage.
    ///
    /// Default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "deprecated")]
    pub deprecated: bool,
}

fn allow_null<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<serde_json::Value>, D::Error> {
    Ok(Some(Deserialize::deserialize(deserializer)?))
}

fn default_additional_properties() -> Either<bool, ReferenceOr<Box<Schema>>> {
    Either::Left(true)
}
fn is_default_additional_properties(either: &Either<bool, ReferenceOr<Box<Schema>>>) -> bool {
    matches!(either, Either::Left(true))
}

/// [JSON Schema Validation, Section 6.25](https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.25)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Type {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    /// Matches any [number](Self::Number) with a zero fractional part.
    #[serde(rename = "integer")]
    Integer,
}

/// [Modified](https://spec.openapis.org/oas/v3.0.3.html#data-types) from [OpenAPI Specification, Section 4.4](https://spec.openapis.org/oas/v3.0.3.html#data-types)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
    /// Signed 32 bits [i32]
    Int32,
    /// Signed 64 bits [i64]
    Int64,
    /// [f32]
    Float,
    /// [f64]
    Double,
    /// Base64-encoded characters
    Byte,
    /// Any sequence of octets (bytes)
    Binary,
    ///  As defined by `full-date` - [[RFC3339](https://www.rfc-editor.org/info/rfc3339/)] [Section 5.6](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6)
    Date,
    ///  As defined by `date-time` - [[RFC3339](https://www.rfc-editor.org/info/rfc3339/)] [Section 5.6](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6)
    DateTime,
    /// A hint to UIs to obscure input
    Password,
    Other(String),
}

impl<'de> Deserialize<'de> for Format {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        let format = match raw.as_str() {
            "int32" => Format::Int32,
            "int64" => Format::Int64,
            "float" => Format::Float,
            "double" => Format::Double,
            "byte" => Format::Byte,
            "binary" => Format::Binary,
            "date" => Format::Date,
            "date-time" => Format::DateTime,
            "password" => Format::Password,
            _ => Format::Other(raw),
        };
        Ok(format)
    }
}

impl Serialize for Format {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Format::Int32 => "int32",
            Format::Int64 => "int64",
            Format::Float => "float",
            Format::Double => "double",
            Format::Byte => "byte",
            Format::Binary => "binary",
            Format::Date => "date",
            Format::DateTime => "date-time",
            Format::Password => "password",
            Format::Other(other) => other.as_str(),
        }
        .serialize(serializer)
    }
}

/// When request bodies or response payloads may be one of a number of different schemas,
/// a discriminator object can be used to aid in serialization, deserialization, and validation.
///
/// The discriminator is a specific object in a schema which is used to inform the consumer of the
/// specification of an alternative schema based on the value associated with it.
///
/// When using the discriminator, inline schemas will not be considered.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Discriminator {
    ///**REQUIRED**. The name of the property in the payload that will hold the discriminator value.
    #[serde(rename = "propertyName")]
    pub property_name: String,
    /// An object to hold mappings between payload values and schema names or references.
    #[serde(default, skip_serializing_if = "is_default", rename = "mapping")]
    pub mapping: Option<IndexMap<String, String>>,
}

/// A metadata object that allows for more fine-tuned XML model definitions.
///
/// When using arrays, XML element names are not inferred (for singular/plural forms)
/// and the [`name`](Self::name) property **SHOULD** be used to add that information. See examples for expected behavior.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Xml {
    /// Replaces the name of the element/attribute used for the described schema property. When defined within `items`, it will affect the name of the individual XML elements within the list. When defined alongside `type` being `array` (outside the `items`), it will affect the wrapping element and only if `wrapped` is `true`. If `wrapped` is `false`, it will be ignored.
    #[serde(default, skip_serializing_if = "is_default", rename = "name")]
    pub name: Option<String>,
    /// The URI of the namespace definition. Value **MUST** be in the form of an absolute URI.
    #[serde(default, skip_serializing_if = "is_default", rename = "namespace")]
    pub namespace: Option<String>,
    /// The prefix to be used for the [`name`](Xml::name).
    #[serde(default, skip_serializing_if = "is_default", rename = "prefix")]
    pub prefix: Option<String>,
    /// Declares whether the property definition translates to an attribute instead of an element. Default value is `false`.
    #[serde(default, skip_serializing_if = "is_default", rename = "attribute")]
    pub attribute: bool,
    /// **MAY** be used only for an array definition. Signifies whether the array is wrapped (for example, `&lt;books&gt;&lt;book/&gt;&lt;book/&gt;&lt;/books&gt;`) or unwrapped (`&lt;book/&gt;&lt;book/&gt;`). Default value is `false`. The definition takes effect only when defined alongside `type` being `array` (outside the `items`).
    #[serde(default, skip_serializing_if = "is_default", rename = "wrapped")]
    pub wrapped: bool,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Defines a security scheme that can be used by the operations.
///
/// Supported schemes are HTTP authentication, an API key (either as a header, a cookie parameter or as a query parameter),
/// OAuth2’s common flows (implicit, password, client credentials and authorization code) as defined in [[RFC6749](https://www.rfc-editor.org/info/rfc6749/)],
/// and [OpenID Connect Discovery](https://tools.ietf.org/html/draft-ietf-oauth-discovery-06).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    ApiKey {
        ///  A short description for security scheme. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
        #[serde(default, skip_serializing_if = "is_default", rename = "description")]
        description: Option<String>,
        /// **REQUIRED**. The name of the header, query or cookie parameter to be used.
        #[serde(rename = "name")]
        name: String,
        /// **REQUIRED**. The location of the API key.
        #[serde(rename = "in")]
        in_: ApiKeyLocation,

        /// This object **MAY** be extended with [Specification Extensions](Extensions).
        #[serde(default, flatten)]
        extensions: Extensions<Self>,
    },
    #[serde(rename = "http")]
    Http {
        /// A short description for security scheme. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
        #[serde(default, skip_serializing_if = "is_default", rename = "description")]
        description: Option<String>,
        // TODO: This technically could be another enum type, but it needs an overflow variant...
        /// **REQUIRED**. The name of the HTTP Authorization scheme to be used in the Authorization header as defined in [[RFC7235](https://httpwg.org/specs/rfc7235.html)] [Section 4.2](https://httpwg.org/specs/rfc7235.html#section-4.2). The values used **SHOULD** be registered in the IANA Authentication Scheme registry.
        #[serde(rename = "scheme")]
        scheme: String,
        /// A hint to the client to identify how the bearer token is formatted. Bearer tokens are usually generated by an authorization server, so this information is primarily for documentation purposes.
        #[serde(default, skip_serializing_if = "is_default", rename = "bearerFormat")]
        bearer_format: Option<String>,

        /// This object **MAY** be extended with [Specification Extensions](Extensions).
        #[serde(default, flatten)]
        extensions: Extensions<Self>,
    },
    #[serde(rename = "oauth2")]
    Oauth2 {
        ///  A short description for security scheme. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
        #[serde(default, skip_serializing_if = "is_default", rename = "description")]
        description: Option<String>,
        /// **REQUIRED**. An object containing configuration information for the flow types supported.
        #[serde(rename = "flows")]
        flows: OauthFlows,

        /// This object **MAY** be extended with [Specification Extensions](Extensions).
        #[serde(default, flatten)]
        extensions: Extensions<Self>,
    },
    #[serde(rename = "openIdConnect")]
    OpenIdConnect {
        ///  A short description for security scheme. [[CommonMark](https://spec.commonmark.org/)] syntax **MAY** be used for rich text representation.
        #[serde(default, skip_serializing_if = "is_default", rename = "description")]
        description: Option<String>,
        /// **REQUIRED**. OpenId Connect URL to discover OAuth2 configuration values. This **MUST** be in the form of a URL.
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: Url,

        /// This object **MAY** be extended with [Specification Extensions](Extensions).
        #[serde(default, flatten)]
        extensions: Extensions<Self>,
    },
}

/// The location of the API key.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ApiKeyLocation {
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "cookie")]
    Cookie,
}

/// Allows configuration of the supported OAuth Flows.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OauthFlows {
    /// Configuration for the OAuth Implicit flow
    #[serde(default, skip_serializing_if = "is_default", rename = "implicit")]
    pub implicit: Option<Box<OauthFlowImplicit>>,
    /// Configuration for the OAuth Resource Owner Password flow
    #[serde(default, skip_serializing_if = "is_default", rename = "password")]
    pub password: Option<Box<OauthFlowPassword>>,
    /// Configuration for the OAuth Client Credentials flow.  Previously called `application` in OpenAPI 2.0.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "clientCredentials"
    )]
    pub client_credentials: Option<Box<OauthFlowClientCredentials>>,
    /// Configuration for the OAuth Authorization Code flow.  Previously called `accessCode` in OpenAPI 2.0.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "authorizationCode"
    )]
    pub authorization_code: Option<Box<OauthFlowAuthorizationCode>>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Configuration for the OAuth Authorization Code flow.  Previously called `accessCode` in OpenAPI 2.0.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OauthFlowAuthorizationCode {
    /// **REQUIRED**. The authorization URL to be used for this flow. This **MUST** be in the form of a URL.
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: Url,
    /// **REQUIRED**. The token URL to be used for this flow. This **MUST** be in the form of a URL.
    #[serde(rename = "tokenUrl")]
    pub token_url: Url,
    /// The URL to be used for obtaining refresh tokens. This **MUST** be in the form of a URL.
    #[serde(default, skip_serializing_if = "is_default", rename = "refreshUrl")]
    pub refresh_url: Option<Url>,
    /// **REQUIRED**. The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it. The map MAY be empty.
    #[serde(rename = "scopes")]
    pub scopes: IndexMap<String, String>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Configuration for the OAuth Client Credentials flow.  Previously called `application` in OpenAPI 2.0.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OauthFlowClientCredentials {
    /// **REQUIRED**. The token URL to be used for this flow. This **MUST** be in the form of a URL.
    #[serde(rename = "tokenUrl")]
    pub token_url: Url,
    /// The URL to be used for obtaining refresh tokens. This **MUST** be in the form of a URL.
    #[serde(default, skip_serializing_if = "is_default", rename = "refreshUrl")]
    pub refresh_url: Option<Url>,
    /// **REQUIRED**. The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it. The map MAY be empty.
    #[serde(rename = "scopes")]
    pub scopes: IndexMap<String, String>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Configuration for the OAuth Resource Owner Password flow
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OauthFlowPassword {
    /// **REQUIRED**. The token URL to be used for this flow. This **MUST** be in the form of a URL.
    #[serde(default, skip_serializing_if = "is_default", rename = "tokenUrl")]
    pub token_url: Option<Url>,
    /// The URL to be used for obtaining refresh tokens. This **MUST** be in the form of a URL.
    #[serde(default, skip_serializing_if = "is_default", rename = "refreshUrl")]
    pub refresh_url: Option<Url>,
    /// **REQUIRED**. The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it. The map MAY be empty.
    #[serde(rename = "scopes")]
    pub scopes: IndexMap<String, String>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Configuration for the OAuth Implicit flow
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OauthFlowImplicit {
    /// **REQUIRED**. The authorization URL to be used for this flow. This **MUST** be in the form of a URL.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "authorizationUrl"
    )]
    pub authorization_url: Option<Url>,
    /// The URL to be used for obtaining refresh tokens. This **MUST** be in the form of a URL.
    #[serde(default, skip_serializing_if = "is_default", rename = "refreshUrl")]
    pub refresh_url: Option<Url>,
    /// **REQUIRED**. The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it. The map MAY be empty.
    #[serde(rename = "scopes")]
    pub scopes: IndexMap<String, String>,

    /// This object **MAY** be extended with [Specification Extensions](Extensions).
    #[serde(default, flatten)]
    pub extensions: Extensions<Self>,
}

/// Lists the required security schemes to execute this operation.
///
/// The name used for each property **MUST** correspond to a security scheme declared in the [Security Schemes](Components::security_schemes) under the [Components Object](Components).
///
/// Security Requirement Objects that contain multiple schemes require that all schemes **MUST** be satisfied for a request to be authorized.
/// This enables support for scenarios where multiple query parameters or HTTP headers are required to convey security information.
///
/// When a list of Security Requirement Objects is defined on the [OpenAPI Object](OpenApi::security) or [Operation Object](Operation), only
/// one of the Security Requirement Objects in the list needs to be satisfied to authorize the request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct SecurityRequirement(IndexMap<String, Vec<String>>);

/// Alias for `T | Reference Object`.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Reference(Reference<T>),
    Inline(T),
}

impl<'de, T> Deserialize<'de> for ReferenceOr<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let value = serde_json::Value::deserialize(deserializer)?;
        // Manually check if a reference...
        if value.get("$ref").is_some() {
            Deserialize::deserialize(value)
                .map(Self::Reference)
                .map_err(D::Error::custom)
        } else {
            Deserialize::deserialize(value)
                .map(Self::Inline)
                .map_err(D::Error::custom)
        }
    }
}

impl<T: Default> Default for ReferenceOr<T> {
    fn default() -> Self {
        ReferenceOr::Inline(T::default())
    }
}

pub use crate::ext::{email::Email, http::HttpStatusCode, url::Url};
pub use crate::extensions::Extensions;
use crate::extensions::MixedPatternVisitor;
pub use crate::utils::Either;

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &T::default()
}

#[cfg(test)]
mod tests {
    use crate::document::*;
    macro_rules! example {
        (json @ $id: ident, $ty: ty, $json: tt) => {
            #[test]
            fn $id() {
                let value = serde_json::json!($json);
                let _: $ty = serde::Deserialize::deserialize(value).unwrap();
            }
        };

        (yaml @ $id: ident, $ty: ty, $yaml: expr) => {
            #[test]
            fn $id() {
                let _: $ty = serde_yaml::from_str($yaml).unwrap();
            }
        };
    }

    example!(json @ info_example_json, Info, { "title": "Sample Pet Store App", "description": "This is a sample server for a pet store.", "termsOfService": "http://example.com/terms/", "contact": { "name": "API Support", "url": "http://www.example.com/support", "email": "support@example.com" }, "license": { "name": "Apache 2.0", "url": "https://www.apache.org/licenses/LICENSE-2.0.html" }, "version": "1.0.1" });
    example!(yaml @ info_example_yaml, Info, "title: Sample Pet Store App\ndescription: This is a sample server for a pet store.\ntermsOfService: http://example.com/terms/\ncontact:\n  name: API Support\n  url: http://www.example.com/support\n  email: support@example.com\nlicense:\n  name: Apache 2.0\n  url: https://www.apache.org/licenses/LICENSE-2.0.html\nversion: 1.0.1\n");

    example!(json @ contact_example_json, Contact, { "name": "API Support", "url": "http://www.example.com/support", "email": "support@example.com" });
    example!(yaml @ contact_example_yaml, Contact, "name: API Support\nurl: http://www.example.com/support\nemail: support@example.com\n");

    example!(json @ license_example_json, License, { "name": "Apache 2.0", "url": "https://www.apache.org/licenses/LICENSE-2.0.html" });
    example!(yaml @ license_example_yaml, License, "name: Apache 2.0\nurl: https://www.apache.org/licenses/LICENSE-2.0.html");

    example!(json @ server_example_json, Server, { "url": "https://development.gigantic-server.com/v1", "description": "Development server" });
    example!(yaml @ server_example_yaml, Server, "url: https://development.gigantic-server.com/v1\ndescription: Development server");
    example!(json @ server_example_multiple_json, Vec<Server>, [{ "url": "https://{username}.gigantic-server.com:{port}/{basePath}", "description": "The production API server", "variables": { "username": { "default": "demo", "description": "this value is assigned by the service provider, in this example `gigantic-server.com`" }, "port": { "enum": ["8443", "443"], "default": "8443" }, "basePath": { "default": "v2" } } }]);
    example!(yaml @ server_example_multiple_yaml, Vec<Server>, "- url: https://{username}.gigantic-server.com:{port}/{basePath}\n  description: The production API server\n  variables:\n    username:\n      # note! no enum here means it is an open value\n      default: demo\n      description: this value is assigned by the service provider, in this example `gigantic-server.com`\n    port:\n      enum:\n        - \'8443\'\n        - \'443\'\n      default: \'8443\'\n    basePath:\n      # open meaning there is the opportunity to use special base paths as assigned by the provider, default is `v2`\n      default: v2");

    // ServerVariable: No official spec examples!

    example!(json @ components_example_json, Components, { "schemas": { "GeneralError": { "type": "object", "properties": { "code": { "type": "integer", "format": "int32" }, "message": { "type": "string" } } }, "Category": { "type": "object", "properties": { "id": { "type": "integer", "format": "int64" }, "name": { "type": "string" } } }, "Tag": { "type": "object", "properties": { "id": { "type": "integer", "format": "int64" }, "name": { "type": "string" } } } }, "parameters": { "skipParam": { "name": "skip", "in": "query", "description": "number of items to skip", "required": true, "schema": { "type": "integer", "format": "int32" } }, "limitParam": { "name": "limit", "in": "query", "description": "max records to return", "required": true, "schema" : { "type": "integer", "format": "int32" } } }, "responses": { "NotFound": { "description": "Entity not found." }, "IllegalInput": { "description": "Illegal input for operation." }, "GeneralError": { "description": "General Error", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/GeneralError" } } } } }, "securitySchemes": { "api_key": { "type": "apiKey", "name": "api_key", "in": "header" }, "petstore_auth": { "type": "oauth2", "flows": { "implicit": { "authorizationUrl": "http://example.org/api/oauth/dialog", "scopes": { "write:pets": "modify pets in your account", "read:pets": "read your pets" } } } } } });
    example!(yaml @ components_example_yaml, Components, "schemas:\n  GeneralError:\n    type: object\n    properties:\n      code:\n        type: integer\n        format: int32\n      message:\n        type: string\n  Category:\n    type: object\n    properties:\n      id:\n        type: integer\n        format: int64\n      name:\n        type: string\n  Tag:\n    type: object\n    properties:\n      id:\n        type: integer\n        format: int64\n      name:\n        type: string\nparameters:\n  skipParam:\n    name: skip\n    in: query\n    description: number of items to skip\n    required: true\n    schema:\n      type: integer\n      format: int32\n  limitParam:\n    name: limit\n    in: query\n    description: max records to return\n    required: true\n    schema:\n      type: integer\n      format: int32\nresponses:\n  NotFound:\n    description: Entity not found.\n  IllegalInput:\n    description: Illegal input for operation.\n  GeneralError:\n    description: General Error\n    content:\n      application/json:\n        schema:\n          $ref: \'#/components/schemas/GeneralError\'\nsecuritySchemes:\n  api_key:\n    type: apiKey\n    name: api_key\n    in: header\n  petstore_auth:\n    type: oauth2\n    flows:\n      implicit:\n        authorizationUrl: http://example.org/api/oauth/dialog\n        scopes:\n          write:pets: modify pets in your account\n          read:pets: read your pets");

    // TODO: Add the field name examples

    example!(json @ paths_example_json, Paths, { "/pets": { "get": { "description": "Returns all pets from the system that the user has access to", "responses": { "200": { "description": "A list of pets.", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/pet" } } } } } } } } });
    example!(yaml @ paths_example_yaml, Paths, "/pets:\n  get:\n    description: Returns all pets from the system that the user has access to\n    responses:\n      \'200\':\n        description: A list of pets.\n        content:\n          application/json:\n            schema:\n              type: array\n              items:\n                $ref: \'#/components/schemas/pet\'");

    example!(json @ path_item_example_json, PathItem, { "get": { "description": "Returns pets based on ID", "summary": "Find pets by ID", "operationId": "getPetsById", "responses": { "200": { "description": "pet response", "content": { "*/*": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/Pet" } } } } }, "default": { "description": "error payload", "content": { "text/html": { "schema": { "$ref": "#/components/schemas/ErrorModel" } } } } } }, "parameters": [{ "name": "id", "in": "path", "description": "ID of pet to use", "required": true, "schema": { "type": "array", "items": { "type": "string" } }, "style": "simple" }] });
    example!(yaml @ path_item_example_yaml, PathItem, "get:\n  description: Returns pets based on ID\n  summary: Find pets by ID\n  operationId: getPetsById\n  responses:\n    \'200\':\n      description: pet response\n      content:\n        \'*/*\' :\n          schema:\n            type: array\n            items:\n              $ref: \'#/components/schemas/Pet\'\n    default:\n      description: error payload\n      content:\n        \'text/html\':\n          schema:\n            $ref: \'#/components/schemas/ErrorModel\'\nparameters:\n- name: id\n  in: path\n  description: ID of pet to use\n  required: true\n  schema:\n    type: array\n    items:\n      type: string \n  style: simple\n");

    example!(json @ operation_example_json, Operation, { "tags": ["pet"], "summary": "Updates a pet in the store with form data", "operationId": "updatePetWithForm", "parameters": [{ "name": "petId", "in": "path", "description": "ID of pet that needs to be updated", "required": true, "schema": { "type": "string" } }], "requestBody": { "content": { "application/x-www-form-urlencoded": { "schema": { "type": "object", "properties": { "name": { "description": "Updated name of the pet", "type": "string" }, "status": { "description": "Updated status of the pet", "type": "string" } }, "required": ["status"] } } } }, "responses": { "200": { "description": "Pet updated.", "content": { "application/json": {}, "application/xml": {} } }, "405": { "description": "Method Not Allowed", "content": { "application/json": {}, "application/xml": {} } } }, "security": [{ "petstore_auth": ["write:pets", "read:pets"] }] });
    example!(yaml @ operation_example_yaml, Operation, "tags:\n- pet\nsummary: Updates a pet in the store with form data\noperationId: updatePetWithForm\nparameters:\n- name: petId\n  in: path\n  description: ID of pet that needs to be updated\n  required: true\n  schema:\n    type: string\nrequestBody:\n  content:\n    \'application/x-www-form-urlencoded\':\n      schema:\n       properties:\n          name:\n            description: Updated name of the pet\n            type: string\n          status:\n            description: Updated status of the pet\n            type: string\n       required:\n         - status\nresponses:\n  \'200\':\n    description: Pet updated.\n    content:\n      \'application/json\': {}\n      \'application/xml\': {}\n  \'405\':\n    description: Method Not Allowed\n    content:\n      \'application/json\': {}\n      \'application/xml\': {}\nsecurity:\n- petstore_auth:\n  - write:pets\n  - read:pets");

    example!(json @ external_documentation_example_json, ExternalDocumentation, { "description": "Find more info here", "url": "https://example.com" });
    example!(yaml @ external_documentation_example_yaml, ExternalDocumentation, "description: Find more info here\nurl: https://example.com");

    example!(json @ parameter_example_i64_json, Parameter, { "name": "token", "in": "header", "description": "token to be passed as a header", "required": true, "schema": { "type": "array", "items": { "type": "integer", "format": "int64" } }, "style": "simple" });
    example!(yaml @ parameter_example_i64_yaml, Parameter, "name: token\nin: header\ndescription: token to be passed as a header\nrequired: true\nschema:\n  type: array\n  items:\n    type: integer\n    format: int64\nstyle: simple");
    example!(json @ parameter_example_string_json, Parameter, { "name": "username", "in": "path", "description": "username to fetch", "required": true, "schema": { "type": "string" } });
    example!(yaml @ parameter_example_string_yaml, Parameter, "name: username\nin: path\ndescription: username to fetch\nrequired: true\nschema:\n  type: string\n");
    example!(json @ parameter_example_optional_query_json, Parameter, { "name": "id", "in": "query", "description": "ID of the object to fetch", "required": false, "schema": { "type": "array", "items": { "type": "string" } }, "style": "form", "explode": true });
    example!(yaml @ parameter_example_optional_query_yaml, Parameter, "name: id\nin: query\ndescription: ID of the object to fetch\nrequired: false\nschema:\n  type: array\n  items:\n    type: string\nstyle: form\nexplode: true");
    example!(json @ parameter_example_free_form_query_json, Parameter, { "in": "query", "name": "freeForm", "schema": { "type": "object", "additionalProperties": { "type": "integer" } }, "style": "form" });
    example!(yaml @ parameter_example_free_form_query_yaml, Parameter, "in: query\nname: freeForm\nschema:\n  type: object\n  additionalProperties:\n    type: integer\nstyle: form");
    example!(json @ parameter_example_content_json, Parameter, { "in": "query", "name": "coordinates", "content": { "application/json": { "schema": { "type": "object", "required": ["lat", "long"], "properties": { "lat": { "type": "number" }, "long": { "type": "number" } } } } } });
    example!(yaml @ parameter_example_content_yaml, Parameter, "in: query\nname: coordinates\ncontent:\n  application/json:\n    schema:\n      type: object\n      required:\n        - lat\n        - long\n      properties:\n        lat:\n          type: number\n        long:\n          type: number");

    example!(json @ request_body_example_referenced_model_json, RequestBody, { "description": "user to add to the system", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/User" }, "examples": { "user" : { "summary": "User Example", "externalValue": "http://foo.bar/examples/user-example.json" } } }, "application/xml": { "schema": { "$ref": "#/components/schemas/User" }, "examples": { "user" : { "summary": "User example in XML", "externalValue": "http://foo.bar/examples/user-example.xml" } } }, "text/plain": { "examples": { "user" : { "summary": "User example in Plain text", "externalValue": "http://foo.bar/examples/user-example.txt" } } }, "*/*": { "examples": { "user" : { "summary": "User example in other format", "externalValue": "http://foo.bar/examples/user-example.whatever" } } } } });
    example!(yaml @ request_body_example_referenced_model_yaml, RequestBody, "description: user to add to the system\ncontent:\n  \'application/json\':\n    schema:\n      $ref: \'#/components/schemas/User\'\n    examples:\n      user:\n        summary: User Example\n        externalValue: \'http://foo.bar/examples/user-example.json\'\n  \'application/xml\':\n    schema:\n      $ref: \'#/components/schemas/User\'\n    examples:\n      user:\n        summary: User Example in XML\n        externalValue: \'http://foo.bar/examples/user-example.xml\'\n  \'text/plain\':\n    examples:\n      user:\n        summary: User example in text plain format\n        externalValue: \'http://foo.bar/examples/user-example.txt\'\n  \'*/*\':\n    examples:\n      user:\n        summary: User example in other format\n        externalValue: \'http://foo.bar/examples/user-example.whatever\'");
    example!(json @ request_body_example_array_string_json, RequestBody, { "description": "user to add to the system", "content": { "text/plain": { "schema": { "type": "array", "items": { "type": "string" } } } } });
    example!(yaml @ request_body_example_array_string_yaml, RequestBody, "description: user to add to the system\nrequired: true\ncontent:\n  text/plain:\n    schema:\n      type: array\n      items:\n        type: string");

    // Spec is wrong: `frog` example is one level too deep...
    example!(json @ media_type_example_json, MediaType, { "schema": { "$ref": "#/components/schemas/Pet" }, "examples": { "cat" : { "summary": "An example of a cat", "value": { "name": "Fluffy", "petType": "Cat", "color": "White", "gender": "male", "breed": "Persian" } }, "dog": { "summary": "An example of a dog with a cat's name", "value" :  { "name": "Puma", "petType": "Dog", "color": "Black", "gender": "Female", "breed": "Mixed" } },"frog": { "$ref": "#/components/examples/frog-example" } } });
    example!(yaml @ media_type_example_yaml, MediaType, "schema:\n    $ref: \"#/components/schemas/Pet\"\nexamples:\n    cat:\n        summary: An example of a cat\n        value:\n            name: Fluffy\n            petType: Cat\n            color: White\n            gender: male\n            breed: Persian\n    dog:\n        summary: An example of a dog with a cat\'s name\n        value:\n            name: Puma\n            petType: Dog\n            color: Black\n            gender: Female\n            breed: Mixed\n    frog:\n        $ref: \"#/components/examples/frog-example\"");
    // Skipping two examples
    example!(yaml @ media_type_example_file_upload_base64_yaml, MediaType, "# content transferred with base64 encoding\nschema:\n  type: string\n  format: base64");
    example!(yaml @ media_type_example_file_upload_binary_yaml, MediaType, "# content transferred in binary (octet-stream):\nschema:\n  type: string\n  format: binary\n");
    example!(yaml @ media_type_example_file_upload_multiple_files_yaml, MediaType, "schema:\n  properties:\n    # The property name \'file\' will be used for all files.\n    file:\n      type: array\n      items:\n        type: string\n        format: binary");
    example!(yaml @ media_type_example_file_upload_urlencoded_yaml, MediaType, "schema:\n  type: object\n  properties:\n    id:\n      type: string\n      format: uuid\n    address:\n      # complex types are stringified to support RFC 1866\n      type: object\n      properties: {}");
    // This example is actually wrong within the spec (schema.properties.addresses.type is not JSON Schema Validation-compliant). Oops!
    example!(yaml @ media_type_example_file_upload_multipart_yaml, MediaType, "schema:\n  type: object\n  properties:\n    id:\n      type: string\n      format: uuid\n    address:\n      # default Content-Type for objects is `application/json`\n      type: object\n      properties: {}\n    profileImage:\n      # default Content-Type for string/binary is `application/octet-stream`\n      type: string\n      format: binary\n    children:\n      # default Content-Type for arrays is based on the `inner` type (text/plain here)\n      type: array\n      items:\n        type: string\n    addresses:\n      # default Content-Type for arrays is based on the `inner` type (object shown, so `application/json` in this example)\n      type: array\n      items:\n        $ref: \'#/components/schemas/Address\'");

    example!(yaml @ encoding_example_xml_yaml, Encoding, "# require XML Content-Type in utf-8 encoding\ncontentType: application/xml; charset=utf-8\n    ");
    example!(yaml @ encoding_example_image_yaml, Encoding, "# only accept png/jpeg\ncontentType: image/png, image/jpeg\nheaders:\n  X-Rate-Limit-Limit:\n    description: The number of allowed requests in the current period\n    schema:\n      type: integer\n");

    example!(json @ responses_example_json, Responses, { "200": { "description": "a pet to be returned", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Pet" } } } }, "default": { "description": "Unexpected error", "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ErrorModel" } } } } });
    example!(yaml @ responses_example_yaml, Responses, "\'200\':\n  description: a pet to be returned\n  content:\n    application/json:\n      schema:\n        $ref: \'#/components/schemas/Pet\'\ndefault:\n  description: Unexpected error\n  content:\n    application/json:\n      schema:\n        $ref: \'#/components/schemas/ErrorModel\'");

    example!(json @ response_example_json, Response, { "description": "A complex object array response", "content": { "application/json": { "schema": { "type": "array", "items": { "$ref": "#/components/schemas/VeryComplexType" } } } } });
    example!(yaml @ response_example_yaml, Response, "description: A complex object array response\ncontent:\n  application/json:\n    schema:\n      type: array\n      items:\n        $ref: \'#/components/schemas/VeryComplexType\'");
    example!(json @ response_example_string_json, Response, { "description": "A simple string response", "content": { "text/plain": { "schema": { "type": "string" } } } });
    example!(yaml @ response_example_string_yaml, Response, "description: A simple string response\ncontent:\n  text/plain:\n    schema:\n      type: string");
    example!(json @ response_example_headers_json, Response, { "description": "A simple string response", "content": { "text/plain": { "schema": { "type": "string", "example": "whoa!" } } }, "headers": { "X-Rate-Limit-Limit": { "description": "The number of allowed requests in the current period", "schema": { "type": "integer" } }, "X-Rate-Limit-Remaining": { "description": "The number of remaining requests in the current period", "schema": { "type": "integer" } }, "X-Rate-Limit-Reset": { "description": "The number of seconds left in the current period", "schema": { "type": "integer" } } } });
    example!(yaml @ response_example_headers_yaml, Response, "description: A simple string response\ncontent:\n  text/plain:\n    schema:\n      type: string\n    example: \'whoa!\'\nheaders:\n  X-Rate-Limit-Limit:\n    description: The number of allowed requests in the current period\n    schema:\n      type: integer\n  X-Rate-Limit-Remaining:\n    description: The number of remaining requests in the current period\n    schema:\n      type: integer\n  X-Rate-Limit-Reset:\n    description: The number of seconds left in the current period\n    schema:\n      type: integer");
    example!(json @ response_example_no_return_json, Response, { "description": "object created" });
    example!(yaml @ response_example_no_return_yaml, Response, "description: object created");

    example!(yaml @ callback_example_yaml, Callback, "\'{$request.query.queryUrl}\':\n  post:\n    requestBody:\n      description: Callback payload\n      content:\n        \'application/json\':\n          schema:\n            $ref: \'#/components/schemas/SomePayload\'\n    responses:\n      \'200\':\n        description: callback successfully processed");
    example!(yaml @ callback_example_with_params_yaml, Callback, "\'http://notificationServer.com?transactionId={$request.body#/id}&email={$request.body#/email}\':\n  post:\n    requestBody:\n      description: Callback payload\n      content:\n        \'application/json\':\n          schema:\n            $ref: \'#/components/schemas/SomePayload\'\n    responses:\n      \'200\':\n        description: callback successfully processed");

    example!(yaml @ example_example_foo_yaml, Example, "summary: A foo example\nvalue: {\"foo\": \"bar\"}");
    example!(yaml @ example_example_bar_yaml, Example, "summary: A bar example\nvalue: {\"bar\": \"baz\"}");
    example!(yaml @ example_example_xml_yaml, Example, "summary: This is an example in XML\nexternalValue: 'http://example.org/examples/address-example.xml'");
    example!(yaml @ example_example_zip_yaml, ReferenceOr<Example>, "$ref: \'#/components/examples/zip-example\'");
    example!(yaml @ example_example_confirmation_success_yaml, ReferenceOr<Example>, "$ref: \'#/components/examples/confirmation-success\'");

    example!(yaml @ link_example_yaml, Link, "# the target link operationId\noperationId: getUserAddress\nparameters:\n  # get the `id` field from the request path parameter named `id`\n  userId: $request.path.id");
    example!(yaml @ link_example_uuid_yaml, Link, "operationId: getUserAddressByUUID\nparameters:\n  # get the `uuid` field from the `uuid` field in the response body\n  userUuid: $response.body#/uuid");
    example!(yaml @ link_example_operation_ref_relative_yaml, Link, "# returns array of \'#/components/schemas/repository\'\noperationRef: \'#/paths/~12.0~1repositories~1{username}/get\'\nparameters:\n    username: $response.body#/username");
    example!(yaml @ link_example_operation_ref_absolute_yaml, Link, "# returns array of \'#/components/schemas/repository\'\noperationRef: \'https://na2.gigantic-server.com/#/paths/~12.0~1repositories~1{username}/get\'\nparameters:\n    username: $response.body#/username");

    example!(json @ header_example_json, Header, { "description": "The number of allowed requests in the current period", "schema": { "type": "integer" } });
    example!(yaml @ header_example_yaml, Header, "description: The number of allowed requests in the current period\nschema:\n  type: integer");

    example!(json @ tag_example_json, Tag, { "name": "pet", "description": "Pets operations" });
    example!(yaml @ tag_example_yaml, Tag, "name: pet\ndescription: Pets operations");

    example!(json @ reference_example_json, Reference<Schema>, { "$ref": "#/components/schemas/Pet" });
    example!(yaml @ reference_example_yaml, Reference<Schema>, "$ref: \'#/components/schemas/Pet\'");
    example!(json @ reference_example_relative_json, Reference<Schema>, { "$ref": "Pet.json" });
    example!(yaml @ reference_example_relative_yaml, Reference<Schema>, "$ref: Pet.yaml");
    example!(json @ reference_example_relative_embedded_json, Reference<Schema>, { "$ref": "definitions.json#/Pet" });
    example!(yaml @ reference_example_relative_embedded_yaml, Reference<Schema>, "$ref: definitions.yaml#/Pet");

    example!(json @ schema_example_primitive_json, Schema, { "type": "string", "format": "email" });
    example!(yaml @ schema_example_primitive_yaml, Schema, "type: string\nformat: email\n");
    example!(json @ schema_example_simple_json, Schema, { "type": "object", "required": ["name"], "properties": { "name": { "type": "string" }, "address": { "$ref": "#/components/schemas/Address" }, "age": { "type": "integer", "format": "int32", "minimum": 0 } } });
    example!(yaml @ schema_example_simple_yaml, Schema, "type: object\nrequired:\n- name\nproperties:\n  name:\n    type: string\n  address:\n    $ref: \'#/components/schemas/Address\'\n  age:\n    type: integer\n    format: int32\n    minimum: 0\n");
    example!(json @ schema_example_dictionary_json, Schema, { "type": "object", "additionalProperties": { "type": "string" } });
    example!(yaml @ schema_example_dictionary_yaml, Schema, "type: object\nadditionalProperties:\n  type: string");
    example!(json @ schema_example_dictionary_model_json, Schema, {  "type": "object", "additionalProperties": { "$ref": "#/components/schemas/ComplexModel" } });
    example!(yaml @ schema_example_dictionary_model_yaml, Schema, "type: object\nadditionalProperties:\n  $ref: \'#/components/schemas/ComplexModel\'");
    example!(json @ schema_example_example_json, Schema, { "type": "object", "properties": { "id": { "type": "integer", "format": "int64" }, "name": { "type": "string" } }, "required": ["name"], "example": { "name": "Puma", "id": 1 } });
    example!(yaml @ schema_example_example_yaml, Schema, "type: object\nproperties:\n  id:\n    type: integer\n    format: int64\n  name:\n    type: string\nrequired:\n- name\nexample:\n  name: Puma\n  id: 1");
    example!(json @ schema_example_composition_json, IndexMap<String, Schema>, { "ErrorModel": { "type": "object", "required": ["message", "code"], "properties": { "message": { "type": "string" }, "code": { "type": "integer", "minimum": 100, "maximum": 600 } } }, "ExtendedErrorModel": { "allOf": [{ "$ref": "#/components/schemas/ErrorModel" }, { "type": "object", "required": ["rootCause"], "properties": { "rootCause": { "type": "string" } } }] } });
    example!(yaml @ schema_example_composition_yaml, IndexMap<String, Schema>, "ErrorModel:\n  type: object\n  required:\n  - message\n  - code\n  properties:\n    message:\n      type: string\n    code:\n      type: integer\n      minimum: 100\n      maximum: 600\nExtendedErrorModel:\n  allOf:\n  - $ref: \'#/components/schemas/ErrorModel\'\n  - type: object\n    required:\n    - rootCause\n    properties:\n      rootCause:\n        type: string");
    example!(json @ schema_example_polymorphism_json, IndexMap<String, Schema>, { "Pet": { "type": "object", "discriminator": { "propertyName": "petType" }, "properties": { "name": { "type": "string" }, "petType": { "type": "string" } }, "required": ["name", "petType"] }, "Cat": { "description": "A representation of a cat. Note that `Cat` will be used as the discriminator value.", "allOf": [{ "$ref": "#/components/schemas/Pet" }, { "type": "object", "properties": { "huntingSkill": { "type": "string", "description": "The measured skill for hunting", "default": "lazy", "enum": ["clueless", "lazy", "adventurous", "aggressive"] } }, "required": ["huntingSkill"] }] }, "Dog": { "description": "A representation of a dog. Note that `Dog` will be used as the discriminator value.", "allOf": [{ "$ref": "#/components/schemas/Pet" }, { "type": "object", "properties": { "packSize": { "type": "integer", "format": "int32", "description": "the size of the pack the dog is from", "default": 0, "minimum": 0 } }, "required": ["packSize"] }] } });
    example!(yaml @ schema_example_polymorphism_yaml, IndexMap<String, Schema>, "Pet:\n  type: object\n  discriminator:\n    propertyName: petType\n  properties:\n    name:\n      type: string\n    petType:\n      type: string\n  required:\n  - name\n  - petType\nCat:  ## \"Cat\" will be used as the discriminator value\n  description: A representation of a cat\n  allOf:\n  - $ref: \'#/components/schemas/Pet\'\n  - type: object\n    properties:\n      huntingSkill:\n        type: string\n        description: The measured skill for hunting\n        enum:\n        - clueless\n        - lazy\n        - adventurous\n        - aggressive\n    required:\n    - huntingSkill\nDog:  ## \"Dog\" will be used as the discriminator value\n  description: A representation of a dog\n  allOf:\n  - $ref: \'#/components/schemas/Pet\'\n  - type: object\n    properties:\n      packSize:\n        type: integer\n        format: int32\n        description: the size of the pack the dog is from\n        default: 0\n        minimum: 0\n    required:\n    - packSize");

    example!(yaml @ discriminator_example_yaml, Discriminator, "propertyName: petType");
    example!(yaml @ discriminator_example_mapping_yaml, Discriminator, "propertyName: petType\nmapping:\n  dog: \'#/components/schemas/Dog\'\n  monster: \'https://gigantic-server.com/schemas/Monster/schema.json\'");
    example!(yaml @ discriminator_example_mapping_dog_yaml, Discriminator, "propertyName: petType\nmapping:\n  dog: Dog");

    example!(json @ xml_example_name_json, Xml, { "name": "animal" });
    example!(yaml @ xml_example_name_yaml, Xml, "name: animal");
    example!(json @ xml_example_json, Xml, { "namespace": "http://example.com/schema/sample", "prefix": "sample" });
    example!(yaml @ xml_example_yaml, Xml, "namespace: http://example.com/schema/sample\nprefix: sample");
    example!(json @ xml_example_wrapped_json, Xml, { "wrapped": true });
    example!(yaml @ xml_example_wrapped_yaml, Xml, "wrapped: true");
    example!(json @ xml_example_wrapped_with_name_json, Xml, { "name": "aliens", "wrapped": true });
    example!(yaml @ xml_example_wrapped_with_name_yaml, Xml, "name: aliens\nwrapped: true");

    example!(json @ security_scheme_basic_example_json, SecurityScheme, { "type": "http", "scheme": "basic" }
    );
    example!(yaml @ security_scheme_basic_example_yaml, SecurityScheme, "type: http\nscheme: basic");
    example!(json @ security_scheme_api_key_example_json, SecurityScheme, { "type": "apiKey", "name": "api_key", "in": "header" });
    example!(yaml @ security_scheme_api_key_example_yaml, SecurityScheme, "type: apiKey\nname: api_key\nin: header");
    example!(json @ security_scheme_jwt_bearer_example_json, SecurityScheme, { "type": "http", "scheme": "bearer", "bearerFormat": "JWT" });
    example!(yaml @ security_scheme_jwt_bearer_example_yaml, SecurityScheme, "type: http\nscheme: bearer\nbearerFormat: JWT");
    example!(json @ security_scheme_implicit_oauth2_example_json, SecurityScheme, { "type": "oauth2", "flows": { "implicit": { "authorizationUrl": "https://example.com/api/oauth/dialog", "scopes": { "write:pets": "modify pets in your account", "read:pets": "read your pets" } } } });
    example!(yaml @ security_scheme_implicit_oauth2_example_yaml, SecurityScheme, "type: oauth2\nflows:\n  implicit:\n    authorizationUrl: https://example.com/api/oauth/dialog\n    scopes:\n      write:pets: modify pets in your account\n      read:pets: read your pets");

    example!(json @ oauth_flows_example_json, OauthFlows, { "implicit": { "authorizationUrl": "https://example.com/api/oauth/dialog", "scopes": { "write:pets": "modify pets in your account", "read:pets": "read your pets" } }, "authorizationCode": { "authorizationUrl": "https://example.com/api/oauth/dialog", "tokenUrl": "https://example.com/api/oauth/token", "scopes": { "write:pets": "modify pets in your account", "read:pets": "read your pets" } } });
    example!(yaml @ oauth_flows_example_yaml, OauthFlows, "implicit:\n  authorizationUrl: https://example.com/api/oauth/dialog\n  scopes:\n    write:pets: modify pets in your account\n    read:pets: read your pets\nauthorizationCode:\n  authorizationUrl: https://example.com/api/oauth/dialog\n  tokenUrl: https://example.com/api/oauth/token\n  scopes:\n    write:pets: modify pets in your account\n    read:pets: read your pets");

    // OAuth Flow Objects: No more examples other than the ones contained above...

    example!(json @ security_requirement_non_oauth2_example_json, SecurityRequirement, { "api_key": [] });
    example!(yaml @ security_requirement_non_oauth2_example_yaml, SecurityRequirement, "api_key: []");
    example!(json @ security_requirement_oauth2_example_json, SecurityRequirement, { "petstore_auth": ["write:pets", "read:pets"] });
    example!(yaml @ security_requirement_oauth2_example_yaml, SecurityRequirement, "petstore_auth:\n- write:pets\n- read:pets");
    example!(json @ security_requirement_optional_oauth2_example_json, Vec<SecurityRequirement>, [{}, { "petstore_auth": ["write:pets", "read:pets"] }]);
    example!(yaml @ security_requirement_optional_oauth2_example_yaml, Vec<SecurityRequirement>, "- {}\n- petstore_auth:\n  - write:pets\n  - read:pets");
}
