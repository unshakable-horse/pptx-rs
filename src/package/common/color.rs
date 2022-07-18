use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct SchemeClr {
    val: String,

    #[serde(rename(serialize="a:lumMod",deserialize="lumMod"))]
    lum_mod:Option<LumMod>,

    #[serde(rename(serialize="a:satMod",deserialize="satMod"))]
    sat_mod:Option<SatMod>,

    #[serde(rename(serialize="a:tint",deserialize="tint"))]
    tint:Option<Tint>,

}

#[derive(Deserialize, Serialize, Debug)]
pub struct LumMod{
    val:String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SatMod{
    val:String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tint{
    val:String
}