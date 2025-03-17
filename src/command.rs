use std::fmt::Display;

use regex::Regex;
use walkdir::WalkDir;

use crate::{Mode, CARGO_REGISTRY};

#[derive(Clone)]
pub(crate) struct Crate {
    pub(crate) name: String,
    pub(crate) version: Version,
}

impl Crate {
    fn parse(file_name: String) -> Option<Crate> {
        let re = Regex::new(r"^(.*)-([\d\.]+)\.crate$").ok()?;
        let caps = re.captures(&file_name)?;
        Some(Crate {
            name: caps[1].to_string(),
            version: Version {
                v: caps[2].to_string(),
            },
        })
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, self.version)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Version {
    v: String,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.v)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.v.cmp(&self.v))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let s_v = self
            .v
            .split(".")
            .filter_map(|s| s.parse::<i16>().ok())
            .collect::<Vec<i16>>();
        let o_v = other
            .v
            .split(".")
            .filter_map(|s| s.parse::<i16>().ok())
            .collect::<Vec<i16>>();
        s_v.iter()
            .zip(o_v.iter())
            .find(|(s, o)| s != o)
            .map(|(s, o)| o.cmp(s))
            .expect("fail to compare version")
    }
}

pub(crate) fn list() -> Vec<Crate> {
    let walk_dir = WalkDir::new(CARGO_REGISTRY.as_str());
    walk_dir
        .max_depth(2)
        .into_iter()
        .filter_map(|r| {
            r.ok()
                .and_then(|d| d.file_name().to_str().map(|n| n.to_string()))
                .and_then(Crate::parse)
        })
        .collect()
}

pub(crate) fn search(name: String, mode: Mode) -> Vec<Crate> {
    let walk_dir = WalkDir::new(CARGO_REGISTRY.as_str());
    let mut crates = walk_dir
        .max_depth(2)
        .into_iter()
        .filter_map(|r| -> Option<Crate> {
            r.ok()
                .and_then(|d| d.file_name().to_str().map(|n| n.to_string()))
                .filter(|file_name| file_name.contains(&name))
                .and_then(Crate::parse)
        })
        .collect();
    match mode {
        Mode::All => crates,
        Mode::New => {
            crates.sort_by_key(|c| c.version.clone());
            crates.first().map_or(vec![], |c| vec![c.clone()])
        }
    }
}
