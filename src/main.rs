use html_parser::{Dom, Node::*, Result};

fn main() -> Result<()> {
    let html = r#"
    <p> lorem ipsum </p>
    <br/>
    <form action="asdf" method="get">
    <input type="number">
    </form>
    <FancyHR />
    "#;

    let dom_tree = Dom::parse(html);

    extract(dom_tree.unwrap().children);

    Ok(())
}

// Recursive function to go through the DOM tree and printout a basic structure
fn extract(dom_tree: Vec<html_parser::Node>) {
    for i in dom_tree {
        match i {
            Element(element) => {
                // for self closing tags
                if element.variant == html_parser::ElementVariant::Void {
                    println!("<{} />", element.name);
                    continue;
                }

                println!("<{}>", &element.name);

                // Recursive child extraction
                if element.children.len() > 0 {
                    extract(element.children)
                }

                println!("</{}>", element.name)
            },
            Text(text) =>{
                println!("{}",text);
            },

            Comment(_) => {
                println!("comment");
            }
        }
    }
}
