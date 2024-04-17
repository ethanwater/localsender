use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Packet {
    pub command: String,
    pub filename: String,
    pub data: Vec<u8>,
    pub size: usize,
}


//TODO: do we async this?
impl Serialize for Packet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Packet", 4)?;
        state.serialize_field("command", &self.command)?;
        state.serialize_field("filename", &self.filename)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("size", &self.size)?;
        state.end()
    }
}
