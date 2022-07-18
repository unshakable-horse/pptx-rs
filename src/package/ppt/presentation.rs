use serde::Serialize;
use serde::Deserialize;
use crate::package::common::fill::SolidFill;
use crate::package::common::font::{Cs, Ea, Latin};
use crate::package::ppt::ExtLst;


pub const PRESENTATION_XML_FILE_NAME: &'static str = "ppt/presentation.xml";

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "p:presentation")]
pub struct Presentation {
    #[serde(rename = "saveSubsetFonts")]
    save_subset_fonts: Option<String>,

    #[serde(rename = "xmlns:a")]
    a: Option<String>,

    #[serde(rename = "xmlns:r")]
    r: Option<String>,

    #[serde(rename = "xmlns:p")]
    p: Option<String>,

    // #[serde(rename(serialize="p:sldMasterIdLst"))]
    #[serde(rename(serialize = "p:sldMasterIdLst", deserialize = "sldMasterIdLst"))]
    sld_master_id_list: SldMasterIdLst,

    // #[serde(rename(serialize="p:sldIdLst"))]
    // #[serde(rename="p:sldIdLst")]
    #[serde(rename(serialize = "p:sldIdLst", deserialize = "sldIdLst"))]
    sld_id_list: SldIdList,

    #[serde(rename(serialize = "p:sldSz", deserialize = "sldSz"))]
    sld_sz: Sz,

    #[serde(rename(serialize = "p:notesSz", deserialize = "notesSz"))]
    note_sz: Sz,

    #[serde(rename(serialize = "p:defaultTextStyle", deserialize = "defaultTextStyle"))]
    default_text_style: DefaultTextStyle,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    ext_lst: ExtLst,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultTextStyle {
    #[serde(rename(serialize = "a:defPPr", deserialize = "defPPr"))]
    def_p_pr: DefPPr,

    #[serde(rename(serialize = "a:lvl1pPr", deserialize = "lvl1pPr"))]
    lvl1p_pr: LvlpPr,

    #[serde(rename(serialize = "a:lvl2pPr", deserialize = "lvl2pPr"))]
    lvl2p_pr: LvlpPr,

    #[serde(rename(serialize = "a:lvl3pPr", deserialize = "lvl3pPr"))]
    lvl3p_pr: LvlpPr,

    #[serde(rename(serialize = "a:lvl4pPr", deserialize = "lvl4pPr"))]
    lvl4p_pr: LvlpPr,

    #[serde(rename(serialize = "a:lvl5pPr", deserialize = "lvl5pPr"))]
    lvl5p_pr: LvlpPr,

    #[serde(rename(serialize = "a:lvl6pPr", deserialize = "lvl6pPr"))]
    lvl6p_pr: LvlpPr,

    #[serde(rename(serialize = "a:lvl7pPr", deserialize = "lvl7pPr"))]
    lvl7p_pr: LvlpPr,

    #[serde(rename(serialize = "a:lvl8pPr", deserialize = "lvl8pPr"))]
    lvl8p_pr: LvlpPr,

    #[serde(rename(serialize = "a:lvl9pPr", deserialize = "lvl9pPr"))]
    lvl9p_pr: LvlpPr,
}



#[derive(Deserialize, Serialize, Debug)]
pub struct LvlpPr {
    #[serde(rename = "marL")]
    mar_l: Option<String>,

    algn: Option<String>,

    #[serde(rename = "defTabSz")]
    def_tab_sz: Option<String>,

    rtl: Option<String>,

    #[serde(rename = "eaLnBrk")]
    ea_ln_brk: Option<String>,

    #[serde(rename = "latinLnBrk")]
    latin_ln_brk: Option<String>,

    #[serde(rename = "hangingPunct")]
    hanging_punct: Option<String>,

    #[serde(rename(serialize = "a:defRPr", deserialize = "defRPr"))]
    def_r_pr: DefRPr,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DefPPr {
    #[serde(rename(serialize = "a:defRPr", deserialize = "defRPr"))]
    def_r_pr: DefRPr,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DefRPr {
    lang: Option<String>,
    sz: Option<String>,
    kern: Option<String>,

    #[serde(rename(serialize = "a:solidFill", deserialize = "solidFill"))]
    solid_fill: Option<SolidFill>,

    #[serde(rename(serialize = "a:latin", deserialize = "latin"))]
    latin: Option<Latin>,

    #[serde(rename(serialize = "a:ea", deserialize = "ea"))]
    ea: Option<Ea>,

    #[serde(rename(serialize = "a:cs", deserialize = "cs"))]
    cs: Option<Cs>,
}







#[derive(Deserialize, Serialize, Debug)]
pub struct Sz {
    cx: String,
    cy: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct SldMasterIdLst {
    #[serde(rename(deserialize = "$value", serialize = "p:sldMasterId"))]
    list: Vec<SldMasterId>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SldMasterId {
    id: Option<String>,

    #[serde(rename = "r:id")]
    relationId: Option<String>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct SldIdList {
    #[serde(rename(deserialize = "$value", serialize = "p:sldId"))]
    list: Vec<SldId>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SldId {
    id: Option<String>,

    #[serde(rename = "r:id")]
    relationId: Option<String>,
}


