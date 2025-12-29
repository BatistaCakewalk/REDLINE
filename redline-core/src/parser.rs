pub fn parse_statement(tokens: Vec<Token>) {
    match &tokens[0] {
        Token::Let => {
            // Logic for: let x: int = 10
            println!("I found a variable declaration!");
        }
        Token::Def => {
            // Logic for: def my_func():
            println!("I found a function!");
        }
        Token::Identifier(name) => {
            // Logic for function calls: print(name)
            println!("I found a call to {}", name);
        }
        _ => println!("Syntax Error: I don't know what this is."),
    }
}
