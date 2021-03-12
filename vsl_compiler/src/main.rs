use std::env;
use std::fs;
use std::time::Instant;

mod parser;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let source: String = fs::read_to_string(filename).expect("Failed to open file.");

        let (tokens, mut error): (Vec<parser::lexer::Token>, bool) = parser::lexer::lexer(&source);

        if error {
            let time: f64 = start.elapsed().as_micros() as f64 / 1000000 as f64;
            println!("Program completed in {} seconds.", time);
            return;
        }
        /*
        for token in &tokens {
            println!("{}", token.token_string);
        }
        */

        let mut parser = parser::Parser::new(&tokens);
        error = parser.parse(&tokens);

        if error {
            let time: f64 = start.elapsed().as_micros() as f64 / 1000000 as f64;
            println!("Failed to compile program due to errors.");
            println!("Program completed in {} seconds.", time);
            return;
        }

        parser.output_code();
    }
    let time: f64 = start.elapsed().as_micros() as f64 / 1000000 as f64;
    println!("Program completed in {} seconds.", time);
}
