#![allow(unused)]
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use std::fs::{FileType, Metadata};

#[derive(Debug, Deserialize)]
pub struct Object {
    pub filename: String,
    pub data: Vec<u8>,
    pub size: usize,
}

impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Object", 3)?;
        state.serialize_field("filename", &self.filename)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("size", &self.size)?;
        state.end()
    }
}
