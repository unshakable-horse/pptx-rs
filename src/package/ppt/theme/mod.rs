use serde::Serialize;
use serde::Deserialize;
use regex::Regex;
use crate::package::common::fill::{GradFill, SolidFill};
use crate::package::common::font::{Cs, Ea, Font, Latin};

pub const THEME_XML_FILE_NAME_PATTERN: &'static str = "ppt/theme/theme\\d.xml";

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename="a:theme")]
pub struct Theme{
    name:String,

    #[serde(rename="xmlns:a")]
    a:String,

    #[serde(rename(serialize="a:themeElements",deserialize="themeElements"))]
    theme_elements:ThemeElements

}

#[derive(Deserialize, Serialize, Debug)]
pub struct ThemeElements{
    #[serde(rename(serialize="a:clrScheme",deserialize="clrScheme"))]
    clr_scheme:ClrScheme,

    #[serde(rename(serialize="a:fontScheme",deserialize="fontScheme"))]
    font_scheme:FontScheme
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClrScheme{
    name:String,

    #[serde(rename(serialize="a:dk1",deserialize="dk1"))]
    dk1:Dk1,

    #[serde(rename(serialize="a:lt1",deserialize="lt1"))]
    lt1:Lt1,

    #[serde(rename(serialize="a:dk2",deserialize="dk2"))]
    dk2:Dk2,

    #[serde(rename(serialize="a:lt2",deserialize="lt2"))]
    lt2:Lt2,

    #[serde(rename(serialize="a:accent1",deserialize="accent1"))]
    accent1:Accent,

    #[serde(rename(serialize="a:accent2",deserialize="accent2"))]
    accent2:Accent,

    #[serde(rename(serialize="a:accent3",deserialize="accent3"))]
    accent3:Accent,

    #[serde(rename(serialize="a:accent4",deserialize="accent4"))]
    accent4:Accent,

    #[serde(rename(serialize="a:accent5",deserialize="accent5"))]
    accent5:Accent,

    #[serde(rename(serialize="a:accent6",deserialize="accent6"))]
    accent6:Accent,

    #[serde(rename(serialize="a:hlink",deserialize="hlink"))]
    hlink:Hlink,

    #[serde(rename(serialize="a:folHlink",deserialize="folHlink"))]
    fol_hlink:FolHlink



}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dk1{
    #[serde(rename(serialize="a:sysClr",deserialize="sysClr"))]
    sys_clr:SysClr,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Dk2{
    #[serde(rename(serialize="a:srgbClr",deserialize="srgbClr"))]
    srgb_clr:SrgbClr,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Lt1{
    #[serde(rename(serialize="a:sysClr",deserialize="sysClr"))]
    sys_clr:SysClr,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Lt2{
    #[serde(rename(serialize="a:srgbClr",deserialize="srgbClr"))]
    srgb_clr:SrgbClr,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Accent{
    #[serde(rename(serialize="a:srgbClr",deserialize="srgbClr"))]
    srgb_clr:SrgbClr,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Hlink{
    #[serde(rename(serialize="a:srgbClr",deserialize="srgbClr"))]
    srgb_clr:SrgbClr,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct FolHlink{
    #[serde(rename(serialize="a:srgbClr",deserialize="srgbClr"))]
    srgb_clr:SrgbClr,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct SysClr{
    val:String,

    #[serde(rename="lastClr")]
    last_clr:String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct SrgbClr{
    val:String,

}


#[derive(Deserialize, Serialize, Debug)]
pub struct FontScheme{
    name:String,

    #[serde(rename(serialize="a:majorFont",deserialize="majorFont"))]
    major_font:MajorFont,

    #[serde(rename(serialize="a:minorFont",deserialize="minorFont"))]
    minor_font:MajorFont
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MajorFont{
    #[serde(rename(serialize="a:latin",deserialize="latin"))]
    latin:Latin,

    #[serde(rename(serialize="a:ea",deserialize="ea"))]
    ea:Ea,

    #[serde(rename(serialize="a:cs",deserialize="cs"))]
    cs:Cs,

    #[serde(rename(serialize="a:font",deserialize="font"))]
    fonts:Vec<Font>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MinorFont{
    #[serde(rename(serialize="a:latin",deserialize="latin"))]
    latin:Latin,

    #[serde(rename(serialize="a:ea",deserialize="ea"))]
    ea:Ea,

    #[serde(rename(serialize="a:cs",deserialize="cs"))]
    cs:Cs,

    #[serde(rename(serialize="a:font",deserialize="font"))]
    fonts:Vec<Font>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FmtScheme{
    name:String,

    #[serde(rename(serialize="a:fillStyleLst",deserialize="fillStyleLst"))]
    fill_style_lst: FillStyleLst,


    // //todo:
    // #[serde(rename(serialize="a:lnStyleLst",deserialize="lnStyleLst"))]
    // ln_style_lst: StyleLst,
    //
    // #[serde(rename(serialize="a:effectStyleLst",deserialize="effectStyleLst"))]
    // effect_style_lst: StyleLst,
    //
    // #[serde(rename(serialize="a:bgFillStyleLst",deserialize="bgFillStyleLst"))]
    // bg_fill_style_lst: StyleLst,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FillStyleLst {
    #[serde(rename(serialize = "a:solidFill", deserialize = "solidFill"))]
    solid_fill: Option<SolidFill>,

    #[serde(rename(serialize = "a:gradFill", deserialize = "gradFill"))]
    grad_fill: Option<GradFill>,

}