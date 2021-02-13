//! Types relating to [`scopes`] that a [`Bearer Auth token`] can have  
//! Used with [`crate::auth::AuthToken`]
//!
//! [`scopes`]: https://dev.twitch.tv/docs/authentication#scopes
//! [`Bearer Auth Token`]: https://dev.twitch.tv/docs/authentication#sending-user-access-and-app-access-tokens

// {{{ ident!
/// Create identity functions for Eq types
///
/// generates two functions  
/// ```ignore
/// fn ident_to(a: &A)   -> Option<&'static B> { /* ... */ }
/// fn ident_from(b: &B) -> Option<&'static A> { /* ... */ }
/// ```
///
/// ### Examples
///
/// ```
/// # use twitch_api_rs::ident;
/// struct Scopes();
///
/// impl Scopes {
///     ident!(pub <usize, str>: [
///
///         00 => "read",
///         01 => "write",
///
///         02 => "execute"
///
///     ]);
/// }
///
/// // Go from A to B
/// assert_eq!(
///     Scopes::ident_to(&0),
///     Some("read")
/// );
///
/// // Go from B to A
/// assert_eq!(
///     Scopes::ident_from("write"),
///     Some(&1)
/// );
///
/// // Do nothing for undefined values
/// assert_eq!(
///     Scopes::ident_to(&45),
///     None
/// );
/// ```
#[macro_export]
macro_rules! ident {
    (
        $(#[$meta:meta])*
        pub <$ty_a:ty, $ty_b:ty>: [ $( $a:expr => $b:expr ),+ ]
    ) => {
        #[allow(missing_docs)]
        $(#[$meta])*
        pub fn ident_to(a: &$ty_a) -> Option<&'static $ty_b> {
            match a {
                $( $a => Some(&$b), )+
                _ => None,
            }
        }

        #[allow(missing_docs)]
        $(#[$meta])*
        pub fn ident_from(b: &$ty_b) -> Option<&'static $ty_a> {
            match b {
                $( $b => Some(&$a), )+
                _ => None,
            }
        }
    };
    (
        $(#[$meta:meta])*
        <$ty_a:ty, $ty_b:ty>: [ $( $a:expr => $b:expr ),+ ]
    ) => {
        #[allow(missing_docs)]
        $(#[$meta])*
        fn ident_to(a: &$ty_a) -> Option<&'static $ty_b> {
            match a {
                $( $a => Some(&$b), )+
                _ => None,
            }
        }

        #[allow(missing_docs)]
        $(#[$meta])*
        fn ident_from(b: &$ty_b) -> Option<&'static $ty_a> {
            match b {
                $( $b => Some(&$a), )+
                _ => None,
            }
        }
    };
}
// }}}

/// Represents a single twitch Scope
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Scope {
    // General scopes
    AnalyticsReadExtensions = 0,
    AnalyticsReadGames,

    BitsRead,

    ChannelEditCommerial,
    ChannelManageBroadcast,
    ChannelManageExtensions,
    ChannelManageRedemptions,
    ChannelManageVideos,

    ChannelReadEditors,
    ChannelReadHypeTrain,
    ChannelReadRedemptions,
    ChannelReadStreamKey,
    ChannelReadSubscriptions,

    ClipsEdit,

    ModerationRead,

    UserEdit,
    UserEditFollows,
    UserReadBroadcast,
    UserReadEmail,
    UserReadBlockedUsers,

    UserManageBlockedUsers,

    // The following scopes are for for chat and PubSub
    ChannelModerate,

    ChannelEdit,
    ChatRead,

    WhispersRead,
    WhispersEdit,
    // SAFETY: New members must be accounted for in Scope::max(), as that must reflect the total
    // count of enum variants
}

impl Scope {
    const fn max() -> usize {
        // SAFETY: Must be updated to reflect the amount of scopes represented by Scope
        26
    }

    /// Get the Twitch Scope Spec for an internal scope object
    pub fn as_twitch_str(self) -> &'static str {
        match self {
            Self::AnalyticsReadExtensions => "analytics:read:extensions",
            Self::AnalyticsReadGames => "analytics:read:games",
            Self::BitsRead => "bits:read",
            Self::ChannelEditCommerial => "channel:edit:commercial",
            Self::ChannelManageBroadcast => "channel:manage:broadcast",
            Self::ChannelManageExtensions => "channel:manage:extensions",
            Self::ChannelManageRedemptions => "channel:manage:redemptions",
            Self::ChannelManageVideos => "channel:manage:videos",
            Self::ChannelReadEditors => "channel:read:editors",
            Self::ChannelReadHypeTrain => "channel:read:hype_train",
            Self::ChannelReadRedemptions => "channel:read:redemptions",
            Self::ChannelReadStreamKey => "channel:read:stream_key",
            Self::ChannelReadSubscriptions => "channel:read:subscriptions",
            Self::ClipsEdit => "clips:edit",
            Self::ModerationRead => "moderation:read",
            Self::UserEdit => "user:edit",
            Self::UserEditFollows => "user:edit:follows",
            Self::UserReadBroadcast => "user:read:broadcast",
            Self::UserReadEmail => "user:read:email",
            Self::UserReadBlockedUsers => "user:read:blocked_users",
            Self::UserManageBlockedUsers => "user:mange:blocked_users",
            Self::ChannelModerate => "channel:moderate",
            Self::ChannelEdit => "chat:edit",
            Self::ChatRead => "chat:read",
            Self::WhispersRead => "whispers:read",
            Self::WhispersEdit => "whispers:edit",
        }
    }

    /// Turn a Twitch Scope Spec into an internal scope object
    pub fn from_twitch_str(ts: &str) -> Option<Self> {
        match ts {
            "analytics:read:extensions" => Some(Self::AnalyticsReadExtensions),
            "analytics:read:games" => Some(Self::AnalyticsReadGames),
            "bits:read" => Some(Self::BitsRead),
            "channel:edit:commercial" => Some(Self::ChannelEditCommerial),
            "channel:manage:broadcast" => Some(Self::ChannelManageBroadcast),
            "channel:manage:extensions" => Some(Self::ChannelManageExtensions),
            "channel:manage:redemptions" => Some(Self::ChannelManageRedemptions),
            "channel:manage:videos" => Some(Self::ChannelManageVideos),
            "channel:read:editors" => Some(Self::ChannelReadEditors),
            "channel:read:hype_train" => Some(Self::ChannelReadHypeTrain),
            "channel:read:redemptions" => Some(Self::ChannelReadRedemptions),
            "channel:read:stream_key" => Some(Self::ChannelReadStreamKey),
            "channel:read:subscriptions" => Some(Self::ChannelReadSubscriptions),
            "clips:edit" => Some(Self::ClipsEdit),
            "moderation:read" => Some(Self::ModerationRead),
            "user:edit" => Some(Self::UserEdit),
            "user:edit:follows" => Some(Self::UserEditFollows),
            "user:read:broadcast" => Some(Self::UserReadBroadcast),
            "user:read:email" => Some(Self::UserReadEmail),
            "user:read:blocked_users" => Some(Self::UserReadBlockedUsers),
            "user:mange:blocked_users" => Some(Self::UserManageBlockedUsers),
            "channel:moderate" => Some(Self::ChannelModerate),
            "chat:edit" => Some(Self::ChannelEdit),
            "chat:read" => Some(Self::ChatRead),
            "whispers:read" => Some(Self::WhispersRead),
            "whispers:edit" => Some(Self::WhispersEdit),
            _ => None,
        }
    }
}

use bitvec::prelude::{BitArray, Lsb0};

#[derive(Debug, Clone)]
/// Represents a set of scopes available with a specific bearer auth key
pub struct ScopeSet {
    scopes: BitArray<Lsb0, usize>,
}

impl ScopeSet {
    /// Create a new empty set of scopes
    pub fn new() -> Self {
        Self {
            scopes: BitArray::zeroed(),
        }
    }

    #[allow(missing_docs)]
    pub fn contains(&self, scope: Scope) -> bool {
        *(self
            .scopes
            .get(scope as usize)
            .expect("Could not get value from bitset, even though capacity should be large enough"))
    }

    /// Ass a scope to the set, does nothing if the set already contains the scope
    pub fn insert(&mut self, scope: Scope) {
        self.scopes.set(scope as usize, true)
    }

    /// Remove a scope from the set, does nothing if the set does not contain the scope
    pub fn remove(&mut self, scope: Scope) {
        self.scopes.set(scope as usize, false)
    }

    /// Get a borrowing iterator over Self of Twitch Scope Specs
    pub fn spec_iter<'set>(&'set self) -> impl Iterator<Item = &'static str> + 'set {
        SpecIter(ScopeIter {
            cursor: 0,
            set: &self,
        })
    }

    /// Get a borrowing iterator over Self of Scope Enum variants
    pub fn scope_iter<'set>(&'set self) -> impl Iterator<Item = Scope> + 'set {
        ScopeIter {
            cursor: 0,
            set: &self,
        }
    }
}

struct SpecIter<'set>(ScopeIter<'set>);

impl<'set> Iterator for SpecIter<'set> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(Scope::as_twitch_str)
    }
}

impl<'a> std::iter::FromIterator<&'a str> for ScopeSet {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut scope_set = ScopeSet::new();

        for maybe_scope in iter.into_iter() {
            if let Some(scope) = Scope::from_twitch_str(maybe_scope) {
                scope_set.insert(scope);
            }
        }

        scope_set
    }
}

struct ScopeIter<'set> {
    cursor: usize,
    set: &'set ScopeSet,
}

impl<'set> Iterator for ScopeIter<'set> {
    type Item = Scope;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < Scope::max() {
            // SAFETY:
            // This is safe because we know that self.cursor will never be >= Scope::max()
            // which represents the largest usize that is a valid usize pattern that
            // can be transumeted into a Scope variant
            let current_scope: Scope = unsafe { std::mem::transmute(self.cursor) };

            if self.set.contains(current_scope) {
                self.cursor += 1;
                return Some(current_scope);
            } else {
                self.cursor += 1;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn from_list() {
        let list = vec!["user:edit", "channel:read:editors"];

        let scopes: ScopeSet = list.into_iter().collect();

        assert!(
            scopes.contains(Scope::UserEdit),
            "user:edit scope not set correctly"
        );

        assert!(
            scopes.contains(Scope::ChannelReadEditors),
            "user:edit scope not set correctly"
        );
    }

    #[test]
    fn iter_identity() {
        let list = vec!["user:edit", "channel:read:editors"];

        let scopes: ScopeSet = list.into_iter().collect();

        let mut scopes_iter = scopes.spec_iter();

        assert_eq!(
            Some("channel:read:editors"),
            scopes_iter.next(),
            "channel:read:editors scope not returned, possibly out of order"
        );
        assert_eq!(
            Some("user:edit"),
            scopes_iter.next(),
            "user:edit scope not returned or returned out of order"
        );
        assert_eq!(None, scopes_iter.next(), "Iter did not drain correctly");
    }

    #[test]
    fn piecewise() {
        let mut scopes = ScopeSet::new();

        scopes.insert(Scope::UserEdit);
        scopes.insert(Scope::ChannelReadEditors);

        let list: Vec<&'static str> = scopes.spec_iter().collect();

        assert!(list.contains(&"user:edit"), "Did not set user:edits");
        assert!(
            list.contains(&"channel:read:editors"),
            "Did not set channel:read:editors"
        );
    }

    #[test]
    fn removes() {
        let mut scopes = ScopeSet::new();

        scopes.insert(Scope::UserEdit);
        scopes.insert(Scope::ChannelReadEditors);

        scopes.remove(Scope::UserEdit);

        assert!(
            !scopes.contains(Scope::UserEdit),
            "User Edit was not removed correctly"
        );
        assert!(
            scopes.contains(Scope::ChannelReadEditors),
            "Removed too many scopes"
        );
    }
}
