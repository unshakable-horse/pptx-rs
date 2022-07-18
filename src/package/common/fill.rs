use serde::Deserialize;
use serde::Serialize;
use crate::package::common::color::SchemeClr;

#[derive(Deserialize, Serialize, Debug)]
pub struct SolidFill {
    #[serde(rename(serialize = "a:schemeClr", deserialize = "schemeClr"))]
    scheme_clr: SchemeClr,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct GradFill {
    #[serde(rename="rotWithShape")]
    rot_with_shape:String,

    #[serde(rename(serialize = "a:gsLst", deserialize = "gsLst"))]
    gs_lst:GsLst,

    #[serde(rename(serialize = "a:lin", deserialize = "lin"))]
    lin:Lin
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GsLst{
    #[serde(rename(serialize = "a:gs", deserialize = "gs"))]
    list:Vec<Gs>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Gs{
    pos:String,
    #[serde(rename(serialize = "a:schemeClr", deserialize = "schemeClr"))]
    scheme_clr: SchemeClr,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Lin{
    ang:String,
    scaled:String
}