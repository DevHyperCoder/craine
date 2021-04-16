use html_parser::{Dom, Node::*};

fn main() {
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
