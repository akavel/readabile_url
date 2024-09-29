use std::process;

fn main() {
    let mut args = std::env::args();
    let _ = args.next(); // skip name of binary
    if args.len() != 1 {
        eprintln!(
            "readabile_url expects exactly 1 argument, an URL; got {} args",
            args.len()
        );
        process::exit(1);
    }
    let url = args.next().unwrap();
    match readability::extractor::scrape(&url) {
        Ok(product) => {
            println!("{}", product.content);
        }
        Err(err) => {
            eprintln!("readabile_url error: {}", err);
            process::exit(1);
        }
    }
}
