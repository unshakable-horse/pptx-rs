use serde::Deserialize;
use serde::Serialize;


pub const TABLE_STYLES_XML_FILE_NAME: &'static str = "ppt/tableStyles.xml";


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "a:tblStyleLst")]
pub struct TableStyles{
    #[serde(rename = "xmlns:a")]
    a: Option<String>,

    def:Option<String>
}
