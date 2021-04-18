use html_parser::{Dom, Node::*};
use regex::Regex;
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
        handler(page);
    }
}

fn handler(path: &PathBuf) {
    let mut contents =
        read_file_to_lines(path.to_path_buf()).expect("Can not open file for reading");

    for import in parse_import(&mut contents) {
        handler(&import);
    }

    let dom_tree = Dom::parse(&contents.join("\n"));

    let parse = dom_tree_to_html(dom_tree.unwrap().children);
    for line in parse {
        println!("{}", line);
    }

    println!("\n\n-----------------------------------------\n\n")
}

