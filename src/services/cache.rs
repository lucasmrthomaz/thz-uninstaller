use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::constants::{CACHE_ITEM_STRUCT_SIZE, CACHE_UNKNOWN_SIZE, CACHE_VERSION};
use crate::models::CachedEntry;
use crate::utils::formatting::extract_xml_attr;

/// Loads `%APPDATA%\Geek Uninstaller\cache.dat` per `CUninstallCache::Load` (function 0x4250cd / 0x424f39).
pub fn load_uninstall_cache() -> HashMap<String, CachedEntry> {
    let _ver = (CACHE_VERSION, CACHE_ITEM_STRUCT_SIZE);
    let mut map = HashMap::new();
    let Ok(appdata) = env::var("APPDATA") else {
        return map;
    };
    let cache_path = Path::new(&appdata)
        .join("Geek Uninstaller")
        .join("cache.dat");
    let Ok(bytes) = fs::read(cache_path) else {
        return map;
    };
    if bytes.first().copied() != Some(0x1f) {
        return map;
    }
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut xml = String::new();
    if decoder.read_to_string(&mut xml).is_err() {
        return map;
    }

    for line in xml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<CacheItem ") {
            if let Some(key) = extract_xml_attr(trimmed, "key") {
                let location = extract_xml_attr(trimmed, "location").unwrap_or_default();
                let icon_path = extract_xml_attr(trimmed, "iconPath").unwrap_or_default();
                let raw_size = extract_xml_attr(trimmed, "size")
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(CACHE_UNKNOWN_SIZE);
                let size_kb = if raw_size == CACHE_UNKNOWN_SIZE {
                    0
                } else {
                    raw_size
                };
                map.insert(
                    key,
                    CachedEntry {
                        location: if location == "-" {
                            String::new()
                        } else {
                            location
                        },
                        icon_path: if icon_path == "-" {
                            String::new()
                        } else {
                            icon_path
                        },
                        size_kb,
                    },
                );
            }
        }
    }
    map
}

pub fn remove_cache_entry(key_to_remove: &str) -> Result<(), String> {
    let Ok(appdata) = env::var("APPDATA") else {
        return Ok(());
    };
    let cache_dir = Path::new(&appdata).join("Geek Uninstaller");
    let cache_path = cache_dir.join("cache.dat");
    if !cache_path.exists() {
        return Ok(());
    }
    let Ok(bytes) = fs::read(&cache_path) else {
        return Ok(());
    };
    if bytes.first().copied() != Some(0x1f) {
        return Ok(());
    }
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut xml = String::new();
    if decoder.read_to_string(&mut xml).is_err() {
        return Ok(());
    }

    let mut new_lines = Vec::new();
    let mut modified = false;
    for line in xml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<CacheItem ") {
            if let Some(key) = extract_xml_attr(trimmed, "key") {
                if key.eq_ignore_ascii_case(key_to_remove) {
                    modified = true;
                    continue;
                }
            }
        }
        new_lines.push(line);
    }

    if modified {
        let new_xml = new_lines.join("\r\n");
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        if encoder.write_all(new_xml.as_bytes()).is_ok() {
            if let Ok(compressed) = encoder.finish() {
                let _ = fs::write(&cache_path, compressed);
            }
        }
    }
    Ok(())
}
