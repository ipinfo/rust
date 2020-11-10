//   Copyright 2019 IPinfo library developers
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

//! IPinfo error type and kinds.

use std::{error::Error, fmt};

/// Create a new error (of a given kind) with a formatted message
///
/// # Example
///
/// ```
/// #[macro_use] extern crate ipinfo;
///
/// let err = err!(HTTPClientError, "http client error");
/// ```
#[macro_export]
macro_rules! err {
    ($kind:ident) => {
        $crate::IpError::new($crate::IpErrorKind::$kind, None)
    };
    ($kind:ident, $msg:expr) => {
        $crate::IpError::new($crate::IpErrorKind::$kind, Some($msg))
    };
    ($kind:ident, $fmt:expr, $($arg:tt)+) => {
        err!($crate::IpErrorKind::$kind, &format!($fmt, $($arg)+))
    };
}

/// An enum of errors to represent the possible kinds of `IpError`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IpErrorKind {
    /// HTTP client library error.
    HTTPClientError,

    /// Rate limit exceeded error.
    RateLimitExceededError,

    /// IpInfo Request error.
    IpRequestError,

    /// Parse error.
    ParseError,
}

impl IpErrorKind {
    /// Get string representation of the error.
    pub fn as_str(&self) -> &str {
        match self {
            IpErrorKind::HTTPClientError => "HTTP client library error",
            IpErrorKind::RateLimitExceededError => "rate limit exceeded",
            IpErrorKind::IpRequestError => "application error",
            IpErrorKind::ParseError => "parse error",
        }
    }
}

impl fmt::Display for IpErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The IpError type is the only error type that can be returned from this crate's API.
#[derive(Debug, PartialEq)]
pub struct IpError {
    kind: IpErrorKind,
    description: Option<String>,
}

impl IpError {
    /// Create a new error object with an optional error message.
    ///
    /// # Examples
    ///
    /// ```
    /// use ipinfo::{IpError, IpErrorKind};
    ///
    /// let err = IpError::new(IpErrorKind::HTTPClientError, None);
    /// ```
    pub fn new(kind: IpErrorKind, description: Option<&str>) -> Self {
        Self {
            kind,
            description: description.map(|desc| desc.to_string()),
        }
    }

    /// Get IpErrorKind for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ipinfo::{IpError, IpErrorKind};
    ///
    /// let err = IpError::new(IpErrorKind::HTTPClientError, None);
    /// assert_eq!(err.kind(), IpErrorKind::HTTPClientError);
    /// ```
    pub fn kind(&self) -> IpErrorKind {
        self.kind
    }
}

impl fmt::Display for IpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.description {
            Some(ref desc) => write!(f, "{}: {}", self.kind.as_str(), desc),
            None => write!(f, "{}", self.kind.as_str()),
        }
    }
}

impl Error for IpError {}

impl From<IpErrorKind> for IpError {
    fn from(kind: IpErrorKind) -> Self {
        Self {
            kind,
            description: None,
        }
    }
}

impl From<reqwest::Error> for IpError {
    fn from(err: reqwest::Error) -> Self {
        match err.status() {
            Some(status) => err!(
                HTTPClientError,
                &format!("{}: {}", status, &err.to_string())
            ),
            None => err!(HTTPClientError, &err.to_string()),
        }
    }
}

impl From<serde_json::Error> for IpError {
    fn from(err: serde_json::Error) -> Self {
        err!(ParseError, &err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iperrorkind_string_values() {
        assert_eq!(
            IpErrorKind::HTTPClientError.to_string(),
            "HTTP client library error"
        );
        assert_eq!(
            IpErrorKind::RateLimitExceededError.to_string(),
            "rate limit exceeded"
        );
        assert_eq!(
            IpErrorKind::IpRequestError.to_string(),
            "application error"
        );
        assert_eq!(IpErrorKind::ParseError.to_string(), "parse error");
    }

    #[test]
    fn iperror_new() {
        let err = IpError::new(IpErrorKind::HTTPClientError, None);

        assert_eq!(err.kind(), IpErrorKind::HTTPClientError);
        assert_eq!(err.description, None);
        assert_eq!(err.to_string(), "HTTP client library error");
    }

    #[test]
    fn iperrorkind_convert_to_iperror() {
        let err = IpError::new(IpErrorKind::HTTPClientError, None);
        assert_eq!(err, IpError::from(IpErrorKind::HTTPClientError));
    }
}
