use rand::RngExt;
use std::str::FromStr;
use std::fmt;
use serde::{Deserialize, Serialize};

const BASE32_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GroupId(pub String);

impl TaskId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn random() -> Self {  
        let mut rng = rand::rng();
        Self::new((0..6).map(|_| {
            let idx = rng.random_range(0..BASE32_ALPHABET.len());
            BASE32_ALPHABET[idx] as char
        }).collect())
    }
}
impl GroupId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn random() -> Self {  
        let mut rng = rand::rng();
        Self::new((0..6).map(|_| {
            let idx = rng.random_range(0..BASE32_ALPHABET.len());
            BASE32_ALPHABET[idx] as char
        }).collect())
    }
}
impl FromStr for TaskId {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
impl FromStr for GroupId {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for GroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl AsRef<str> for TaskId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl AsRef<str> for GroupId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

