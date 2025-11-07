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

pub struct Storage {
    pub key: String,
    pub datatype: StorageType,
}
