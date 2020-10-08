use der_parser::{oid, oid::Oid};
use std::collections::HashMap;

mod load;

pub use load::*;

pub type Nid = u64;

pub struct OidEntry {
    /// Short name
    pub sn: &'static str,
    pub description: &'static str,
}

impl OidEntry {
    pub fn new(sn: &'static str, description: &'static str) -> OidEntry {
        OidEntry { sn, description }
    }
}

pub struct OidRegistry {
    map: HashMap<Oid<'static>, OidEntry>,
}

impl Default for OidRegistry {
    fn default() -> Self {
        OidRegistry { map: HashMap::new() }
    }
}

impl OidRegistry {
    pub fn insert(&mut self, oid: Oid<'static>, entry: OidEntry) -> Option<OidEntry> {
        self.map.insert(oid, entry)
    }

    #[cfg(feature = "crypto")]
    pub fn with_crypto(self) -> Self {
        self.with_rsadsi().with_nist_hash()
    }

    #[cfg(feature = "crypto")]
    pub fn with_all_crypto(self) -> Self {
        self.with_rsadsi().with_nist_hash().with_pkcs7().with_pkcs9()
    }
}

pub fn format_oid(oid: &Oid, registry: &OidRegistry) -> String {
    if let Some(entry) = registry.map.get(oid) {
        format!("{} ({})", entry.sn, oid)
    } else {
        format!("{}", oid)
    }
}

include!(concat!(env!("OUT_DIR"), "/oid_db.rs"));
