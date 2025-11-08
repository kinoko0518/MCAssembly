use crate::types::*;
use regex::Regex;

pub enum StorageType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
}

impl std::fmt::Display for StorageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Byte => "byte",
                Self::Short => "short",
                Self::Int => "int",
                Self::Long => "long",
                Self::Float => "float",
                Self::Double => "double",
            }
        )
    }
}

impl StorageType {
    fn try_from(from: &str) -> Option<Self> {
        match from {
            "byte" => Some(Self::Byte),
            "short" => Some(Self::Short),
            "int" => Some(Self::Int),
            "long" => Some(Self::Long),
            "float" => Some(Self::Float),
            "double" => Some(Self::Double),
            _ => None,
        }
    }
}

pub struct Storage {
    pub namespace: String,
    pub name: String,
}

impl Storage {
    pub fn try_from(from: &str) -> Result<Self, MCAsmError> {
        let re = Regex::new(r"([a-z0-9\_\-\.]+):([a-z0-9\_\-\.]+)").unwrap();
        re.captures(from)
            .and_then(|caps| {
                if let (Some(namespace), Some(name)) = (caps.get(1), caps.get(2)) {
                    Some(Self {
                        namespace: namespace.as_str().to_string(),
                        name: name.as_str().to_string(),
                    })
                } else {
                    None
                }
            })
            .ok_or(MCAsmError::InvalidStorage)
    }
    pub fn fullname(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
    pub fn store_to_score(
        &self,
        scoreboard: &Scoreboard,
        path: &String,
        magnification: u32,
    ) -> String {
        format!(
            "execute store result score {} {} run data get {} {} {}",
            scoreboard.scoreholder,
            scoreboard.objective,
            self.fullname(),
            path,
            magnification
        )
    }
}

pub struct Path {
    pub path: String,
    pub type_annotation: StorageType,
}

impl Path {
    pub fn try_from(from: &str) -> Result<Self, MCAsmError> {
        if let Some((path, type_annotation)) = from.rsplit_once("::<") {
            Ok(Self {
                path: path.to_string(),
                type_annotation: {
                    let trimmed = type_annotation.trim_end_matches(">");
                    StorageType::try_from(trimmed).ok_or(MCAsmError::UnknownType)?
                },
            })
        } else {
            Ok(Self {
                path: from.to_string(),
                type_annotation: StorageType::Int,
            })
        }
    }
}
