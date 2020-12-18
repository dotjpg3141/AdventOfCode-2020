use std::{error::Error, iter::Peekable};

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./input/input18.txt")?;

    let result = input
        .lines()
        .map(|s| parse_expression_from_str(s, &|_| 1))
        .map(|expr| expr.eval())
        .sum::<i64>();
    println!("Day 18a: {}", result);

    let result = input
        .lines()
        .map(|s| parse_expression_from_str(s, &|binary| if binary == "+" { 2 } else { 1 }))
        .map(|expr| expr.eval())
        .sum::<i64>();
    println!("Day 18a: {}", result);

    Ok(())
}

enum Expression {
    Literal(i64),
    Adition(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn eval(&self) -> i64 {
        match self {
            Expression::Literal(literal) => *literal,
            Expression::Adition(lhs, rhs) => lhs.as_ref().eval() + rhs.as_ref().eval(),
            Expression::Multiplication(lhs, rhs) => lhs.as_ref().eval() * rhs.as_ref().eval(),
        }
    }
}

fn parse_expression_from_str(s: &str, precedence_table: &impl Fn(&str) -> usize) -> Expression {
    let s = s.replace("(", " ( ").replace(")", " ) ");
    let mut tokens = s.split(' ').filter(|s| !s.is_empty()).peekable();
    parse_expression(&mut tokens, precedence_table)
}

fn parse_expression<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a str>>,
    precedence_table: &impl Fn(&str) -> usize,
) -> Expression {
    parse_binary(tokens, precedence_table, 0)
}

fn parse_binary<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a str>>,
    precedence_table: &impl Fn(&str) -> usize,
    min_precedence: usize,
) -> Expression {
    let mut lhs = parse_primary(tokens, precedence_table);
    while let Some(&token) = tokens.peek() {
        let binary = match token {
            "+" => Expression::Adition,
            "*" => Expression::Multiplication,
            _ => break,
        };

        let precedence = precedence_table(token);
        if precedence <= min_precedence {
            break;
        }

        tokens.next().unwrap();

        let rhs = parse_binary(tokens, precedence_table, precedence);
        lhs = binary(Box::new(lhs), Box::new(rhs));
    }
    lhs
}

fn parse_primary<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a str>>,
    precedence_table: &impl Fn(&str) -> usize,
) -> Expression {
    match *tokens.peek().expect("unexpected end of file") {
        "(" => {
            tokens.next().unwrap();
            let expression = parse_expression(tokens, precedence_table);
            assert!(tokens.next() == Some(")"));
            expression
        }
        literal => {
            tokens.next().unwrap();
            Expression::Literal(literal.parse().expect("literal number"))
        }
    }
}
