fn tokenize(input: &str) -> Vec<&str> {
    let tokens: Vec<&str> = input.split(' ').collect();
    return tokens;
}

pub fn parse(input: &str) {
    let tokens = tokenize(input);
    for token in tokens {
        println!("{}", token)
    }
}
