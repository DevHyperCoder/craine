use html_parser::{Dom, Result};

fn main() -> Result<()> {
    let html = "<p> lorem ipsum </p>";
    let json = Dom::parse(html);
    println!("{:#?}", json.unwrap());
    Ok(())
}
