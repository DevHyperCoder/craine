use craine::run;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
