use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::prelude::*;
use html_parser::Node::*;

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

pub fn get_pages_components_list(work_dir: PathBuf) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut pages_vec = vec![];
    let mut components_vec = vec![];

    println!("{:?}", work_dir);

    for i in fs::read_dir(".").expect("[work_dir] Can not read contents of directroy") {
        let path = i.unwrap().path();
        let filename = path
            .file_stem()
            .expect("[work_dir] Can not parse filename")
            .to_str()
            .unwrap();

        if filename
            .chars()
            .next()
            .expect("[work_dir] Can not parse file name")
            .is_uppercase()
        {
            components_vec.push(path);
        } else {
            pages_vec.push(path);
        }
    }

    (pages_vec, components_vec)
}
pub fn get_work_dir() -> Option<PathBuf> {
    let i: Vec<String> = env::args().collect();

    let work_dir = PathBuf::from(&i[1]);

    if !work_dir.is_dir() {
        return None;
    }

    Some(work_dir)
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
