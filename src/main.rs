use craine::*;
use html_parser::Dom;
use html_parser::Node::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::result::Result;

// Currently muts the var
fn parse_import(content: &mut Vec<String>) -> Result<Vec<PathBuf>, String> {
    let regex = Regex::new("import\\s+(\\S+)").unwrap();

    let mut imports = vec![];

    let mut import_line = vec![];

    for (index, i) in content.iter().enumerate() {
        match regex.captures(&i) {
            Some(captures) => {
                let file_path = captures.get(1).map_or("", |m| m.as_str());

                let path = fs::canonicalize(PathBuf::from(file_path)).unwrap();
                if !path.exists() {
                    return Err(format!(
                        r#"
[import] Can not find file/directory
Path: {:?}
                           "#,
                        path
                    ));
                }

                imports.push(path);
                import_line.push(index);
            }
            None => {}
        }
    }

    for i in import_line {
        content[i] = "".to_string();
    }

    Ok(imports)
}

fn main() {
    let work_dir = get_work_dir().expect("[work_dir] Expected directory, got file instead");
    std::env::set_current_dir(&work_dir).expect("Can not set working dir");

    let pages_components = get_pages_components_list().unwrap();

    let pages = &pages_components.0;

    for page in pages {
        let page_hash = handler(page);

        let final_dom = replace_dom(page_hash.0.to_vec(), &page_hash.1);
        let html = dom_tree_to_html(final_dom);
        for i in html {
            println!("{}", i);
        }
    }
}

fn handler(
    path: &PathBuf,
) -> (
    Vec<html_parser::Node>,
    HashMap<String, Vec<html_parser::Node>>,
) {
    let mut hashmap = HashMap::new();
    let mut contents =
        read_file_to_lines(path.to_path_buf()).expect("Can not open file for reading");

    for import in parse_import(&mut contents).unwrap() {
        let returned_hash = handler(&import);
        for key in returned_hash.1 {
            let a = key.0;
            hashmap.entry(a.to_owned()).or_insert(key.1);
        }
    }

    let dom_tree = Dom::parse(&contents.join("\n")).expect("Could not parse DOM");
    hashmap.insert(get_name(&path.to_path_buf()).unwrap(), dom_tree.children.clone());

    (dom_tree.children.clone(), hashmap)
}

// Go through the dom_tree.
// make new dometree
// append the recursive output to the .children of the current element.
fn replace_dom(
    dom_tree: Vec<html_parser::Node>,
    map: &HashMap<String, Vec<html_parser::Node>>,
) -> Vec<html_parser::Node> {
    let mut new_dom_tree: Vec<html_parser::Node> = vec![];

    for i in dom_tree {
        match i {
            Element(mut element) => {
                if map.contains_key(&element.name) {
                    // Add the dom of component to `element.children`
                    // Change varaint to normal so children can be added
                    // Make current element a container ie, div

                    element.children = map.get(&element.name).unwrap().to_vec();
                    element.variant = html_parser::ElementVariant::Normal;
                    element.name = "div".to_string();
                }

                element.children = replace_dom(element.children, map);
                new_dom_tree.push(Element(element));
            }
            Text(text) => new_dom_tree.push(Text(text)),
            _ => {}
        }
    }

    new_dom_tree
}
