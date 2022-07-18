use serde::Serialize;
use serde::Deserialize;


pub const APP_FILE_NAME:&'static str = "docProps/app.xml";

#[derive(Serialize,Deserialize)]
#[serde(rename="Properties")]
pub struct App{
    pub xmlns:String,

    #[serde(rename="xmlns:vt")]
    pub vt:String,

    #[serde(rename="$unflatten=TotalTime")]
    pub total_time:i32,

    #[serde(rename="$unflatten=Words")]
    pub words:i32,

    #[serde(rename="$unflatten=Application")]
    pub application:String,

    #[serde(rename="$unflatten=PresentationFormat")]
    pub presentation_format:String,

    #[serde(rename="$unflatten=Paragraphs")]
    pub paragraphs:i32,

    #[serde(rename="$unflatten=Slides")]
    pub slides:i32,

    #[serde(rename="$unflatten=Notes")]
    pub notes:i32,

    #[serde(rename="$unflatten=HiddenSlides")]
    pub hidden_slides:i32,

    #[serde(rename="$unflatten=MMClips")]
    pub m_m_clips:i32,

    #[serde(rename="$unflatten=ScaleCrop")]
    pub scale_crop:bool,

    #[serde(rename="HeadingPairs")]
    pub heading_pairs:HeadingPairs,

    #[serde(rename="TitlesOfParts")]
    pub titles_of_parts:TitlesOfParts,

    #[serde(rename="$unflatten=LinksUpToDate")]
    pub links_up_to_date:bool,


    #[serde(rename="$unflatten=SharedDoc")]
    pub shared_doc:bool,


    #[serde(rename="$unflatten=HyperlinksChanged")]
    pub hyperlinks_changed:bool,


    #[serde(rename="$unflatten=AppVersion")]
    pub app_version:String,
}

#[derive(Serialize,Deserialize)]
pub struct HeadingPairs{
    #[serde(rename(deserialize = "$value", serialize = "vt:vector"))]
    pub vt_vector:VtVector<VtVariant>
}

#[derive(Serialize,Deserialize)]
pub struct VtVector<T>{
    #[serde(rename="size")]
    pub size:i32,

    #[serde(rename="baseType")]
    pub base_type:String,


    #[serde(rename(deserialize = "$value", serialize = "vt:variant"))]
    pub vt_variants:Vec<T>,

}

#[derive(Serialize,Deserialize)]
pub struct VtVariant{
    #[serde(rename="$unflatten=vt:lpstr",skip_serializing_if = "Option::is_none")]
    vt_lpstr:Option<String>,

    #[serde(rename="$unflatten=vt:i4",skip_serializing_if = "Option::is_none")]
    vt_i4:Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct TitlesOfParts{
    #[serde(rename(deserialize = "$value", serialize = "vt:vector"))]
    pub vt_vector:VtVector<VtLpstr>
}

#[derive(Serialize,Deserialize)]
pub struct VtLpstr{
    #[serde(rename="$value")]
    pub body:String
}