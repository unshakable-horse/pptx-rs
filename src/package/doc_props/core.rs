use serde::Deserialize;
use serde::Serialize;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;


pub const CORE_FILE_NAME: &'static str = "docProps/core.xml";


#[derive(Debug,Deserialize,Serialize)]
#[serde(rename = "cp:coreProperties")]
pub struct Core {
    #[serde(rename = "xmlns:cp")]
    pub cp: String,

    #[serde(rename = "xmlns:dc")]
    pub dc: String,

    #[serde(rename = "xmlns:dcterms")]
    pub dcterms: String,

    #[serde(rename = "xmlns:xsi")]
    pub xsi: String,

    #[serde(rename(serialize = "dcterms:created"))]
    pub created: DcTerms,

    #[serde(rename = "$unflatten=dc:creator")]
    pub creator: String,

    #[serde(rename = "$unflatten=cp:lastModifiedBy")]
    pub last_modified_by: String,

    #[serde(rename(serialize = "dcterms:modified"))]
    pub modified:DcTerms,

    #[serde(rename = "$unflatten=cp:revision")]
    pub revision: String,

    #[serde(rename = "$unflatten=dc:title")]
    pub title: String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct DcTerms {
    #[serde(rename = "xsi:type")]
    pub created_type: String,

    #[serde(rename = "$value")]
    body: String,
}




