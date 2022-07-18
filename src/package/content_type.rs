use serde::{Serialize,Deserialize};

pub const CONTENT_TYPE_FILE_NAME:&'static str="[Content_Types].xml";

#[derive(Serialize,Deserialize)]
#[serde(rename="Types")]
pub struct ContentType {
    pub xmlns:String,

    #[serde(rename="Default")]
    pub defaults:Vec<Default>,

    #[serde(rename="Override")]
    pub overrides:Vec<Override>,
}

#[derive(Serialize,Deserialize)]
pub struct Default{
    #[serde(rename="ContentType")]
    pub content_type:String,

    #[serde(rename="Extension")]
    pub extension:String
}

#[derive(Serialize,Deserialize)]
pub struct Override{
    #[serde(rename="ContentType")]
    pub content_type:String,

    #[serde(rename="PartName")]
    pub part_name:String
}
