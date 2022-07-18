use serde::Serialize;
use serde::Deserialize;


pub mod presentaion_xml_rels;



#[derive(Deserialize, Serialize, Debug)]
pub struct RelationShip {
    #[serde(rename = "Id")]
    id: String,

    #[serde(rename = "Target")]
    target: String,

    #[serde(rename = "Type")]
    r_type: String,
}
