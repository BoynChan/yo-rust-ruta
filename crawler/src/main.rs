use dotenv::dotenv;
fn main() {
    dotenv().ok();
    let pw = std::env::var("PG_PW").unwrap();
    println!("{}", pw);
}
