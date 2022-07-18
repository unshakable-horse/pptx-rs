use serde::Serialize;
use serde::Deserialize;


use crate::package::ppt::_rels::RelationShip;

pub const PRESENTATION_RELS_XML_FILE_NAME: &'static str = "ppt/_rels/presentation.xml.rels";


#[derive(Deserialize, Serialize, Debug)]
pub struct PresentationXmlRels{
    xmlns:String,

    #[serde(rename="Relationship")]
    list: Vec<RelationShip>,
}