use html_parser::{Dom, Node::*};
use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;

// Currently muts the var
fn parse_import(content: &mut Vec<String>) -> Vec<PathBuf> {
    let regex = Regex::new("import\\s+(\\S+)").unwrap();

    let mut imports = vec![];

    let mut import_line = vec![];

    for (index,i) in content.iter().enumerate() {
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
            None => {

                //panic!("[import] {} Error parsing import statement",i);
            }
        }
    }

    for i in import_line {
        content.remove(i);
    }

    imports
}

fn get_pages_components_list(work_dir: PathBuf) -> (Vec<PathBuf>, Vec<PathBuf>) {
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

fn get_work_dir() -> Option<PathBuf> {
    let i: Vec<String> = env::args().collect();

    let work_dir = PathBuf::from(&i[1]);

    if !work_dir.is_dir() {
        return None;
    }

    Some(work_dir)
}

fn read_file_to_lines(path: PathBuf) -> Option<Vec<String>> {
    match fs::File::open(path) {
        Ok(file) => {
            let buf_reader = std::io::BufReader::new(file);
            use std::io::prelude::*;
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

fn main() {
    let work_dir = get_work_dir().expect("[work_dir] Expected directory, got file instead");
    std::env::set_current_dir(&work_dir).expect("Can not set working dir");

    let pages_components = get_pages_components_list(work_dir);

    let pages = &pages_components.0;
    let components = &pages_components.1;

    println!("{:?}", pages);
    println!("{:?}", components);

    for page in pages {
        handler(page);
    }

    let html = r#"
    <p class="t-2 w-100"> lorem ipsum </p>
    <br/>
    <form id="login-form" action="asdf" method="get">
    <input type="number">
    </form>
    <FancyHR />
    "#;

    let dom_tree = Dom::parse(html);

    let a = extract(dom_tree.unwrap().children);

    for i in a {
        print!("{}", i);
    }
}

fn handler(path:&PathBuf) {
        let mut  contents =
            read_file_to_lines(path.to_path_buf()).expect("Can not open file for reading");

        for import in parse_import(&mut contents) {
            handler(&import);
        }

        let dom_tree = Dom::parse(&contents.join("\n"));

let parse = extract(dom_tree.unwrap().children);
        for line in parse {
            println!("{}", line);
        }

        println!("\n\n-----------------------------------------\n\n")
}


// Recursive function to go through the DOM tree and printout a basic structure
fn extract(dom_tree: Vec<html_parser::Node>) -> Vec<String> {
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
                let mut ex = extract(element.children);
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
