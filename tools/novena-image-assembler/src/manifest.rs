use crate::{require, Result};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt, path::Path};

pub const REVISION: &str = "bd5c7d3290bb94bf8af17760c321df67d2641343";
pub const MAX_MANIFEST: u64 = 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Manifest {
    pub schema_version: u32,
    pub profile: String,
    pub sector_size: u32,
    pub source_revision: String,
    pub inputs: Inputs,
    pub layout: Layout,
    pub boot_contract: BootContract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub path: String,
    pub origin: String,
    pub size: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Inputs {
    pub spl: Artifact,
    pub proper: Artifact,
    pub firmware: Artifact,
    pub swap: Artifact,
    pub rootfs: Artifact,
    pub kernel: Artifact,
    pub dtb: Artifact,
}

impl Inputs {
    pub fn entries(&self) -> [(&'static str, &Artifact); 7] {
        [
            ("spl", &self.spl),
            ("proper", &self.proper),
            ("firmware", &self.firmware),
            ("swap", &self.swap),
            ("rootfs", &self.rootfs),
            ("kernel", &self.kernel),
            ("dtb", &self.dtb),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Layout {
    pub disk_id: String,
    pub firmware_start: u64,
    pub swap_start: u64,
    pub root_start: u64,
    pub firmware_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BootContract {
    pub pair_provenance: String,
    pub rom_spl_proper_contract: String,
    pub external_mmc_environment: String,
    pub eeprom_and_hooks: String,
    pub boot_files_and_arguments: String,
    pub kernel_dtb_initrd_contract: String,
    pub external_root_and_fstab: String,
    pub partition_identity_and_collisions: String,
}

impl Manifest {
    pub fn validate(&self) -> Result<()> {
        require(self.schema_version == 1, "schemaVersion must be 1")?;
        require(self.profile == "novena-debian-2015", "unsupported profile")?;
        require(self.sector_size == 512, "sectorSize must be 512")?;
        require(
            self.source_revision == REVISION,
            "2015 sourceRevision required",
        )?;
        for (_, a) in self.inputs.entries() {
            require(
                Path::new(&a.path).is_absolute() && !a.path.contains('\0'),
                "absolute input path without NUL required",
            )?;
            require(!a.origin.trim().is_empty(), "nonempty origin required")?;
            require(a.size > 0, "positive input size required")?;
            require(
                a.sha256.len() == 64
                    && a.sha256
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
                "lowercase SHA256 required",
            )?;
        }
        let b = &self.boot_contract;
        for evidence in [
            &b.pair_provenance,
            &b.rom_spl_proper_contract,
            &b.external_mmc_environment,
            &b.eeprom_and_hooks,
            &b.boot_files_and_arguments,
            &b.kernel_dtb_initrd_contract,
            &b.external_root_and_fstab,
            &b.partition_identity_and_collisions,
        ] {
            require(
                !evidence.trim().is_empty(),
                "all eight boot evidence references required",
            )?;
        }
        Ok(())
    }
}

// Parse into a duplicate-aware tree FIRST: serde_json::Value alone loses duplicate
// keys, and a derived struct alone does not cover arbitrary nested JSON objects.
struct Unique(serde_json::Value);
impl<'de> Deserialize<'de> for Unique {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Unique;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("unique-key JSON")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                mut map: M,
            ) -> std::result::Result<Unique, M::Error> {
                let mut values = BTreeMap::new();
                while let Some(key) = map.next_key::<String>()? {
                    if values.contains_key(&key) {
                        return Err(serde::de::Error::custom(format!("duplicate key: {key}")));
                    }
                    values.insert(key, map.next_value::<Unique>()?.0);
                }
                Ok(Unique(serde_json::Value::Object(
                    values.into_iter().collect(),
                )))
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> std::result::Result<Unique, A::Error> {
                let mut values = Vec::new();
                while let Some(v) = seq.next_element::<Unique>()? {
                    values.push(v.0);
                }
                Ok(Unique(serde_json::Value::Array(values)))
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_string<E: serde::de::Error>(
                self,
                v: String,
            ) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            // This schema contains no floating-point numbers, including 1.0.
            fn visit_f64<E: serde::de::Error>(self, _: f64) -> std::result::Result<Unique, E> {
                Err(E::custom("floating-point numbers forbidden"))
            }
            fn visit_unit<E: serde::de::Error>(self) -> std::result::Result<Unique, E> {
                Ok(Unique(serde_json::Value::Null))
            }
        }
        d.deserialize_any(Visitor)
    }
}

pub fn parse(bytes: &[u8]) -> Result<Manifest> {
    require(
        u64::try_from(bytes.len())? <= MAX_MANIFEST,
        "manifest exceeds 1 MiB",
    )?;
    let tree: Unique = serde_json::from_slice(bytes)?;
    let manifest: Manifest = serde_json::from_value(tree.0)?;
    manifest.validate()?;
    crate::layout::plan(&manifest)?;
    Ok(manifest)
}
