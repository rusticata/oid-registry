//! # OID Registry
//!
//! This crate is a helper crate, containing a database of OID objects. These objets are intended
//! for use when manipulating ASN.1 grammars and BER/DER encodings, for example.
//!
//! This crate provides only a simple registry (similar to a `HashMap`) by default. This object can
//! be used to get names and descriptions from OID.
//!
//! By default, the registry is provided emty.
//! This crate can provide default lists of known OIDs, that can be selected using the build
//! features.

#![deny(/*missing_docs,*/
          unstable_features,
          unused_import_braces,
          unused_qualifications,
          unreachable_pub)]
#![forbid(unsafe_code)]
#![warn(
      /* missing_docs,
      rust_2018_idioms,*/
      missing_debug_implementations,
  )]
// pragmas for doc
// #![deny(intra_doc_link_resolution_failure)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use der_parser::{oid, oid::Oid};
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::From;

mod load;

pub use load::*;

/// An entry stored in the OID registry
#[derive(Debug)]
pub struct OidEntry {
    // Short name
    sn: Cow<'static, str>,
    description: Cow<'static, str>,
}

impl OidEntry {
    /// Create a new entry
    pub fn new<S, T>(sn: S, description: T) -> OidEntry
    where
        S: Into<Cow<'static, str>>,
        T: Into<Cow<'static, str>>,
    {
        let sn = sn.into();
        let description = description.into();
        OidEntry { sn, description }
    }

    #[inline]
    pub fn sn(&self) -> &str {
        &self.sn
    }

    #[inline]
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl From<(&'static str, &'static str)> for OidEntry {
    fn from(t: (&'static str, &'static str)) -> Self {
        Self::new(t.0, t.1)
    }
}

/// Registry of known OIDs
///
/// Use `OidRegistry::default()` to create an empty registry. If the corresponding features have
/// been selected, the `with_xxx()` methods can be used to add sets of known objets to the
/// database.
///
/// # Example
///
/// ```rust
/// use der_parser::{oid, oid::Oid};
/// use oid_registry::{OidEntry, OidRegistry};
///
/// let mut registry = OidRegistry::default()
/// # ;
/// # #[cfg(feature = "crypto")] {
/// #     registry = registry
///     .with_crypto() // only if the 'crypto' feature is enabled
/// # }
/// ;
///
/// // entries can be added by creating an OidEntry object:
/// let entry = OidEntry::new("shortName", "description");
/// registry.insert(oid!(1.2.3.4), entry);
///
/// // when using static strings, a tuple can also be used directly for the entry:
/// registry.insert(oid!(1.2.3.5), ("shortName", "A description"));
///
/// // To query an entry, use the `get` method:
/// const OID_1234: Oid<'static> = oid!(1.2.3.4);
/// let e = registry.get(&OID_1234);
/// assert!(e.is_some());
/// if let Some(e) = e {
///     assert_eq!(e.sn(), "shortName");
/// }
/// ```
#[derive(Debug, Default)]
pub struct OidRegistry {
    map: HashMap<Oid<'static>, OidEntry>,
}

impl OidRegistry {
    /// Insert a new entry
    pub fn insert<E>(&mut self, oid: Oid<'static>, entry: E) -> Option<OidEntry>
    where
        E: Into<OidEntry>,
    {
        self.map.insert(oid, entry.into())
    }

    /// Returns a reference to the registry entry, if found for this OID.
    pub fn get(&self, oid: &Oid<'static>) -> Option<&OidEntry> {
        self.map.get(oid)
    }

    /// Return an Iterator over references to the OID numbers (registry keys)
    pub fn keys(&self) -> impl Iterator<Item = &Oid<'static>> {
        self.map.keys()
    }

    /// Return an Iterator over references to the `OidEntry` values
    pub fn values(&self) -> impl Iterator<Item = &OidEntry> {
        self.map.values()
    }

    /// Return an Iterator over references to the `(Oid, OidEntry)` key/value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&Oid<'static>, &OidEntry)> {
        self.map.iter()
    }

    /// Populate registry with common crypto OIDs (encryption, hash algorithms)
    #[cfg(feature = "crypto")]
    pub fn with_crypto(self) -> Self {
        self.with_rsadsi().with_nist_hash()
    }

    /// Populate registry with all known crypto OIDs (encryption, hash algorithms, PKCS constants,
    /// etc.)
    #[cfg(feature = "crypto")]
    pub fn with_all_crypto(self) -> Self {
        self.with_rsadsi().with_nist_hash().with_pkcs7().with_pkcs9()
    }
}

/// Format a OID to a `String`, using the provided registry to get the short name if present.
pub fn format_oid(oid: &Oid, registry: &OidRegistry) -> String {
    if let Some(entry) = registry.map.get(oid) {
        format!("{} ({})", entry.sn, oid)
    } else {
        format!("{}", oid)
    }
}

include!(concat!(env!("OUT_DIR"), "/oid_db.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    // This test is mostly a compile test, to ensure the API has not changed
    #[test]
    fn test_lifetimes() {
        fn add_entry(input: &str, oid: Oid<'static>, registry: &mut OidRegistry) {
            // test insertion of owned string
            let s = String::from(input);
            let entry = OidEntry::new("test", s);
            registry.insert(oid, entry);
        }

        let mut registry = OidRegistry::default();
        add_entry("a", oid!(1.2.3.4), &mut registry);
        add_entry("b", oid!(1.2.3.5), &mut registry);

        // test insertion of owned data
        let e = OidEntry::new("c", "test_c");
        registry.insert(oid!(1.2.4.1), e);

        registry.insert(oid!(1.2.5.1), ("a", "b"));

        // dbg!(&registry);
    }
}
