fn main() {
    println!("Hello, world!");
    RCAS::parse("x^2");
}

#[derive(Debug, PartialEq, Eq)]
enum Symbols {
    Variable(String),
    Constant(usize),
    Operator(Operations),
}
#[derive(Debug, PartialEq, Eq)]
enum Operations {
    Add ,
    Subtract,
    Multiply,
    Divide,
    Power,
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

        let bytes = phrase.chars().filter(|x| x.is_whitespace());

        for b in bytes.clone() {
            println!("{}", b);
        }

        let mut sys = vec![];

        for b in bytes{
            if b.is_numeric(){
                sys.push(Symbols::Constant(b as usize))
            }
        }

        Self {
            function: phrase.to_string(),
            output: "".to_string(),
        }
    }
}
