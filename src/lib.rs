//! CRAINE is a HTML compiler built for react like components in pure html

#![deny(missing_docs)]

/// Contains ErrorType enum with all the possible error types craine can generate
///
/// Implements fmt::Display for ErrorType
pub mod error_handler;

/// Contains WorkspaceConfig struct and related "workspace" impls
pub mod workspace;

/// Variable parsing module
pub mod var_parser;

/// Cmd opts
pub mod cmd_params;

use html_parser::Dom;
use html_parser::Node::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;

use error_handler::ErrorType;
use workspace::*;

/**
 * Read file to a `\n` seperated to vector
 *
 * Errors if File::open() fails
 */
pub fn read_file_to_lines(path: PathBuf) -> Result<Vec<String>, ErrorType> {
    match fs::File::open(&path) {
        Ok(file) => {
            let buf_reader = BufReader::new(file);
            let line_vec: Vec<_> = buf_reader.lines().collect();

            let mut string_vec: Vec<String> = vec![];

            for i in line_vec {
                string_vec.push(i.unwrap() as String);
            }

            Ok(string_vec)
        }
        Err(_) => Err(ErrorType::WorkDir("Unable to open path")),
    }
}

/**
 * Returns  Option<String> of the file name without extension from a PathBuf
 * Returns None if path.file_stem() returns None OR if the file_stem conversion to string fails
 */
pub fn get_name(path: &Path) -> Option<String> {
    match path.file_stem() {
        None => None,
        Some(file_stem) => match file_stem.to_str() {
            Some(file_str) => Some(file_str.to_owned()),
            None => None,
        },
    }
}

/**
 * Returns true if the first character of the filename is a uppercase letter
 */
pub fn is_component(filename: String) -> bool {
    let first_char = filename.chars().next();

    match first_char {
        Some(first_char) => first_char.is_uppercase(),
        None => false,
    }
}

/**
 * Returns a tuple of two vectors, (pages_vec,components_vec,assets_vec)
 * Reads all the files (no globbing) and uses `is_component()` to construct final vector
 * NOTE: Already assumes that the program's working directory is `work_dir`
 */
pub fn get_pages_components_assets_list(
    src_dir: &PathBuf,
) -> Result<(Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>), ErrorType> {
    let mut pages_vec = vec![];
    let mut components_vec = vec![];
    let mut assets_vec = vec![];

    let contents = match fs::read_dir(src_dir) {
        Ok(contents) => contents,
        Err(_) => return Err(ErrorType::WorkDir("Can not read contents of directroy")),
    };

    for i in contents {
        let path = i.unwrap().path();
        let is_dir = path.is_dir();
        if is_dir {
            // TODO potential error
            let mut pages_components_assets = get_pages_components_assets_list(&path).unwrap();
            pages_vec.append(&mut pages_components_assets.0);
            components_vec.append(&mut pages_components_assets.1);
            assets_vec.append(&mut pages_components_assets.2);
            continue;
        }
        match path.extension() {
            Some(ext) => {
                if ext != "html" {
                    let asset_path = path.clone();
                    assets_vec.push(asset_path);
                    continue;
                }
            }
            None => {}
        };
        match get_name(&path) {
            Some(filename) => {
                if is_component(filename) {
                    components_vec.push(path);
                    continue;
                }
                pages_vec.push(path);
            }
            None => {
                return Err(ErrorType::WorkDir("Can not convert filename to string"));
            }
        }
    }

    Ok((pages_vec, components_vec, assets_vec))
}

/**
 * Recursive function to go through the DOM tree
 *
 * Adds
 * - Classes
 * - Id
 * - Attributes
 * - Children
 * -
 */
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

            Comment(_) => {}
        }
    }
    output
}

struct CraineHash {
    dom_tree: Vec<html_parser::Node>,
    component_hash: HashMap<String, Vec<html_parser::Node>>,
    used_components: Vec<String>,
}

/// Creates a component hash map and returns (Node vector,HashMap of compoenent name and dom_tree)
/// path: A path to a page/component to get the dom tree and compoenent hash of
fn handler(path: &Path, src_dir: &PathBuf) -> Result<CraineHash, ErrorType> {
    let mut hashmap = HashMap::new();
    let mut used_components: Vec<String> = vec![];
    let mut contents =
        read_file_to_lines(path.to_path_buf()).expect("Can not open file for reading");


    let mut final_path:PathBuf = path.to_path_buf();

    final_path.pop();

    let imports = match parse_import(&mut contents, &final_path) {
        Ok(imports) => imports,
        Err(e) => return Err(e),
    };
    for import in imports {
        //blindly trusting that this is a compoenent might be a bad idea in the future
        used_components.push(match get_name(&import) {
            Some(p) => p,
            None => return Err(ErrorType::Parse("unable to get import path")),
        });

        //handle components
        let returned_hash = match handler(&import, &src_dir) {
            Ok(hash) => hash,
            Err(e) => return Err(e),
        };

        // add all component hash to current one
        // if already in, not touch it
        for key in returned_hash.component_hash {
            let a = key.0;
            hashmap.entry(a.to_owned()).or_insert(key.1);
        }

        for key in returned_hash.used_components {
            used_components.push(key.to_string());
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

    Ok(CraineHash {
        dom_tree: dom_tree.children,
        component_hash: hashmap,
        used_components,
    })
}

// Go through the dom_tree.
// make new dometree
// append the recursive output to the .children of the current element.
fn replace_dom(
    dom_tree: Vec<html_parser::Node>,
    map: &HashMap<String, Vec<html_parser::Node>>,
    vars: HashMap<String, String>,
) -> Vec<html_parser::Node> {
    let mut new_dom_tree: Vec<html_parser::Node> = vec![];
    for i in dom_tree {
        let mut scoped_vars = vars.clone();
        match i {
            Element(mut element) => {
                use var_parser::replace_variables;

                for class in &mut element.classes {
                    *class = replace_variables(class, scoped_vars.clone()).unwrap();
                }

                if element.id.is_some() {
                    element.id =
                        Some(replace_variables(&element.id.unwrap(), scoped_vars.clone()).unwrap());
                }

                for attr in &mut element.attributes {
                    match attr.1 {
                        Some(_) => {
                            let new_attr =
                                replace_variables(attr.1.as_ref().unwrap(), scoped_vars.clone())
                                    .unwrap();
                            *attr.1 = Some(new_attr);
                        }
                        None => {}
                    }
                }

                if map.contains_key(&element.name) {
                    // it is a compoenent
                    // parsing variable now

                    for var in element.children {
                        match var {
                            Element(_) => panic!("no elem inside a compoenent"),
                            Text(text) => {
                                let content: Vec<&str> = text.split('\n').collect();

                                let asdf = var_parser::get_variables(&content);

                                match asdf {
                                    Ok(variables) => {
                                        scoped_vars.extend(variables);
                                    }
                                    Err(_) => panic!("asdf"),
                                };
                            }
                            Comment(_) => {}
                        }
                    }

                    // Add the dom of component to `element.children`
                    // Change varaint to normal so children can be added
                    // Make current element a container ie, div
                    element.children = map.get(&element.name).unwrap().to_vec();
                    element.variant = html_parser::ElementVariant::Normal;
                    element.name = "div".to_string();
                }

                element.children = replace_dom(element.children, map, scoped_vars.clone());
                new_dom_tree.push(Element(element));
            }
            Text(texta) => {
                let text = var_parser::replace_variables(&texta, scoped_vars).unwrap();
                new_dom_tree.push(Text(text))
            }
            _ => {}
        }
    }

    new_dom_tree
}

/**
 * Parses import statements and removes the statement from the given `content` vector
 * Uses `^import\s+(\S+)$` to get import statements.
 * Returns err if the file path of the import statement can not be found when making it abs path
 *
 * */
// Currently muts the var
fn parse_import(content: &mut Vec<String>, src_dir: &Path) -> Result<Vec<PathBuf>, ErrorType> {
    let regex = Regex::new("^import\\s+(\\S+)$").unwrap();

    let mut imports = vec![];

    let mut import_line = vec![];

    for (index, i) in content.iter().enumerate() {
        if let Some(captures) = regex.captures(&i) {
            let file_path = captures.get(1).map_or("", |m| m.as_str());
                
            let mut ppath :PathBuf = PathBuf::from(src_dir);

            ppath.push(file_path);

            let path = ppath.canonicalize().unwrap();

            if !path.exists() {
                return Err(ErrorType::Parse("Can not find file/directory"));
            }

            imports.push(path);
            import_line.push(index);
        }
    }

    for i in import_line {
        content[i] = "".to_string();
    }

    Ok(imports)
}

/**
 * Main library function to handle everything. Returns ErrorType (everything is bubled)
 *
 * FLOW
 * - Get Work dir
 * - Get config
 * - Get build dir and create
 * - For each page, get component hash
 * - Replace components with actual html
 * - Write the final dom_tree HTML to file
 *
 * */
pub fn run() -> Result<(), ErrorType> {
    use cmd_params::Config;
    use structopt::StructOpt;
    let opts = Config::from_args();
    let workspace_dir = opts.path;

    if std::env::set_current_dir(&workspace_dir).is_err() {
        return Err(ErrorType::WorkDir("Unable to set current dir"));
    }

    let workspace_config = match get_workspace_config(PathBuf::new().join(".")) {
        Ok(workspace_config) => workspace_config,
        Err(_) => return Err(ErrorType::Parse("Could not parse")),
    };

    let build_dir = match workspace_config.build_dir {
        Some(dir) => dir,
        None => return Err(ErrorType::BuildDir("Unable to find build directory")),
    };

    // clear the dir
    match build_dir.read_dir() {
        Ok(_) => match std::fs::remove_dir_all(&build_dir) {
            Ok(_) => {}
            Err(_) => return Err(ErrorType::BuildDir("Unable to remove build_dir")),
        },
        _ => {}
    };

    match fs::create_dir_all(&build_dir) {
        Ok(_) => {}
        Err(_) => return Err(ErrorType::BuildDir("{:?} Error in creating build dir")),
    };

    let src_dir = match get_src_dir(workspace_dir) {
        None => return Err(ErrorType::Workspace("unable to get src dir")),
        Some(a) => a,
    };

   let pages_components_assets = match get_pages_components_assets_list(&src_dir) {
        Ok(e) => e,
        Err(e) => return Err(e),
    };


    let pages = &pages_components_assets.0;
    let mut used_components = vec![];

    for page in pages {
        let page_hash = match handler(page, &src_dir) {
            Err(e) => return Err(e),
            Ok(hash) => hash,
        };

        for i in page_hash.used_components {
            used_components.push(i)
        }

        let final_dom = replace_dom(
            page_hash.dom_tree.to_vec(),
            &page_hash.component_hash,
            HashMap::new(),
        );
        let html = dom_tree_to_html(final_dom);

        let page_name = match get_name(page) {
            None => return Err(ErrorType::WorkDir("unable to get name")),
            Some(page) => page,
        };

            let pa = PathBuf::new()
                .join(&build_dir)
                .join(format!("{}.html", page_name));

        println!("Writing: {:?}",pa);

        match fs::write(
            pa,
            html.join("\n"),
        ) {
            Ok(_) => {}
            Err(_) => return Err(ErrorType::WorkDir("Unable to write file")),
        };
    }

    let assets = &pages_components_assets.2;
    // Assets

    for asset in assets {
        let mut new = PathBuf::new()
            .join(&build_dir)
            .join(get_name(asset).unwrap());

        add_extension(&mut new, asset.extension().unwrap());

        println!("Copying asset: {:?}", new);
        fs::copy(asset, new).unwrap();
    }

    // Generate warnings

    for i in pages_components_assets.1 {
        //TODO error ; expect call
        if !(used_components.contains(&get_name(&i).expect("asd"))) {
            let path_str = i.to_str();
            let path_str = match path_str {
                Some(path) => path,
                None => return Err(ErrorType::Parse("couldnt open dir")),
            };

            println!("Unused Component: {}", path_str);
        }
    }

    Ok(())
}

fn add_extension(path: &mut std::path::PathBuf, extension: impl AsRef<std::path::Path>) {
    match path.extension() {
        Some(ext) => {
            let mut ext = ext.to_os_string();
            ext.push(".");
            ext.push(extension.as_ref());
            path.set_extension(ext)
        }
        None => path.set_extension(extension.as_ref()),
    };
}
