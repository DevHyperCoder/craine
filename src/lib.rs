pub mod workspace;

use html_parser::Node::*;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

pub fn read_file_to_lines(path: PathBuf) -> Option<Vec<String>> {
    match fs::File::open(path) {
        Ok(file) => {
            let buf_reader = BufReader::new(file);
            let line_vec: Vec<_> = buf_reader.lines().collect();

            let mut string_vec: Vec<String> = vec![];

            for i in line_vec {
                string_vec.push(i.unwrap() as String);
            }

            return Some(string_vec);
        }
        Err(_) => None,
    }
}
pub fn get_name(path: &PathBuf) -> Option<String> {
    match path.file_stem() {
        None => return None,
        Some(file_stem) => match file_stem.to_str() {
            Some(file_str) => return Some(file_str.to_owned()),
            None => return None,
        },
    }
}

fn is_component(filename: String) -> bool {
    let first_char = filename.chars().next();

    match first_char {
        Some(first_char) => first_char.is_uppercase(),
        None => false,
    }
}

// TODO Use work_dir instead of '.'
pub fn get_pages_components_list() -> Result<(Vec<PathBuf>, Vec<PathBuf>), &'static str> {
    let mut pages_vec = vec![];
    let mut components_vec = vec![];

    for i in fs::read_dir(".").expect("[work_dir] Can not read contents of directroy") {
        let path = i.unwrap().path();
        match get_name(&path) {
            Some(filename) => {
                if is_component(filename) {
                    components_vec.push(path);
                } else {
                    pages_vec.push(path);
                }
            }
            None => {
                return Err("[work_dir] Can not convert filename to string");
            }
        }
    }

    println!("{:?} {:?}", pages_vec, components_vec);

    Ok((pages_vec, components_vec))
}

// Recursive function to go through the DOM tree and printout a basic structure
pub fn dom_tree_to_html(dom_tree: Vec<html_parser::Node>) -> Vec<String> {
    let mut output = vec![];
    for i in dom_tree {
        match i {
            Element(element) => {
                output.push(format!("<{} ", element.name));

                // add classes
                if !element.classes.is_empty() {
                    output.push("class=\"".to_string());
                    for i in element.classes {
                        output.push(format!("{} ", i));
                    }
                    output.push("\"".to_string());
                }

                // add id
                match element.id {
                    None => {}
                    Some(id) => {
                        output.push(format!("id=\"{}\"", id));
                    }
                }

                // add attributes.
                // type = "input"
                // OR
                // readonly
                for i in element.attributes {
                    output.push(match i.1 {
                        Some(attrname) => {
                            format!("{}=\"{}\" ", i.0, attrname)
                        }
                        None => {
                            format!("{} ", i.0)
                        }
                    });
                }

                // for self closing tags
                if element.variant == html_parser::ElementVariant::Void {
                    output.push("/>".to_string());
                    continue;
                }

                output.push(">".to_string());

                // Recursive child extraction
                let mut ex = dom_tree_to_html(element.children);
                output.append(&mut ex);

                output.push(format!("</{}>", element.name));
            }

            Text(text) => {
                output.push(text);
            }

            Comment(_) => {
                //println!("comment");
            }
        }
    }
    output
}
