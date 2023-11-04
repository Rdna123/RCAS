fn main() {
    println!("Hello, world!");
    RCAS::parse("x^2");
}

#[derive(Debug, PartialEq, Eq)]
enum Symbols {
    Variable(String),
    Constanct(usize),
    Operator(Operations),
}
#[derive(Debug, PartialEq, Eq)]
enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct RCAS {
    function: String,
    output: String,
}

impl RCAS {
    fn parse(phrase: &str) -> Self {
        if phrase.is_empty() {
            panic!("No function")
        }

        let bytes = phrase.split_whitespace();

        for b in bytes {
            println!("{}", b);
        }

        Self {
            function: phrase.to_string(),
            output: "".to_string(),
        }
    }
}
