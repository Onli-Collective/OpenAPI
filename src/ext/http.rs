//!The HTTP Status Codes are used to indicate the status of the executed operation.
//!
//! The available status codes are defined by [[RFC7231](https://httpwg.org/specs/rfc7231.html)] [Section 6](https://datatracker.ietf.org/doc/html/rfc7231#section-6)
//! and registered status codes are listed in the [IANA Status Code Registry](https://spec.openapis.org/oas/v3.0.3.html#bib-iana-http-status-codes).

use std::ops::Range;

use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub struct HttpStatusCode(Range<u16>);

impl TryFrom<String> for HttpStatusCode {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "1XX" => return Ok(Self(100..200u16)),
            "2XX" => return Ok(Self(200..300u16)),
            "3XX" => return Ok(Self(300..400u16)),
            "4XX" => return Ok(Self(400..500u16)),
            "5XX" => return Ok(Self(500..600u16)),
            _ => (),
        }

        let code = value.parse::<u16>().map_err(|a| a.to_string())?;

        if !(100..=599).contains(&code) {
            return Err("HTTP status code must be between 100 and 599".to_string());
        }

        Ok(Self(code..(code + 1)))
    }
}

impl std::fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            // 1XX, 2XX, 3XX, 4XX, 5XX ranges
            Range {
                start: start @ (100 | 200 | 300 | 400 | 500),
                end,
            } if (end - start) == 100 => {
                write!(f, "{}XX", start / 100)
            }
            // Single status codes
            Range { start, end } if (end - start) == 1 => {
                write!(f, "{start}")
            }
            _ => unimplemented!(),
        }
    }
}

impl From<HttpStatusCode> for String {
    fn from(value: HttpStatusCode) -> Self {
        value.to_string()
    }
}

impl HttpStatusCode {
    /// [RFC 9110, Section 15.2.1](https://www.rfc-editor.org/info/rfc9110/#section-15.2.1)
    pub const CONTINUE: Self = Self(100..(100 + 1));
    /// [RFC 9110, Section 15.2.2](https://www.rfc-editor.org/info/rfc9110/#section-15.2.2)
    pub const SWITCHING_PROTOCOLS: Self = Self(101..(101 + 1));
    /// [RFC 2518](https://www.rfc-editor.org/info/rfc2518)
    pub const PROCESSING: Self = Self(102..(102 + 1));
    /// [RFC 8297](https://www.rfc-editor.org/info/rfc8297)
    pub const EARLY_HINTS: Self = Self(103..(103 + 1));
    /// [draft-ietf-httpbis-resumable-upload-05](https://www.iana.org/go/draft-ietf-httpbis-resumable-upload-05)
    #[deprecated(
        note = "TEMPORARY - registered 2024-11-13, extension registered 2025-09-15, expires 2026-11-13"
    )]
    pub const UPLOAD_RESUMPTION_SUPPORTED: Self = Self(104..(104 + 1));

    // 105-199: Unassigned

    /// [RFC 9110, Section 15.3.1](https://www.rfc-editor.org/info/rfc9110/#section-15.3.1)
    pub const OK: Self = Self(200..(200 + 1));
    /// [RFC 9110, Section 15.3.2](https://www.rfc-editor.org/info/rfc9110/#section-15.3.2)
    pub const CREATED: Self = Self(201..(201 + 1));
    /// [RFC 9110, Section 15.3.3](https://www.rfc-editor.org/info/rfc9110/#section-15.3.3)
    pub const ACCEPTED: Self = Self(202..(202 + 1));
    /// [RFC 9110, Section 15.3.4](https://www.rfc-editor.org/info/rfc9110/#section-15.3.4)
    pub const NON_AUTHORITATIVE_INFORMATION: Self = Self(203..(203 + 1));
    /// [RFC 9110, Section 15.3.5](https://www.rfc-editor.org/info/rfc9110/#section-15.3.5)
    pub const NO_CONTENT: Self = Self(204..(204 + 1));
    /// [RFC 9110, Section 15.3.6](https://www.rfc-editor.org/info/rfc9110/#section-15.3.6)
    pub const RESET_CONTENT: Self = Self(205..(205 + 1));
    /// [RFC 9110, Section 15.3.7](https://www.rfc-editor.org/info/rfc9110/#section-15.3.7)
    pub const PARTIAL_CONTENT: Self = Self(206..(206 + 1));
    /// [RFC 4918](https://www.rfc-editor.org/info/rfc4918)
    pub const MULTI_STATUS: Self = Self(207..(207 + 1));
    /// [RFC 5842](https://www.rfc-editor.org/info/rfc5842)
    pub const ALREADY_REPORTED: Self = Self(208..(208 + 1));

    // 209-225: Unassigned

    /// [RFC 3229](https://www.rfc-editor.org/info/rfc3229)
    pub const IM_USED: Self = Self(226..(226 + 1));

    // 227-299: Unassigned

    /// [RFC 9110, Section 15.4.1](https://www.rfc-editor.org/info/rfc9110/#section-15.4.1)
    pub const MULTIPLE_CHOICES: Self = Self(300..(300 + 1));
    /// [RFC 9110, Section 15.4.2](https://www.rfc-editor.org/info/rfc9110/#section-15.4.2)
    pub const MOVED_PERMANENTLY: Self = Self(301..(301 + 1));
    /// [RFC 9110, Section 15.4.3](https://www.rfc-editor.org/info/rfc9110/#section-15.4.3)
    pub const FOUND: Self = Self(302..(302 + 1));
    /// [RFC 9110, Section 15.4.4](https://www.rfc-editor.org/info/rfc9110/#section-15.4.4)
    pub const SEE_OTHER: Self = Self(303..(303 + 1));
    /// [RFC 9110, Section 15.4.5](https://www.rfc-editor.org/info/rfc9110/#section-15.4.5)
    pub const NOT_MODIFIED: Self = Self(304..(304 + 1));
    /// [RFC 9110, Section 15.4.6](https://www.rfc-editor.org/info/rfc9110/#section-15.4.6)
    pub const USE_PROXY: Self = Self(305..(305 + 1));
    /// [RFC 9110, Section 15.4.7](https://www.rfc-editor.org/info/rfc9110/#section-15.4.7)
    pub const UNUSED306: Self = Self(306..(306 + 1));
    /// [RFC 9110, Section 15.4.8](https://www.rfc-editor.org/info/rfc9110/#section-15.4.8)
    pub const TEMPORARY_REDIRECT: Self = Self(307..(307 + 1));
    /// [RFC 9110, Section 15.4.9](https://www.rfc-editor.org/info/rfc9110/#section-15.4.9)
    pub const PERMANENT_REDIRECT: Self = Self(308..(308 + 1));

    // 309-399: Unassigned

    /// [RFC 9110, Section 15.5.1](https://www.rfc-editor.org/info/rfc9110/#section-15.5.1)
    pub const BAD_REQUEST: Self = Self(400..(400 + 1));
    /// [RFC 9110, Section 15.5.2](https://www.rfc-editor.org/info/rfc9110/#section-15.5.2)
    pub const UNAUTHORIZED: Self = Self(401..(401 + 1));
    /// [RFC 9110, Section 15.5.3](https://www.rfc-editor.org/info/rfc9110/#section-15.5.3)
    pub const PAYMENT_REQUIRED: Self = Self(402..(402 + 1));
    /// [RFC 9110, Section 15.5.4](https://www.rfc-editor.org/info/rfc9110/#section-15.5.4)
    pub const FORBIDDEN: Self = Self(403..(403 + 1));
    /// [RFC 9110, Section 15.5.5](https://www.rfc-editor.org/info/rfc9110/#section-15.5.5)
    pub const NOT_FOUND: Self = Self(404..(404 + 1));
    /// [RFC 9110, Section 15.5.6](https://www.rfc-editor.org/info/rfc9110/#section-15.5.6)
    pub const METHOD_NOT_ALLOWED: Self = Self(405..(405 + 1));
    /// [RFC 9110, Section 15.5.7](https://www.rfc-editor.org/info/rfc9110/#section-15.5.7)
    pub const NOT_ACCEPTABLE: Self = Self(406..(406 + 1));
    /// [RFC 9110, Section 15.5.8](https://www.rfc-editor.org/info/rfc9110/#section-15.5.8)
    pub const PROXY_AUTHENTICATION_REQUIRED: Self = Self(407..(407 + 1));
    /// [RFC 9110, Section 15.5.9](https://www.rfc-editor.org/info/rfc9110/#section-15.5.9)
    pub const REQUEST_TIMEOUT: Self = Self(408..(408 + 1));
    /// [RFC 9110, Section 15.5.10](https://www.rfc-editor.org/info/rfc9110/#section-15.5.10)
    pub const CONFLICT: Self = Self(409..(409 + 1));
    /// [RFC 9110, Section 15.5.11](https://www.rfc-editor.org/info/rfc9110/#section-15.5.11)
    pub const GONE: Self = Self(410..(410 + 1));
    /// [RFC 9110, Section 15.5.12](https://www.rfc-editor.org/info/rfc9110/#section-15.5.12)
    pub const LENGTH_REQUIRED: Self = Self(411..(411 + 1));
    /// [RFC 9110, Section 15.5.13](https://www.rfc-editor.org/info/rfc9110/#section-15.5.13)
    pub const PRECONDITION_FAILED: Self = Self(412..(412 + 1));
    /// [RFC 9110, Section 15.5.14](https://www.rfc-editor.org/info/rfc9110/#section-15.5.14)
    pub const CONTENT_TOO_LARGE: Self = Self(413..(413 + 1));
    /// [RFC 9110, Section 15.5.15](https://www.rfc-editor.org/info/rfc9110/#section-15.5.15)
    pub const URI_TOO_LONG: Self = Self(414..(414 + 1));
    /// [RFC 9110, Section 15.5.16](https://www.rfc-editor.org/info/rfc9110/#section-15.5.16)
    pub const UNSUPPORTED_MEDIA_TYPE: Self = Self(415..(415 + 1));
    /// [RFC 9110, Section 15.5.17](https://www.rfc-editor.org/info/rfc9110/#section-15.5.17)
    pub const RANGE_NOT_SATISFIABLE: Self = Self(416..(416 + 1));
    /// [RFC 9110, Section 15.5.18](https://www.rfc-editor.org/info/rfc9110/#section-15.5.18)
    pub const EXPECTATION_FAILED: Self = Self(417..(417 + 1));

    // 418: Unused (RFC 9110, Section 15.5.19) https://www.rfc-editor.org/info/rfc9110/#section-15.5.19
    // 419-420: Unassigned

    /// [RFC 9110, Section 15.5.20](https://www.rfc-editor.org/info/rfc9110/#section-15.5.20)
    pub const MISDIRECTED_REQUEST: Self = Self(421..(421 + 1));
    /// [RFC 9110, Section 15.5.21](https://www.rfc-editor.org/info/rfc9110/#section-15.5.21)
    pub const UNPROCESSABLE_CONTENT: Self = Self(422..(422 + 1));
    /// [RFC 4918](https://www.rfc-editor.org/info/rfc4918)
    pub const LOCKED: Self = Self(423..(423 + 1));
    /// [RFC 4918](https://www.rfc-editor.org/info/rfc4918)
    pub const FAILED_DEPENDENCY: Self = Self(424..(424 + 1));
    /// [RFC 8470](https://www.rfc-editor.org/info/rfc8470)
    pub const TOO_EARLY: Self = Self(425..(425 + 1));
    /// [RFC 9110, Section 15.5.22](https://www.rfc-editor.org/info/rfc9110/#section-15.5.22)
    pub const UPGRADE_REQUIRED: Self = Self(426..(426 + 1));

    // 427: Unassigned

    /// [RFC 6585](https://www.rfc-editor.org/info/rfc6585)
    pub const PRECONDITION_REQUIRED: Self = Self(428..(428 + 1));
    /// [RFC 6585](https://www.rfc-editor.org/info/rfc6585)
    pub const TOO_MANY_REQUESTS: Self = Self(429..(429 + 1));

    // 430: Unassigned

    /// [RFC 6585](https://www.rfc-editor.org/info/rfc6585)
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: Self = Self(431..(431 + 1));

    // 432-450: Unassigned

    /// [RFC 7725](https://www.rfc-editor.org/info/rfc7725)
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: Self = Self(451..(451 + 1));

    // 452-499: Unassigned

    /// [RFC 9110, Section 15.6.1](https://www.rfc-editor.org/info/rfc9110/#section-15.6.1)
    pub const INTERNAL_SERVER_ERROR: Self = Self(500..(500 + 1));
    /// [RFC 9110, Section 15.6.2](https://www.rfc-editor.org/info/rfc9110/#section-15.6.2)
    pub const NOT_IMPLEMENTED: Self = Self(501..(501 + 1));
    /// [RFC 9110, Section 15.6.3](https://www.rfc-editor.org/info/rfc9110/#section-15.6.3)
    pub const BAD_GATEWAY: Self = Self(502..(502 + 1));
    /// [RFC 9110, Section 15.6.4](https://www.rfc-editor.org/info/rfc9110/#section-15.6.4)
    pub const SERVICE_UNAVAILABLE: Self = Self(503..(503 + 1));
    /// [RFC 9110, Section 15.6.5](https://www.rfc-editor.org/info/rfc9110/#section-15.6.5)
    pub const GATEWAY_TIMEOUT: Self = Self(504..(504 + 1));
    /// [RFC 9110, Section 15.6.6](https://www.rfc-editor.org/info/rfc9110/#section-15.6.6)
    pub const HTTP_VERSION_NOT_SUPPORTED: Self = Self(505..(505 + 1));
    /// [RFC 2295](https://www.rfc-editor.org/info/rfc2295)
    pub const VARIANT_ALSO_NEGOTIATES: Self = Self(506..(506 + 1));
    /// [RFC 4918](https://www.rfc-editor.org/info/rfc4918)
    pub const INSUFFICIENT_STORAGE: Self = Self(507..(507 + 1));
    /// [RFC 5842](https://www.rfc-editor.org/info/rfc5842)
    pub const LOOP_DETECTED: Self = Self(508..(508 + 1));

    // 509: Unassigned,

    /// [RFC 2774; Status change of HTTP experiments to Historic](https://www.rfc-editor.org/info/rfc2774)
    #[deprecated]
    pub const NOT_EXTENDED: Self = Self(510..(510 + 1));
    /// [RFC 6585](https://www.rfc-editor.org/info/rfc6585)
    pub const NETWORK_AUTHENTICATION_REQUIRED: Self = Self(511..(511 + 1));

    // 512-599: Unassigned
}
