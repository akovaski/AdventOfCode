use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let expressions: Vec<Expression> = lines.iter().map(|l| parse_expression(l)).collect();
    let evaluations: Vec<i64> = expressions.iter().map(|expr| evaluate_expression(expr)).collect();

    println!("Sum of expressions: {}", evaluations.iter().sum::<i64>());

    Ok(())
}

fn evaluate_expression(exp: &Expression) -> i64 {
    match exp {
        Expression::Literal(x) => *x,
        Expression::Mul(v) => v.iter().map(|expr| evaluate_expression(expr)).product(),
        Expression::Add(v) => v.iter().map(|expr| evaluate_expression(expr)).sum(),
    }
}

#[derive(Debug)]
enum Expression {
    Literal(i64),
    Mul(Vec<Expression>),
    Add(Vec<Expression>),
}

fn parse_add_expression(line: &str) -> Expression {
    let unit = eat_add_unit(line);
    if unit.len() == line.len() {
        match line.chars().nth(0).unwrap() {
            '0'..='9' => Expression::Literal(line.parse().unwrap()),
            '(' => parse_expression(&line[1..line.len()-1]),
            _ => unreachable!(),
        }
    } else {
        let mut units = Vec::new();
        let mut pos = 0;
        while pos < line.len() {
            let new_unit = eat_add_unit(&line[pos..]);
            pos += new_unit.len();
            pos += parse_possible_operator(&line[pos..]).len();
            units.push(parse_expression(new_unit));
        }
        Expression::Add(units)
    }
}

fn parse_expression(line: &str) -> Expression {
    let mut units = Vec::new();
    let mut pos = 0;
    while pos < line.len() {
        let new_unit = eat_mul_unit(&line[pos..]);
        pos += new_unit.len();
        pos += parse_possible_operator(&line[pos..]).len();
        units.push(parse_add_expression(new_unit));
    }
    Expression::Mul(units)
}

fn parse_possible_operator(line: &str) -> &str {
    let mut i = 0;
    while i < line.len() {
        match &line[i..=i].chars().nth(0).unwrap() {
            ' '|'+'|'*' => i += 1,
            _ => break,
        }
    }
    &line[0..i]
}

fn eat_mul_unit(line: &str) -> &str {
    let mut end = line.len();
    let mut i = 0;
    while i < line.len() {
        match &line[i..=i].chars().nth(0).unwrap() {
            '*' => {
                end = i - 1;
                break;
            }
            '(' => {
                let paren_unit = eat_parens(&line[i..]);
                i += paren_unit.len();
            }
            '0'..='9' | ' ' | '+' => i += 1,
            _ => unreachable!(),
        }
    }
    &line[0..end]
}

fn eat_add_unit(line: &str) -> &str {
    match line.chars().nth(0).unwrap() {
        '0'..='9' => {
            let mut end = line.len();
            for i in 0..line.len() {
                if &line[i..=i] == "+" {
                    end = i - 1;
                    break;
                }
            }
            &line[0..end]
        }
        '(' => eat_parens(line),
        _ => unreachable!(),
    }
}

fn eat_parens(line: &str) -> &str {
    let mut num_open_paren = 0;
    let mut close_paren_i = None;
    for (i, c) in line.chars().enumerate() {
        num_open_paren += if c == '(' {
            1
        } else if c == ')' {
            -1
        } else {
            0
        };

        if num_open_paren == 0 {
            close_paren_i = Some(i);
            break;
        }
    }
    let close_paren_i = close_paren_i.unwrap();
    &line[0..=close_paren_i]
}
