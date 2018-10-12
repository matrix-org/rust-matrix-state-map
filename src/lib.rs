//! `StateMap` is a specialised container for efficiently storing Matrix state
//! maps.
//!
//! State maps are effectively maps from 2-tuple of `(type, state_key)` to an
//! event (or event ID). In most scenarios the majority of the types in a state
//! map are well known, so we can specialize storage to reduce memory usage
//! compared to naively storing a map of string tuples.

use std::borrow::Borrow;
use std::collections::{hash_map, HashMap};
use std::fmt::Debug;
use std::iter::FromIterator;

/// The creation event type - `m.room.create`
pub const TYPE_CREATE: &str = "m.room.create";
/// The power levels event type - `m.room.power_levels`
pub const TYPE_POWER_LEVELS: &str = "m.room.power_levels";
/// The join rules event type - `m.room.join_rules`
pub const TYPE_JOIN_RULES: &str = "m.room.join_rules";
/// The history visibility event type - `m.room.history_visibility`
pub const TYPE_HISTORY_VISIBILITY: &str = "m.room.history_visibility";
/// The name event type - `m.room.name`
pub const TYPE_NAME: &str = "m.room.name";
/// The topic event type - `m.room.topic`
pub const TYPE_TOPIC: &str = "m.room.topic";
/// The room avatar event type - `m.room.avatar`
pub const TYPE_AVATAR: &str = "m.room.avatar";
/// The guest_access event type - `m.room.guest_access`
pub const TYPE_GUEST_ACCESS: &str = "m.room.guest_access";
/// The canonical_alias event type - `m.room.canonical_alias`
pub const TYPE_CANONICAL_ALIASES: &str = "m.room.canonical_alias";
/// The related_groups event type - `m.room.related_groups`
pub const TYPE_RELATED_GROUPS: &str = "m.room.related_groups";
/// The encryption event type - `m.room.encryption`
pub const TYPE_ENCRYPTION: &str = "m.room.encryption";

/// The member event type - `m.room.member`
pub const TYPE_MEMBERSHIP: &str = "m.room.member";
/// The aliases event type - `m.room.aliases`
pub const TYPE_ALIASES: &str = "m.room.aliases";
/// The third_party_invite event type - `m.room.third_party_invite`
pub const TYPE_THIRD_PARTY_INVITE: &str = "m.room.third_party_invite";

/// List of event types that are commonly used for state with empty state
/// keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WellKnownEmptyKeys {
    /// The creation event type - `m.room.create`
    Create,
    /// The power levels event type - `m.room.power_levels`
    PowerLevels,
    /// The join rules event type - `m.room.join_rules`
    JoinRules,
    /// The history visibility event type - `m.room.history_visibility`
    HistoryVisibility,
    /// The name event type - `m.room.name`
    Name,
    /// The topic event type - `m.room.topic`
    Topic,
    /// The room avatar event type - `m.room.avatar`
    Avatar,
    /// The guest_access event type - `m.room.guest_access`
    GuestAccess,
    /// The canonical_alias event type - `m.room.canonical_alias`
    CanonicalAliases,
    /// The related_groups event type - `m.room.related_groups`
    RelatedGroups,
    /// The encryption event type - `m.room.encryption`
    Encryption,
}

impl WellKnownEmptyKeys {
    /// Gets the event type as a string
    pub fn as_str(self) -> &'static str {
        match self {
            WellKnownEmptyKeys::Create => TYPE_CREATE,
            WellKnownEmptyKeys::PowerLevels => TYPE_POWER_LEVELS,
            WellKnownEmptyKeys::JoinRules => TYPE_JOIN_RULES,
            WellKnownEmptyKeys::HistoryVisibility => TYPE_HISTORY_VISIBILITY,
            WellKnownEmptyKeys::Name => TYPE_NAME,
            WellKnownEmptyKeys::Topic => TYPE_TOPIC,
            WellKnownEmptyKeys::Avatar => TYPE_AVATAR,
            WellKnownEmptyKeys::GuestAccess => TYPE_GUEST_ACCESS,
            WellKnownEmptyKeys::CanonicalAliases => TYPE_CANONICAL_ALIASES,
            WellKnownEmptyKeys::RelatedGroups => TYPE_RELATED_GROUPS,
            WellKnownEmptyKeys::Encryption => TYPE_ENCRYPTION,
        }
    }

    /// Attempts to convert from a string event type
    pub fn from_str(t: &str) -> Option<WellKnownEmptyKeys> {
        match t {
            TYPE_CREATE => Some(WellKnownEmptyKeys::Create),
            TYPE_POWER_LEVELS => Some(WellKnownEmptyKeys::PowerLevels),
            TYPE_JOIN_RULES => Some(WellKnownEmptyKeys::JoinRules),
            TYPE_HISTORY_VISIBILITY => Some(WellKnownEmptyKeys::HistoryVisibility),
            TYPE_NAME => Some(WellKnownEmptyKeys::Name),
            TYPE_TOPIC => Some(WellKnownEmptyKeys::Topic),
            TYPE_AVATAR => Some(WellKnownEmptyKeys::Avatar),
            TYPE_GUEST_ACCESS => Some(WellKnownEmptyKeys::GuestAccess),
            TYPE_CANONICAL_ALIASES => Some(WellKnownEmptyKeys::CanonicalAliases),
            TYPE_RELATED_GROUPS => Some(WellKnownEmptyKeys::RelatedGroups),
            TYPE_ENCRYPTION => Some(WellKnownEmptyKeys::Encryption),
            _ => None,
        }
    }
}

/// A specialised container for storing state mapping.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StateMap<E: Debug + Clone> {
    well_known: HashMap<WellKnownEmptyKeys, E>,
    membership: HashMap<String, E>,
    aliases: HashMap<String, E>,
    invites: HashMap<String, E>,
    others: HashMap<String, HashMap<String, E>>,
}

impl<E> StateMap<E>
where
    E: Debug + Clone,
{
    pub fn new() -> StateMap<E> {
        StateMap {
            well_known: HashMap::new(),
            membership: HashMap::new(),
            aliases: HashMap::new(),
            invites: HashMap::new(),
            others: HashMap::new(),
        }
    }

    pub fn get_well_known(&self, key: WellKnownEmptyKeys) -> Option<&E> {
        self.well_known.get(&key)
    }

    pub fn get_aliases(&self, server: &str) -> Option<&E> {
        self.aliases.get(server)
    }

    pub fn get_membership(&self, user: &str) -> Option<&E> {
        self.membership.get(user)
    }

    pub fn get_third_party_invites(&self, token: &str) -> Option<&E> {
        self.invites.get(token)
    }

    pub fn get(&self, t: &str, s: &str) -> Option<&E> {
        if s == "" {
            if let Some(key) = WellKnownEmptyKeys::from_str(t) {
                return self.get_well_known(key);
            }
        }

        match (t.borrow(), s.borrow()) {
            (TYPE_MEMBERSHIP, user) => self.get_membership(user),
            (TYPE_ALIASES, server) => self.get_aliases(server),
            (TYPE_THIRD_PARTY_INVITE, token) => self.get_third_party_invites(token),

            (t, s) => self.others.get(t).and_then(|m| m.get(s)),
        }
    }

    pub fn get_mut(&mut self, t: &str, s: &str) -> Option<&mut E> {
        if s == "" {
            if let Some(key) = WellKnownEmptyKeys::from_str(t) {
                return self.well_known.get_mut(&key);
            }
        }

        match (t.borrow(), s.borrow()) {
            (TYPE_MEMBERSHIP, user) => self.membership.get_mut(user),
            (TYPE_ALIASES, server) => self.aliases.get_mut(server),
            (TYPE_THIRD_PARTY_INVITE, token) => self.invites.get_mut(token),

            (t, s) => self.others.get_mut(t).and_then(|m| m.get_mut(s)),
        }
    }

    pub fn insert_well_known(&mut self, k: WellKnownEmptyKeys, value: E) {
        self.well_known.insert(k, value);
    }

    pub fn insert(&mut self, t: &str, s: &str, value: E) {
        if s == "" {
            if let Some(key) = WellKnownEmptyKeys::from_str(t) {
                self.well_known.insert(key, value);
                return;
            }
        }

        match (t, s) {
            (TYPE_MEMBERSHIP, user) => self.membership.insert(user.into(), value),
            (TYPE_ALIASES, server) => self.aliases.insert(server.into(), value),
            (TYPE_THIRD_PARTY_INVITE, token) => self.invites.insert(token.into(), value),

            (t, s) => self
                .others
                .entry(t.into())
                .or_insert_with(HashMap::new)
                .insert(s.into(), value),
        };
    }

    pub fn contains_key(&self, t: &str, s: &str) -> bool {
        self.get(t, s).is_some()
    }

    /// Returns an iterator over all keys in the state map
    pub fn keys(&self) -> impl Iterator<Item = (&str, &str)> {
        let w = self.well_known.keys().map(|k| (k.as_str(), ""));

        let m = self.membership.keys().map(|u| (TYPE_MEMBERSHIP, u as &str));

        let a = self.aliases.keys().map(|s| (TYPE_ALIASES, s as &str));

        let i = self
            .invites
            .keys()
            .map(|t| (TYPE_THIRD_PARTY_INVITE, t as &str));

        let o = self
            .others
            .iter()
            .flat_map(|(t, h)| h.keys().map(move |s| (t as &str, s as &str)));

        w.chain(m).chain(a).chain(i).chain(o)
    }

    /// Returns an iterator over all keys and values in the state map
    pub fn iter(&self) -> impl Iterator<Item = ((&str, &str), &E)> {
        let w = self.well_known.iter().map(|(k, e)| ((k.as_str(), ""), e));

        let m = self
            .membership
            .iter()
            .map(|(u, e)| ((TYPE_MEMBERSHIP, u as &str), e));

        let a = self
            .aliases
            .iter()
            .map(|(s, e)| ((TYPE_ALIASES, s as &str), e));

        let i = self
            .invites
            .iter()
            .map(|(t, e)| ((TYPE_THIRD_PARTY_INVITE, t as &str), e));

        let o = self
            .others
            .iter()
            .flat_map(|(t, h)| h.iter().map(move |(s, e)| ((t as &str, s as &str), e)));

        w.chain(m).chain(a).chain(i).chain(o)
    }

    /// Returns an iterator over all values in the state map
    pub fn values(&self) -> impl Iterator<Item = &E> {
        let w = self.well_known.values();

        let m = self.membership.values();

        let a = self.aliases.values();

        let i = self.invites.values();

        let o = self.others.values().flat_map(|h| h.values());

        w.chain(m).chain(a).chain(i).chain(o)
    }

    /// Returns an iterator over all entries with a type of `m.room.member`,
    /// returning the state_key and value
    pub fn iter_members(&self) -> impl Iterator<Item = (&str, &E)> {
        self.membership.iter().map(|(u, e)| (u as &str, e))
    }

    /// Returns an iterator over all entries with a type of `m.room.join_rules`,
    /// returning the state_key and value.
    ///
    /// **Note**: This also returns entries whose state key is not empty. This
    /// is really only useful for v1 state resolution algorithms.
    pub fn iter_join_rules(&self) -> impl Iterator<Item = (&str, &E)> {
        let i = self
            .well_known
            .get(&WellKnownEmptyKeys::JoinRules)
            .into_iter()
            .map(|e| ("", e));

        let o = self
            .others
            .get(TYPE_JOIN_RULES)
            .into_iter()
            .flat_map(|h| h.iter().map(move |(s, e)| (s as &str, e)));

        i.chain(o)
    }

    /// Returns an iterator over all entries with a type not of `m.room.member`,
    /// returning the type/state_key and value.
    pub fn iter_non_members(&self) -> impl Iterator<Item = ((&str, &str), &E)> {
        let w = self.well_known.iter().map(|(k, e)| ((k.as_str(), ""), e));

        let a = self
            .aliases
            .iter()
            .map(|(s, e)| ((TYPE_ALIASES, s as &str), e));

        let i = self
            .invites
            .iter()
            .map(|(t, e)| ((TYPE_THIRD_PARTY_INVITE, t as &str), e));

        let o = self
            .others
            .iter()
            .flat_map(|(t, h)| h.iter().map(move |(s, e)| ((t as &str, s as &str), e)));

        w.chain(a).chain(i).chain(o)
    }

    pub fn len(&self) -> usize {
        let others: usize = self.others.values().map(|x| x.len()).sum();
        self.well_known.len()
            + self.membership.len()
            + self.aliases.len()
            + self.invites.len()
            + others
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<E> StateMap<E>
where
    E: Debug + Clone + Default,
{
    /// Gets a mutable reference to a value in the map, inserting a default
    /// value if the entry doesn't exist.
    pub fn get_mut_or_default(&mut self, t: &str, s: &str) -> &mut E {
        if s == "" {
            if let Some(key) = WellKnownEmptyKeys::from_str(t) {
                return self.well_known.entry(key).or_insert_with(E::default);
            }
        }

        if let Some(entry) = match (t, s) {
            (TYPE_MEMBERSHIP, user) => Some(self.membership.entry(user.into())),
            (TYPE_ALIASES, server) => Some(self.aliases.entry(server.into())),
            (TYPE_THIRD_PARTY_INVITE, token) => Some(self.invites.entry(token.into())),

            _ => None,
        } {
            entry.or_insert_with(E::default)
        } else {
            self.others
                .entry(t.into())
                .or_insert_with(HashMap::new)
                .entry(s.into())
                .or_insert_with(E::default)
        }
    }
}

impl<E> StateMap<E>
where
    E: Debug + Clone + PartialEq,
{
    /// Inserts the entry into the map if it doesn't conflict with an existing
    /// entry (either the key doesn't exist or the values are the same)
    /// returning None. If there is a conflict the existing entry is removed and
    /// returned.
    pub fn add_or_remove<F>(&mut self, t: &str, s: &str, v: F) -> Option<E>
    where
        F: Borrow<E>,
    {
        let value = v.borrow();
        if s == "" {
            if let Some(key) = WellKnownEmptyKeys::from_str(t) {
                match self.well_known.entry(key) {
                    hash_map::Entry::Occupied(o) => {
                        if o.get() != value {
                            return Some(o.remove());
                        } else {
                            return None;
                        }
                    }
                    hash_map::Entry::Vacant(v) => {
                        v.insert(value.clone());
                        return None;
                    }
                }
            }
        }

        if let Some(entry) = match (t, s) {
            (TYPE_MEMBERSHIP, user) => Some(self.membership.entry(user.into())),
            (TYPE_ALIASES, server) => Some(self.aliases.entry(server.into())),
            (TYPE_THIRD_PARTY_INVITE, token) => Some(self.invites.entry(token.into())),

            _ => None,
        } {
            match entry {
                hash_map::Entry::Occupied(o) => {
                    if o.get() != value {
                        return Some(o.remove());
                    } else {
                        return None;
                    }
                }
                hash_map::Entry::Vacant(v) => {
                    v.insert(value.clone());
                    return None;
                }
            }
        } else {
            match self.others.entry(t.into()) {
                hash_map::Entry::Occupied(mut o) => match o.get_mut().entry(s.into()) {
                    hash_map::Entry::Occupied(o) => {
                        if o.get() != value {
                            return Some(o.remove());
                        } else {
                            return None;
                        }
                    }
                    hash_map::Entry::Vacant(v) => {
                        v.insert(value.clone());
                        return None;
                    }
                },
                hash_map::Entry::Vacant(v) => {
                    v.insert(HashMap::new()).insert(s.into(), value.clone());
                    return None;
                }
            }
        }
    }
}

impl<E> FromIterator<((String, String), E)> for StateMap<E>
where
    E: Debug + Clone,
{
    fn from_iter<T: IntoIterator<Item = (((String, String), E))>>(iter: T) -> StateMap<E> {
        let mut state_map = StateMap::new();

        for ((t, s), e) in iter {
            state_map.insert(&t, &s, e);
        }

        state_map
    }
}

impl<'a, E> FromIterator<((&'a str, &'a str), E)> for StateMap<E>
where
    E: Debug + Clone,
{
    fn from_iter<T: IntoIterator<Item = (((&'a str, &'a str), E))>>(iter: T) -> StateMap<E> {
        let mut state_map = StateMap::new();

        for ((t, s), e) in iter {
            state_map.insert(&t, &s, e);
        }

        state_map
    }
}

impl<E> Extend<((String, String), E)> for StateMap<E>
where
    E: Debug + Clone,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = ((String, String), E)>,
    {
        for ((t, s), e) in iter {
            self.insert(&t, &s, e);
        }
    }
}

impl<'a, E> Extend<((&'a str, &'a str), E)> for StateMap<E>
where
    E: Debug + Clone,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = ((&'a str, &'a str), E)>,
    {
        for ((t, s), e) in iter {
            self.insert(t, s, e);
        }
    }
}

#[test]
fn add_or_remove_test() {
    let mut state_map = StateMap::new();

    for &(t, s) in &[
        ("test", "test2"),
        (TYPE_POWER_LEVELS, ""),
        (TYPE_POWER_LEVELS, "foo"),
        (TYPE_MEMBERSHIP, "foo"),
    ] {
        state_map.insert(t, s, 1);

        let res = state_map.add_or_remove(t, s, 2);
        assert_eq!(res, Some(1));

        assert_eq!(state_map.get(t, s), None);

        let res = state_map.add_or_remove(t, s, 1);
        assert_eq!(res, None);

        let res = state_map.add_or_remove(t, s, 1);
        assert_eq!(res, None);
    }
}

#[test]
fn iter_test() {
    let mut state_map = StateMap::new();

    let mut expected = HashMap::new();

    let mut val = 0;

    for &(t, s) in &[
        ("test", "test2"),
        (TYPE_POWER_LEVELS, ""),
        (TYPE_POWER_LEVELS, "foo"),
        (TYPE_MEMBERSHIP, "foo"),
    ] {
        state_map.insert(t, s, val);
        expected.insert((t, s), val);

        val += 1;
    }

    let actual_entries: HashMap<_, _> = state_map.iter().map(|(k, i)| (k, *i)).collect();

    assert_eq!(expected, actual_entries);
}
