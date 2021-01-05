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
        Expression::Mul(x, y) => evaluate_expression(x) * evaluate_expression(y),
        Expression::Add(x, y) => evaluate_expression(x) + evaluate_expression(y),
    }
}

#[derive(Debug)]
enum Expression {
    Literal(i64),
    Mul(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
}

fn parse_expression(line: &str) -> Expression {
    let last_expr_str = match line.chars().last().unwrap() {
        '0'..='9' => {
            line.split(' ').last().unwrap()
        }
        ')' => {
            let mut num_open_paren = 0;
            let mut opening_paren_i = None;
            let mut i = line.len() -1;
            for c in line.chars().rev() {
                num_open_paren += if c == ')' { 1 } else if c == '(' { -1 } else { 0 };
                if num_open_paren == 0 {
                    opening_paren_i = Some(i);
                    break;
                }
                i -= 1;
            }
            let opening_paren_i = opening_paren_i.unwrap();
            &line[opening_paren_i..]
        }
        _ => unreachable!(),
    };

    if last_expr_str.len() == line.len() {
        if line.chars().next().unwrap() != '(' {
            Expression::Literal(line.parse().unwrap())
        } else {
            parse_expression(&line[1..line.len()-1])
        }
    } else {
        let operator_pos = line.len() - last_expr_str.len() - 2;
        let operator = line.chars().nth(operator_pos).unwrap();
        let remaining_expression = &line[..operator_pos-1];
        match operator {
            '*' => Expression::Mul(
                Box::new(parse_expression(last_expr_str)),
                Box::new(parse_expression(remaining_expression))),
            '+' => Expression::Add(
                Box::new(parse_expression(last_expr_str)),
                Box::new(parse_expression(remaining_expression))),
            _ => unreachable!(),
        }
    }
}
