//! Wrappers for basic owned types that indicate their usage
#![allow(missing_docs)]

// {{{ macros
#[macro_export]
#[doc(hidden)]
macro_rules! field_wrapper_name {
    ($($type:ty => $field:expr),+) => {
        $(
            impl FieldValue for $type {
                fn field_name() -> &'static str {
                    $field
                }
            }
        )*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! quick_deref_into {
    ($(($type:ty, $inner:ty)),+) => {
        $(
            impl std::ops::Deref for $type {
                type Target = $inner;
                fn deref(&self) -> &$inner {
                    &self.0
                }
            }

            impl std::ops::DerefMut for $type {
                fn deref_mut(&mut self) -> &mut $inner {
                    &mut self.0
                }
            }

            impl $type {
                fn into_inner(self) -> $inner {
                    self.0
                }
            }

        )*
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! from_inner {
    ($(($type:ty, $inner:ty)),+) => {
        $(
            impl From<$inner> for $type {
                fn from(f: $inner) -> Self {
                    Self(f)
                }
            }
        )*
    }
}

// }}}

use serde::{Deserialize, Serialize};

/// Values for broadcaster objects and requests
pub mod broadcasters {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    /// The id number of a broadcaster object
    pub struct BroadcasterId(String);

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    /// The display name of a channel object
    pub struct BroadcasterName(String);

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    /// The language of a broadcaster object
    pub struct BroadcasterLanguage(ISOLanguage);

    field_wrapper_name![
        BroadcasterId => "broadcaster_id",
        BroadcasterName => "broadcaster_name",
        BroadcasterLanguage => "broadcaster_language"
    ];

    quick_deref_into![
        (BroadcasterId, String),
        (BroadcasterName, String),
        (BroadcasterLanguage, ISOLanguage)
    ];

    from_inner![
        (BroadcasterId, String),
        (BroadcasterName, String),
        (BroadcasterLanguage, ISOLanguage)
    ];
}

/// Values for game objects and requests
pub mod games {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct GameName(String);

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    /// The ID number of a game on twitch
    pub struct GameId(String);

    field_wrapper_name![
        GameName => "game_name",
        GameId => "game_id"
    ];

    quick_deref_into![(GameName, String), (GameId, String)];
    from_inner![(GameName, String), (GameId, String)];
}

/// Values for extension objects and requests
pub mod extensions {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    /// The id assined to an extension when it was created
    pub struct ExtensionId(String);

    field_wrapper_name![
        ExtensionId => "extension_id"
    ];

    quick_deref_into![(ExtensionId, String)];
    from_inner![(ExtensionId, String)];
}

/// Values for clip objects and requests
pub mod clips {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ClipId(String);

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ClipTitle(String);

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ViewCount(u32);

    field_wrapper_name![
        ClipId => "id",
        ClipTitle => "title",
        ViewCount => "view_count"
    ];

    quick_deref_into![(ClipId, String), (ClipTitle, String), (ViewCount, u32)];

    from_inner![(ClipId, String), (ClipTitle, String), (ViewCount, u32)];
}

/// Values for user objects and requests
pub mod users {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct UserId(String);

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct UserName(String);

    field_wrapper_name![
        UserId => "id",
        UserName => "user_name"
    ];

    quick_deref_into![(UserId, String), (UserName, String)];

    from_inner![(UserId, String), (UserName, String)];
}

pub mod videos {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct VideoId(String);

    #[repr(transparent)]
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct VideoLanguage(ISOLanguage);

    field_wrapper_name![
        VideoId => "id",
        VideoLanguage => "language"
    ];

    quick_deref_into![(VideoId, String), (VideoLanguage, ISOLanguage)];
    from_inner![(VideoId, String), (VideoLanguage, ISOLanguage)];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// A Pagination key for enpoints that may return more than 100 results
pub struct Pagination {
    pub cursor: Option<String>,
}

impl From<String> for Pagination {
    fn from(inner: String) -> Self {
        Self {
            cursor: Some(inner),
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
/// The max amount returned per page, used in requests like
/// [`crate::resource::clips::get_clips::GetClipsRequest`]
pub struct Count(u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Represents a time window
pub struct Period {
    pub started_at: StartedAt,
    pub ended_at: EndedAt,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
/// Represents a [`RFC3339`] formatted datetime
///
/// [`RFC3339`]: https://datatracker.ietf.org/doc/rfc3339/
pub struct RFC3339Time(String);

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
/// Represents the beginning of a time window
pub struct StartedAt(RFC3339Time);

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
/// Represents the end of a time window
pub struct EndedAt(RFC3339Time);

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
/// Represents a language, either a [`ISO 639-1`] two-letter language code or 'other'
///
/// [`ISO 639-1`]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
pub struct ISOLanguage(String);

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Url(String);

field_wrapper_name![
    Pagination => "pagination",
    Count => "count",

    RFC3339Time => "time",
    Period => "period",
    StartedAt => "started_at",
    EndedAt => "ended_at",
    ISOLanguage => "language"
];

quick_deref_into![
    (RFC3339Time, String),
    (StartedAt, RFC3339Time),
    (EndedAt, RFC3339Time),
    (ISOLanguage, String),
    (Url, String)
];

from_inner![
    (RFC3339Time, String),
    (StartedAt, RFC3339Time),
    (EndedAt, RFC3339Time),
    (ISOLanguage, String),
    (Url, String)
];

/// Used to indicated that this type is used a field value
pub trait FieldValue {
    /// Get the commonly used name of a field of this type that twitch is expecting
    fn field_name() -> &'static str;
}
