use crate::types::*;

pub enum StorageType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    List,
    Compound,
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
                Self::String => "string",
                Self::List => "list",
                Self::Compound => "compound",
            }
        )
    }
}

pub struct Storage {
    pub namespace: String,
    pub name: String,
    pub datatype: StorageType,
}

impl Storage {
    pub fn fullname(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
    pub fn store_to_score(
        &self,
        scoreboard: &Scoreboard,
        path: &String,
        magnification: u32,
    ) -> Result<String, MCAsmError> {
        match self.datatype {
            StorageType::Byte
            | StorageType::Short
            | StorageType::Int
            | StorageType::Long
            | StorageType::Float
            | StorageType::Double => Ok(format!(
                "execute store result score {} {} run data get {} {} {}",
                scoreboard.scoreholder,
                scoreboard.objective,
                self.fullname(),
                path,
                magnification
            )),
            _ => Err(MCAsmError::InvalidAssignment),
        }
    }
}
