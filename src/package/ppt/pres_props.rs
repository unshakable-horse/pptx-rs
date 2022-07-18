use serde::Deserialize;
use serde::Serialize;
use crate::package::ppt::ExtLst;


pub const PRESENTATION_PROPS_XML_FILE_NAME: &'static str = "ppt/presProps.xml";


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "p:presentationPr")]
pub struct PresentationProperties{
    #[serde(rename = "xmlns:a")]
    a: Option<String>,

    #[serde(rename = "xmlns:r")]
    r: Option<String>,

    #[serde(rename = "xmlns:p")]
    p: Option<String>,

    #[serde(rename(serialize="p:extLst",deserialize="extLst"))]
    ext_list:ExtLst

}
