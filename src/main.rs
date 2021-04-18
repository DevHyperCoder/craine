use html_parser::Dom;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use craine::*;

// Currently muts the var
fn parse_import(content: &mut Vec<String>) -> Vec<PathBuf> {
    let regex = Regex::new("import\\s+(\\S+)").unwrap();

    let mut imports = vec![];

    let mut import_line = vec![];

    for (index, i) in content.iter().enumerate() {
        match regex.captures(&i) {
            Some(captures) => {
                let file_path = captures.get(1).map_or("", |m| m.as_str());

                let path = fs::canonicalize(PathBuf::from(file_path)).unwrap();
                if !path.exists() {
                    panic!(
                        r#"
[import] Can not find file/directory
Path: {:?}
                           "#,
                        path
                    );
                }

                imports.push(path);
                import_line.push(index);
            }
            None => {}
        }
    }

    for i in import_line {
        content.remove(i);
    }

    imports
}

fn main() {
    let work_dir = get_work_dir().expect("[work_dir] Expected directory, got file instead");
    std::env::set_current_dir(&work_dir).expect("Can not set working dir");

    let pages_components = get_pages_components_list(work_dir);

    let pages = &pages_components.0;

    for page in pages {
        let page_hash = handler(page);
        println!("{:#?}", page_hash);
    }
}

fn handler(path: &PathBuf) -> HashMap<String, Vec<html_parser::Node>> {
    let mut hashmap = HashMap::new();
    let mut contents =
        read_file_to_lines(path.to_path_buf()).expect("Can not open file for reading");
    for import in parse_import(&mut contents) {
        let returned_hash = handler(&import);
        for key in returned_hash {
            let a = key.0;
            hashmap.entry(a.to_owned()).or_insert(key.1);
        }
    }

    // TODO error handling when dom is None
    let dom_tree = Dom::parse(&contents.join("\n"));

    hashmap.insert(
        path.to_path_buf().to_str().unwrap().to_owned(),
        dom_tree.unwrap().children,
    );

    hashmap
}
