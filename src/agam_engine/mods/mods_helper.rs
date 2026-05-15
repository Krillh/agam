
use std::path::PathBuf;

use regex::Regex;
use serde::Deserialize;
use serde_with::DeserializeFromStr;
use tinystr::TinyAsciiStr;


/// A unique mod id that includes its string id and its version
#[derive(Debug, PartialEq, Eq, Hash, DeserializeFromStr)]
pub struct ModId (TinyAsciiStr<16>, Version);
impl std::fmt::Display for ModId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}^{}", self.0, self.1)
    }
}
impl std::str::FromStr for ModId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version([u16; 4]);
impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0[3] != other.0[3] { return self.0[3].partial_cmp(&other.0[3]); }
        if self.0[2] != other.0[2] { return self.0[2].partial_cmp(&other.0[2]); }
        if self.0[1] != other.0[1] { return self.0[1].partial_cmp(&other.0[1]); }
        if self.0[1] != other.0[1] { return self.0[0].partial_cmp(&other.0[0]); }
        return Some(std::cmp::Ordering::Equal);
    }
}
impl TryFrom<&str> for Version {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r#"^(\d+)\.(\d+)\.(\d+)(?:\:pre-(\d+))?$"#).unwrap();
        let values = re.captures(value).ok_or("Version parsing failed: regex did not capture anything")?;
        let v0: u16 = values[1].parse().or(Err(format!("Version parsing failes: major version `{}` is not a u16", &values[1])))?;
        let v1: u16 = values[2].parse().or(Err(format!("Version parsing failes: minor version `{}` is not a u16", &values[2])))?;
        let v2: u16 = values[3].parse().or(Err(format!("Version parsing failes: patch version `{}` is not a u16", &values[3])))?;
        let v3: u16 = if let Some(pre_version) = values.get(4) {
            pre_version.as_str().parse::<u16>().or(Err(format!("Version parsing failes: pre-  version number `{}` is not a u16", &values[4])))?
        } else { 0 };

        Ok(Version([v0,v1,v2,v3]))

    }
}
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0[3] == 0 {
            write!(f, "{}.{}.{}", self.0[0], self.0[1], self.0[2])
        } else {
            write!(f, "{}.{}.{}:pre-{}", self.0[0], self.0[1], self.0[2], self.0[3])
        }
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct ModMetadata {
    pub name: String,
    pub string_id: String,
    pub version: String,
    #[serde(skip)]
    pub path: PathBuf,
}
impl ModMetadata {
    pub fn get_mod_id(&self) -> ModId {
        ModId (
            TinyAsciiStr::try_from_str(&self.string_id).unwrap(),
            Version::try_from(&self.version[..]).unwrap()
        )
    }
    pub fn with_path(self, path: PathBuf) -> Self {
        let mut s = self;
        s.path = path;
        return s;
    }
    pub fn is_valid(&self) -> bool {
        Version::try_from(&self.version[..]).is_ok()
    }
}
