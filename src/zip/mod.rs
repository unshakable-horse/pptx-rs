use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use quick_xml::de::from_str;
use quick_xml::events::Event;
use quick_xml::events::Event::Decl;
use quick_xml::{Reader, Writer};
use quick_xml::se::{to_string, to_writer};
use regex::Regex;
use xml::EventReader;
use xml::reader::XmlEvent;
use zip::read::ZipFile;
use zip::ZipArchive;
use crate::package::{content_type, doc_props};
use crate::package::content_type::ContentType;
use crate::package::doc_props::app::App;
use crate::package::doc_props::core::Core;
use crate::package::doc_props::DOC_PROPS_DIR;
use crate::package::ppt::{_rels, pres_props, presentation, table_styles, view_props};
use crate::package::ppt::_rels::presentaion_xml_rels::PresentationXmlRels;
use crate::package::ppt::media::MEDIA_DIR;
use crate::package::ppt::pres_props::PresentationProperties;
use crate::package::ppt::presentation::Presentation;
use crate::package::ppt::table_styles::TableStyles;
use crate::package::ppt::theme::{Theme, THEME_XML_FILE_NAME_PATTERN};
use crate::package::ppt::view_props::ViewProps;
use crate::xml::read_xml_dcl;

pub fn open(path: &str) -> ZipArchive<File> {
    let file: File = File::open(path).unwrap();
    let zip: ZipArchive<File> = ZipArchive::new(file).unwrap();
    return zip;
}

pub fn print_name_list(zip: &ZipArchive<File>) {
    let iterator = zip.file_names();
    for x in iterator {
        println!("x = {}", x);
    }
}

pub fn list_file(zip: &mut ZipArchive<File>) {
    let len = zip.len();

    let  THEME_XML_FILE_NAME_PATTERN_REGEX: Regex = Regex::new(THEME_XML_FILE_NAME_PATTERN).unwrap();

    for i in 0..len {
        let mut file = zip.by_index(i).unwrap();

        let file_name = file.name();
        // println!("file_name = {}", file_name);
        match file_name {
            content_type::CONTENT_TYPE_FILE_NAME => {
                let mut xml = String::new();
                let _ = file.read_to_string(&mut xml);

                let mut file = File::create("dist/[Content_Types].xml").unwrap();

                let dcl = read_xml_dcl(&xml);
                if let Ok(dcl) = dcl {
                    let _ = file.write(dcl.as_bytes());
                }

                let content_type: ContentType = from_str(&xml).unwrap();
                let _ = to_writer(file, &content_type);
            }
            doc_props::app::APP_FILE_NAME => {
                let mut xml = String::new();
                file.read_to_string(&mut xml);
                let app: App = quick_xml::de::from_str(&xml).unwrap();
                let mut file = File::create("dist/docProps/app.xml").unwrap();

                let dcl = read_xml_dcl(&xml);
                if let Ok(dcl) = dcl {
                    let _ = file.write(dcl.as_bytes());
                }

                to_writer(file, &app);
            }
            doc_props::core::CORE_FILE_NAME => {
                let mut xml = String::new();
                let _ = file.read_to_string(&mut xml);
                let app: Core = quick_xml::de::from_str(&xml).unwrap();
                let mut file = File::create("dist/docProps/core.xml").unwrap();

                let dcl = read_xml_dcl(&xml);
                if let Ok(dcl) = dcl {
                    let _ = file.write(dcl.as_bytes());
                }
                let _ = to_writer(file, &app);
            }
            presentation::PRESENTATION_XML_FILE_NAME => {
                let mut xml = String::new();
                let _ = file.read_to_string(&mut xml);
                let presentation: Presentation = from_str(&xml).unwrap();
                let mut file = File::create("dist/ppt/presentation.xml").unwrap();

                let dcl = read_xml_dcl(&xml);
                if let Ok(dcl) = dcl {
                    let _ = file.write(dcl.as_bytes());
                }
                let _ = to_writer(file, &presentation);
            }
            pres_props::PRESENTATION_PROPS_XML_FILE_NAME => {
                let mut xml = String::new();
                let _ = file.read_to_string(&mut xml);
                let presentation_properties: PresentationProperties = from_str(&xml).unwrap();
                let mut file = File::create("dist/ppt/presProps.xml").unwrap();

                let dcl = read_xml_dcl(&xml);
                if let Ok(dcl) = dcl {
                    let _ = file.write(dcl.as_bytes());
                }
                let _ = to_writer(file, &presentation_properties);
            }
            table_styles::TABLE_STYLES_XML_FILE_NAME => {
                let mut xml = String::new();
                let _ = file.read_to_string(&mut xml);
                let table_styles: TableStyles = from_str(&xml).unwrap();
                let mut file = File::create("dist/ppt/tableStyles.xml").unwrap();

                let dcl = read_xml_dcl(&xml);
                if let Ok(dcl) = dcl {
                    let _ = file.write(dcl.as_bytes());
                }
                let _ = to_writer(file, &table_styles);
            }
            view_props::VIEW_PROPS_XML_FILE_NAME => {
                let mut xml = String::new();
                let _ = file.read_to_string(&mut xml);
                let view_props: ViewProps = from_str(&xml).unwrap();
                let mut file = File::create("dist/ppt/viewProps.xml").unwrap();

                let dcl = read_xml_dcl(&xml);
                if let Ok(dcl) = dcl {
                    let _ = file.write(dcl.as_bytes());
                }
                let _ = to_writer(file, &view_props);
            }

            _rels::presentaion_xml_rels::PRESENTATION_RELS_XML_FILE_NAME => {
                let mut xml = String::new();
                let _ = file.read_to_string(&mut xml);
                let presentation_xml_rels: PresentationXmlRels = from_str(&xml).unwrap();
                let mut file = File::create("dist/ppt/rels/presentation.xml.rels").unwrap();

                let dcl = read_xml_dcl(&xml);
                if let Ok(dcl) = dcl {
                    let _ = file.write(dcl.as_bytes());
                }
                let _ = to_writer(file, &presentation_xml_rels);
            }
            _ => {
                let current_file_name = file_name.to_string();


                if current_file_name.contains(DOC_PROPS_DIR)
                    | current_file_name.contains(MEDIA_DIR) {
                    let mut buf = Vec::new();
                    // file.read(&mut buf).expect("TODO: panic message");
                    file.read_to_end(&mut buf);

                    let mut target_file_name = "dist".to_string();
                    target_file_name.push_str("/");
                    target_file_name.push_str(&current_file_name);

                    let mut target_file = File::create(&target_file_name).unwrap();

                    // println!("buf.size ={}", buf.len());

                    let _ = target_file.write(&mut buf);
                } else if THEME_XML_FILE_NAME_PATTERN_REGEX.is_match(file_name) {
                    let mut xml = String::new();
                    let _ = file.read_to_string(&mut xml);
                    let theme: Theme = from_str(&xml).unwrap();
                    let mut target_file_name = "dist".to_string();
                    target_file_name.push_str("/");
                    target_file_name.push_str(&current_file_name);

                    let mut file = File::create(target_file_name).unwrap();

                    let dcl = read_xml_dcl(&xml);
                    if let Ok(dcl) = dcl {
                        let _ = file.write(dcl.as_bytes());
                    }
                    let _ = to_writer(file, &theme);
                } else {
                    println!("remain files = {}", current_file_name);
                }
            }
        }
    }
}

fn print_xml(file: ZipFile) {
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                print!("{}+{}  | ", indent(depth), name);
                for attribute in attributes {
                    print!("{}={}", attribute.name.prefix.unwrap(), attribute.value)
                }
                println!("");
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}


fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}