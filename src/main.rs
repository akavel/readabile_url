fn main() {
    println!("Hello, world!");
    match readability::extractor::scrape("https://akavel.com/solo-nondisappointed") {
        Ok(product) => {
            println!("------- html ------");
            println!("{}", product.content);
            println!("---- plain text ---");
            println!("{}", product.text);
        }
        Err(_) => println!("error occured"),
    }
}
