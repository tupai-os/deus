use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Char(pub u32);
