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

    extract(dom_tree.unwrap().children);
}

// Recursive function to go through the DOM tree and printout a basic structure
fn extract(dom_tree: Vec<html_parser::Node>) {
    for i in dom_tree {
        match i {
            Element(element) => {
                println!("<{} ", element.name);

                // add classes
                if element.classes.len() > 0 {
                    print!("class=\"");
                    for i in element.classes {
                        print!("{} ", i);
                    }
                    print!("\"");
                }

                // add id
                match element.id {
                    None => {},
                    Some(id) => {
                        println!("id=\"{}\"", id);
                    }
                }

                // TODO add support for non-value stuff as well
                for i in element.attributes {
                    println!("{}=\"{:?}\"", i.0, i.1);
                }

                // for self closing tags
                if element.variant == html_parser::ElementVariant::Void {
                    println!("/>");
                    continue;
                }

                println!(">");

                // Recursive child extraction
                if element.children.len() > 0 {
                    extract(element.children)
                }

                println!("</{}>", element.name)
            }
            Text(text) => {
                println!("{}", text);
            }

            Comment(_) => {
                println!("comment");
            }
        }
    }
}
