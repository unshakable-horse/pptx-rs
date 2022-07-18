use serde::Deserialize;
use serde::Serialize;

pub mod presentation;
pub mod pres_props;
pub mod table_styles;
pub mod view_props;
pub mod media;
pub mod _rels;
pub mod theme;


#[derive(Deserialize, Serialize, Debug)]
pub struct ExtLst {
    #[serde(rename(serialize = "p:ext", deserialize = "ext"))]
    list: Vec<Ext>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Ext {
    uri: String,

    #[serde(rename(serialize = "p15:sldGuideLst", deserialize = "sldGuideLst"))]
    sld_guide_lst: Option<SldGuideLst>,

    #[serde(rename(serialize = "p15:chartTrackingRefBased", deserialize = "chartTrackingRefBased"))]
    chart_tracking_ref_based: Option<ChartTrackingRefBased>,

    #[serde(rename(serialize = "p14:defaultImageDpi", deserialize = "defaultImageDpi"))]
    default_image_dpi: Option<DefaultImageDpi>,

    #[serde(rename(serialize = "p14:discardImageEditData", deserialize = "discardImageEditData"))]
    discard_image_edit_data: Option<DiscardImageEditData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SldGuideLst {
    #[serde(rename = "xmlns:p15")]
    p15: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChartTrackingRefBased{
    #[serde(rename = "xmlns:p15")]
    p15: String,

    val:String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscardImageEditData{
    #[serde(rename = "xmlns:p14")]
    p14: String,

    val:String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultImageDpi{
    #[serde(rename = "xmlns:p14")]
    p14: String,

    val:String
}