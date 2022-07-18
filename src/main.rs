use std::{env, time};
use std::ops::Sub;
use std::time::Instant;
use pptx_rs::zip;

fn main() {


    let instant = Instant::now();
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("filename = {}",filename);

    let mut zipFile = zip::open(filename);
    zip::list_file(& mut zipFile);
    let end = time::SystemTime::now();

    let i = instant.elapsed().as_millis();
    println!("cost time  = {} ms",i);

}
