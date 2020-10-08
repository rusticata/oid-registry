use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write};
use std::path::Path;

pub struct Entry {
    name: String,
    oid: String,
    sn: String,
    description: String,
}

pub type LoadedMap = HashMap<String, Vec<Entry>>;

pub fn load_file<P: AsRef<Path>>(path: P) -> Result<LoadedMap> {
    let mut map = HashMap::new();

    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // split by tabs
        let mut iter = line.splitn(5, '\t');
        let feature = iter.next().expect("invalid oid_db format: missing feature").replace("-", "_");
        let name = iter.next().expect("invalid oid_db format: missing name").to_string();
        let oid = iter.next().expect("invalid oid_db format: missing OID").to_string();
        let sn = iter.next().expect("invalid oid_db format: missing short name").to_string();
        let description = iter.next().expect("invalid oid_db format: missing description").to_string();

        let entry = Entry {
            name,
            oid,
            sn,
            description,
        };

        let v = map.entry(feature.to_string()).or_insert_with(Vec::new);

        v.push(entry);
    }
    Ok(map)
}

pub fn generate_file<P: AsRef<Path>>(map: &LoadedMap, dest_path: P) -> Result<()> {
    let mut out_file = File::create(&dest_path)?;
    for feat_entries in map.values() {
        for v in feat_entries {
            if v.name != "\"\"" {
                writeln!(out_file, "pub const {}: Oid<'static> = oid!({});", v.name, v.oid)?;
            }
        }
    }
    writeln!(out_file)?;
    writeln!(out_file, "impl OidRegistry {{")?;
    for (k, v) in map {
        writeln!(out_file, r#"    #[cfg(feature = "{}")]"#, k)?;
        writeln!(out_file, "    pub fn with_{}(mut self) -> Self {{", k)?;
        for item in v {
            writeln!(
                out_file,
                r#"        self.insert(oid!({}), OidEntry::new("{}", "{}"));"#,
                item.oid, item.sn, item.description
            )?;
        }
        writeln!(out_file, "        self")?;
        writeln!(out_file, "    }}")?;
        writeln!(out_file)?;
    }
    writeln!(out_file, "}}")?;
    Ok(())
}
