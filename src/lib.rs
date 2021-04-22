pub mod error_handler;
pub mod workspace;

use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::collections::HashMap;
use std::result::Result;
use html_parser::Dom;
use html_parser::Node::*;
use regex::Regex;

use error_handler::ErrorType;
use workspace::*;

pub fn read_file_to_lines(path: PathBuf) -> Result<Vec<String>,ErrorType> {
    match fs::File::open(&path) {
        Ok(file) => {
            let buf_reader = BufReader::new(file);
            let line_vec: Vec<_> = buf_reader.lines().collect();

            let mut string_vec: Vec<String> = vec![];

            for i in line_vec {
                string_vec.push(i.unwrap() as String);
            }

            return Ok(string_vec);
        }
        Err(_) => Err(ErrorType::WorkDir("Unable to open path")),
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
pub fn get_pages_components_list() -> Result<(Vec<PathBuf>, Vec<PathBuf>), ErrorType> {
    let mut pages_vec = vec![];
    let mut components_vec = vec![];

    let contents = match fs::read_dir(".") {
        Ok(contents) => contents,
        Err(_) => return Err(ErrorType::WorkDir("Can not read contents of directroy"))
    };

    for i in contents {
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
                return Err(ErrorType::WorkDir("Can not convert filename to string"));
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


fn handler(
    path: &PathBuf,
) -> Result<
    (
        Vec<html_parser::Node>,
        HashMap<String, Vec<html_parser::Node>>,
    ),
    ErrorType,
> {
    let mut hashmap = HashMap::new();
    let mut contents =
        read_file_to_lines(path.to_path_buf()).expect("Can not open file for reading");

    let imports = match parse_import(&mut contents) {
        Ok(imports) => imports,
        Err(e) => return Err(e),
    };

    for import in imports {
        let returned_hash = match handler(&import) {
            Ok(hash) => hash,
            Err(e) => return Err(e),
        };

        for key in returned_hash.1 {
            let a = key.0;
            hashmap.entry(a.to_owned()).or_insert(key.1);
        }
    }

    let dom_tree = match Dom::parse(&contents.join("\n")) {
        Ok(tree) => tree,
        Err(_) => return Err(ErrorType::Parse("Unable to parse dom tree")),
    };

    hashmap.insert(
        get_name(&path.to_path_buf()).unwrap(),
        dom_tree.children.clone(),
    );

    Ok((dom_tree.children.clone(), hashmap))
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

// Currently muts the var
fn parse_import(content: &mut Vec<String>) -> Result<Vec<PathBuf>, ErrorType> {
    let regex = Regex::new("import\\s+(\\S+)").unwrap();

    let mut imports = vec![];

    let mut import_line = vec![];

    for (index, i) in content.iter().enumerate() {
        match regex.captures(&i) {
            Some(captures) => {
                let file_path = captures.get(1).map_or("", |m| m.as_str());

                let path = fs::canonicalize(PathBuf::from(file_path)).unwrap();
                if !path.exists() {
                    return Err(ErrorType::Parse("Can not find file/directory"));
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


pub fn run() -> Result<(),ErrorType> {

    let work_dir = match get_work_dir() {
        Some(work_dir) => work_dir,
        None => return Err(ErrorType::Parse("Expected dir got file")),
    };

    match std::env::set_current_dir(&work_dir) {
        Err(_) => return Err(ErrorType::WorkDir("Unable to set current dir")),
        Ok(_) => {},

    }

    let workspace_config = match get_workspace_config(PathBuf::new().join(".")) {
        Ok(workspace_config) => workspace_config,
        Err(_) => return Err(ErrorType::Parse("Could not parse")),
    };

    let build_dir = match workspace_config.build_dir {
        Some(dir) => dir,
        None => return Err(ErrorType::BuildDir("Unable to find build directory")),
    };

    match fs::create_dir_all(&build_dir) {
        Ok(_) => {}
        Err(_) => return Err(ErrorType::BuildDir("{:?} Error in creating build dir")),
    };

    // TODO change behaviour in future
    if !build_dir.read_dir().unwrap().next().is_none() {
        return Err(ErrorType::BuildDir("build dir is not empty"));
    }

    let pages_components = match get_pages_components_list() {
        Ok(e) => e,
        Err(e) => return Err(e)
    };

    let pages = &pages_components.0;

    for page in pages {
        let page_hash = match  handler(page) {
            Err(e) => return Err(e),
            Ok(hash) => hash,
        };

        let final_dom = replace_dom(page_hash.0.to_vec(), &page_hash.1);
        let html = dom_tree_to_html(final_dom);

        let page_name = match get_name(page) {
            None => return Err(ErrorType::WorkDir("unable to get name")),
            Some(page) => page,
        };

        match fs::write(
            PathBuf::new().join(&build_dir).join(page_name),
            html.join("\n"),
        ) {

            Ok(_) => {},
            Err(_) => return Err(ErrorType::WorkDir("Unable to write file")),
        };
        
    }

    Ok(())
}
