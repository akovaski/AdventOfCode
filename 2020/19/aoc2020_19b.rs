use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let (mut rules, messages) = parse_input(&lines);
    rules.insert(8, Rule::Choice(vec![42], vec![42, 8]));
    rules.insert(11, Rule::Choice(vec![42, 31], vec![42, 11, 31]));
    let rules = rules;

    let num_valid_messages = messages.iter().filter(|m| test_message(&rules, m, 0).len() != 0).count();

    println!("Number of valid messages {}", num_valid_messages);

    Ok(())
}

fn test_message<'a>(rules: &HashMap<i32, Rule>, message: &'a str, rid: i32) -> Vec<&'a str> {
    if rid == 0 {
        //println!();
        //println!("test message '{}'", message);
    }
    let rule = rules.get(&rid).unwrap();
    match rule {
        Rule::Literal(c) => {
            assert!(rid != 0);
            if message.len() > 0 && message.chars().nth(0).unwrap() == *c {
                vec![&message[0..=0]]
            } else {
                Vec::new()
            }
        }
        Rule::Seq(seq) => {
            let prefix_strings = test_sequence(rules, message, &seq);
            if rid == 0 {
                prefix_strings.iter().filter(|s| s.len() == message.len()).map(|p| *p).collect()
            } else {
                prefix_strings
            }
        }
        Rule::Choice(seq_a, seq_b) => {
            assert!(rid != 0);
            let mut prefix_strings = Vec::new();
            prefix_strings.extend(test_sequence(rules, message, &seq_a));
            prefix_strings.extend(test_sequence(rules, message, &seq_b));
            prefix_strings
        }
    }
}

fn test_sequence<'a>(rules: &HashMap<i32, Rule>, message: &'a str, seq: &[i32]) -> Vec<&'a str> {
    let mut prefix_strings: Vec<&str> = vec![""];
    for &seq_rule in seq {
        let mut new_prefix_strings = Vec::new();
        for prefix in prefix_strings {
            for suffix in test_message(rules, &message[prefix.len()..], seq_rule) {
                new_prefix_strings.push(&message[0..prefix.len() + suffix.len()]);
            }
        }
        prefix_strings = new_prefix_strings;
        //println!("seqrule {}", seq_rule);
        //dbg!(&prefix_strings);
    }
    prefix_strings
}


#[derive(Debug)]
enum Rule {
    Literal(char),
    Seq(Vec<i32>),
    Choice(Vec<i32>, Vec<i32>),
}

fn parse_input(lines: &[String]) -> (HashMap<i32, Rule>, Vec<&str>) {
    enum ParseMode {
        Rules,
        Messages,
    }

    let mut parse_mode = ParseMode::Rules;
    let mut rules = HashMap::new();
    let mut messages: Vec<&str> = Vec::new();
    for line in lines {
        match parse_mode {
            ParseMode::Rules => {
                if line == "" {
                    parse_mode = ParseMode::Messages;
                } else {
                    let s: Vec<&str> = line.split(": ").collect();
                    assert!(s.len() == 2);
                    let rid: i32 = s[0].parse().unwrap();
                    let rhs: Vec<&str> = s[1].split(" | ").collect();
                    let rule = match rhs.len() {
                        1 => {
                            let rhs: Vec<&str> = rhs[0].split('"').collect();
                            match rhs.len() {
                                1 => {
                                    let s: Vec<i32> = rhs[0].split(' ').map(|rs| rs.parse::<i32>().unwrap()).collect();
                                    Rule::Seq(s)
                                }
                                3 => {
                                    assert!(rhs[1].len() == 1);
                                    Rule::Literal(rhs[1].chars().next().unwrap())
                                }
                                _ => unreachable!(),
                            }
                        }
                        2 => {
                            let a: Vec<i32> = rhs[0].split(' ').map(|rs| rs.parse::<i32>().unwrap()).collect();
                            let b: Vec<i32> = rhs[1].split(' ').map(|rs| rs.parse::<i32>().unwrap()).collect();
                            Rule::Choice(a, b)
                        }
                        _ => unreachable!(),
                    };
                    rules.insert(rid, rule);
                }
            }
            ParseMode::Messages => messages.push(line),
        }
    }
    (rules, messages)
}
