use pest::{self,Parser};

fn main() {
    println!("Hello, world!");
    // RCAS::parse("x^2");
}

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CAS;

pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = CAS::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::UnaryExpr => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let child = pair.next().unwrap();
            let child = build_ast_from_term(child);
            parse_unary_expr(op, child)
        }
        Rule::BinaryExpr => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let mut lhs = build_ast_from_term(lhspair);
            let op = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let mut rhs = build_ast_from_term(rhspair);
            let mut retval = parse_binary_expr(op, lhs, rhs);
            loop {
                let pair_buf = pair.next();
                if let Some(op) = pair_buf {
                    lhs = retval;
                    rhs = build_ast_from_term(pair.next().unwrap());
                    retval = parse_binary_expr(op, lhs, rhs);
                } else {
                    return retval;
                }
            }
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Int => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "-" => (-1, &istr[1..]),
                _ => (1, istr),
            };
            let int: i32 = istr.parse().unwrap();
            Node::Int(sign * int)
        }
        Rule::Expr => build_ast_from_expr(pair),
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn parse_unary_expr(pair: pest::iterators::Pair<Rule>, child: Node) -> Node {
    Node::UnaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            _ => unreachable!(),
        },
        child: Box::new(child),
    }
}

fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, lhs: Node, rhs: Node) -> Node {
    Node::BinaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            _ => unreachable!(),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Int(i32),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
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

// impl RCAS {
//     fn parse(phrase: &str) -> Self {
//         if phrase.is_empty() {
//             panic!("No function")
//         }
//
//         let bytes = phrase.chars().filter(|x| x.is_whitespace());
//
//         for b in bytes.clone() {
//             println!("{}", b);
//         }
//
//         let mut sys = vec![];
//
//         for b in bytes{
//             if b.is_numeric(){
//                 sys.push(Symbols::Constant(b as usize))
//             }
//         }
//
//         Self {
//             function: phrase.to_string(),
//             output: "".to_string(),
//         }
//     }
// }
