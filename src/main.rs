mod tok;

fn main() {
    let string = "3 + 4 + (5 + 6)";
    let tokens = tok::tokenize(string).unwrap();
    for token in tokens {
        println!("{:?}", token)
    }
}
