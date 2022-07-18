use serde::Deserialize;
use serde::Serialize;


#[derive(Deserialize, Serialize, Debug)]
pub struct Latin{
    typeface:String,
    panose:Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Ea{
    typeface:String,
    panose:Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Cs{
    typeface:String,
    panose:Option<String>
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Font{
    script:String,
    typeface:String
}

