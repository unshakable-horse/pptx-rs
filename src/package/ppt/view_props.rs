
use serde::Deserialize;
use serde::Serialize;
use crate::package::ppt::SldGuideLst;


pub const VIEW_PROPS_XML_FILE_NAME: &'static str = "ppt/viewProps.xml";



#[derive(Deserialize, Serialize, Debug)]
pub struct ViewProps{
    #[serde(rename = "xmlns:a")]
    a: Option<String>,

    #[serde(rename = "xmlns:r")]
    r: Option<String>,

    #[serde(rename = "xmlns:p")]
    p: Option<String>,

    #[serde(rename(serialize="p:normalViewPr",deserialize="normalViewPr"))]
    normal_view_pr:NormalViewPr,

    #[serde(rename(serialize="p:slideViewPr",deserialize="slideViewPr"))]
    slide_view_pr:SlideViewPr,

    #[serde(rename(serialize="p:notesTextViewPr",deserialize="notesTextViewPr"))]
    notes_text_view_pr:NotesTextViewPr,

    #[serde(rename(serialize="p:gridSpacing",deserialize="gridSpacing"))]
    grid_spacing:GridSpacing
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GridSpacing{
    cx:String,
    cy:String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SlideViewPr{
    #[serde(rename(serialize="p:cSldViewPr",deserialize="cSldViewPr"))]
    c_sld_view_pr:CSldViewPr
}
#[derive(Deserialize, Serialize, Debug)]
pub struct NotesTextViewPr{
    #[serde(rename(serialize="p:cViewPr",deserialize="cViewPr"))]
    c_view_pr:CViewPr,
}



#[derive(Deserialize, Serialize, Debug)]
pub struct CSldViewPr{
    #[serde(rename = "snapToGrid")]
    snap_to_grid:String,


    #[serde(rename(serialize="p:cViewPr",deserialize="cViewPr"))]
    c_view_pr:CViewPr,

    #[serde(rename(serialize="p:guideLst",deserialize="guideLst"))]
    guide_lst:Option<GuideLst>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CViewPr{
    #[serde(rename = "varScale")]
    var_scale:Option<String>,

    #[serde(rename(serialize="p:scale",deserialize="scale"))]
    scale:Scale,

    #[serde(rename(serialize="p:origin",deserialize="origin"))]
    origin:Origin,


}

#[derive(Deserialize, Serialize, Debug)]
pub struct GuideLst{

}


#[derive(Deserialize, Serialize, Debug)]
pub struct Scale{
    #[serde(rename(serialize="a:sx",deserialize="sx"))]
    sx:ScaleX,

    #[serde(rename(serialize="a:sy",deserialize="sy"))]
    sy:ScaleX
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScaleX{
    n:String,
    d:String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Origin{
    x:String,
    y:String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct NormalViewPr{
    #[serde(rename="horzBarState")]
    horz_bar_state:String,

    #[serde(rename(serialize="p:restoredLeft",deserialize="restoredLeft"))]
    restored_left: Restored,

    #[serde(rename(serialize="p:restoredTop",deserialize="restoredTop"))]
    restored_top: Restored,

}


#[derive(Deserialize, Serialize, Debug)]
pub struct Restored {
    sz:String,

    autoAdjust:Option<String>
}