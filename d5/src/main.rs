fn main() {
    println!("Hello, world!");
    let retur: String = callword(String::from("Hello"));
    println!("{}", retur);
}


fn callword(sentence: String) -> String{
    let mut retu: String = String::new();
    for c in sentence.chars() {
        for uc in c.to_uppercase() {
            retu.push(uc);
        }
    }
    retu
}
