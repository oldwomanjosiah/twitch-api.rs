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

#[derive(Debug, Clone)]
/// Represents a set of scopes available with a specific bearer auth key
pub struct ScopeSet {
    scopes: u32,
}

impl ScopeSet {
    /// Create a new empty set of scopes
    pub fn new() -> Self {
        Self { scopes: 0 }
    }

    /// Get whether this type contains a single scope
    pub fn get(&self, bit: usize) -> bool {
        ((1 << bit) & self.scopes) > 0
    }

    /// Set a bit for a single scope, returns the old value
    pub fn set(&mut self, bit: usize, val: bool) -> bool {
        let old = self.get(bit);

        let mask = (val as usize) << bit;

        if val {
            self.scopes |= mask as u32;
        } else {
            self.scopes &= !mask as u32;
        }

        old
    }

    /// Returns true if the set contains the scope specified  
    /// Returns false if it does not or the scope does not exist
    ///
    /// ```
    /// # use twitch_api_rs::auth::scopes::*;
    /// let mut scopes = ScopeSet::new();
    ///
    /// scopes.set(2, true);
    ///
    /// assert!(scopes.contains("bits:read"));
    ///
    /// assert!(!scopes.contains("channel:edit:commercial"));
    /// ```
    pub fn contains(&self, scope: &str) -> bool {
        if let Some(&scope) = Self::ident_from(scope) {
            self.get(scope)
        } else {
            false
        }
    }

    const fn count() -> usize {
        26
    }

    /// Get an iterator over scopes idents iterator
    pub fn iter<'a>(&'a self) -> ScopesIter<'a> {
        self.into_iter()
    }

    ident!(
        #[doc="Identity function for converting between scope internal number and twitch token string"]
        pub <usize, str>: [
            // General scopes
            00 => "analytics:read:extensions",
            01 => "analytics:read:games",

            02 => "bits:read",

            03 => "channel:edit:commercial",
            04 => "channel:manage:broadcast",
            05 => "channel:manage:etensions",
            06 => "channel:manage:redemptions",
            07 => "channel:manage:videos",

            08 => "channel:read:editors",
            09 => "channel:read:hype_train",
            10 => "channel:read:redemptions",
            11 => "channel:read:stream_key",
            12 => "channel:read:subscriptions",

            13 => "clips:edit",

            14 => "moderation:read",

            15 => "user:edit",
            16 => "user:edit:follows",
            17 => "user:read:broadcast",
            18 => "user:read:email",
            19 => "user:read:blocked_users",

            20 => "user:mange:blocked_users",

            // The following scopes are for for chat and PubSub
            21 => "channel:moderate",

            22 => "chat:edit",
            23 => "chat:read",

            24 => "whispers:read",
            25 => "whispers:edit"
    ]);
}

impl<'a> IntoIterator for &'a ScopeSet {
    type Item = &'static str;
    type IntoIter = ScopesIter<'a>;

    fn into_iter(self) -> ScopesIter<'a> {
        ScopesIter {
            scopes: self,
            cursor: 0,
        }
    }
}

impl<'a> std::iter::FromIterator<&'a str> for ScopeSet {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut scopes = ScopeSet { scopes: 0 };

        for maybe_scope in iter.into_iter() {
            if let Some(&scope) = ScopeSet::ident_from(maybe_scope) {
                scopes.set(scope, true);
            }
        }

        scopes
    }
}

#[derive(Debug)]
/// Iterator type for scopes
pub struct ScopesIter<'a> {
    scopes: &'a ScopeSet,
    cursor: u32,
}

impl<'a> Iterator for ScopesIter<'a> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < ScopeSet::count() as u32 {
            if self.scopes.get(self.cursor as usize) {
                let val: &'static str = ScopeSet::ident_to(&(self.cursor as usize))
                    .expect("Tried to iterate over undefined value");
                self.cursor += 1;
                return Some(val);
            } else {
                self.cursor += 1;
                continue;
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
            scopes.scopes & (1 << 15) as u32 > 0,
            "user:edit scope not set correctly"
        );

        assert!(
            scopes.scopes & (1 << 08) as u32 > 0,
            "user:edit scope not set correctly"
        );
    }

    #[test]
    fn iter_identity() {
        let list = vec!["user:edit", "channel:read:editors"];

        let scopes: ScopeSet = list.into_iter().collect();

        let mut scopes_iter = scopes.iter();

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
    fn from_bits() {
        let mut scopes = ScopeSet::new();

        scopes.set(8, true);
        scopes.set(15, true);

        let list: Vec<&'static str> = scopes.iter().collect();

        assert!(list.contains(&"user:edit"), "Did not set user:edits");
        assert!(
            list.contains(&"channel:read:editors"),
            "Did not set channel:read:editors"
        );
    }
}
