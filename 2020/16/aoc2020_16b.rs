use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let (rules, your_ticket, mut nearby_tickets) = parse_input(&lines);
    nearby_tickets.push(your_ticket.clone());
    let nearby_tickets = nearby_tickets;
    

    let mut valids: Vec<&[i64]> = Vec::new();

    for ticket in &nearby_tickets {
        let mut valid_ticket = true;
        for &val in ticket {
            let mut contained = false;
            'check_ranges: for rule in &rules {
                for &range in &rule.ranges {
                    if val >= range.0 && val <= range.1 {
                        contained = true;
                        break 'check_ranges;
                    }
                }
            }

            if !contained {
                valid_ticket = false;
                break;
            }
        }
        if valid_ticket {
            valids.push(ticket);
        }
    }

    let num_columns = nearby_tickets[0].len();

    let mut all_rules_match = Vec::new();
    for rid in 0..rules.len() {
        all_rules_match.push(rid);
    }
    let mut rule_matches: Vec<Vec<usize>> = Vec::new();
    for _ in 0..num_columns {
        rule_matches.push(all_rules_match.clone());
    }
    for ticket in &valids {
        for (i, &val) in ticket.iter().enumerate() {
            let mut invalid_rules: Vec<usize> = Vec::new();
            for (rid, rule) in rules.iter().enumerate() {
                let mut valid_rule = false;
                for &range in &rule.ranges {
                    if val >= range.0 && val <= range.1 {
                        valid_rule = true;
                        break;
                    }
                }
                if !valid_rule {
                    invalid_rules.push(rid);
                }
            }

            rule_matches[i] = remove_rules(&rule_matches[i], &invalid_rules);
        }
    }

    let mut already_matched: HashMap<usize, ()> = HashMap::new();
    let mut column_rules = vec![99999; num_columns];
    while rule_matches.iter().any(|v| v.len() > 0) {

        let mut rid_match = None;
        for (i, matches) in rule_matches.iter().enumerate() {
            if matches.len() == 1 && !already_matched.contains_key(&matches[0]) {
                rid_match = Some((i, matches[0]));
                already_matched.insert(matches[0],());
            }
        }
        let rid_match = rid_match.unwrap();
        column_rules[rid_match.0] = rid_match.1;

        for matches in &mut rule_matches {
            *matches = remove_rules(matches, &[rid_match.1]);
        }
    }

    let mut vals = Vec::new();
    for (i, rule) in rules.iter().enumerate() {
        if rule.name.split(' ').nth(0).unwrap() == "departure" {
            let column = column_rules.iter().position(|&x| x == i).unwrap();
            let my_val = your_ticket[column];
            vals.push(my_val);
        }
    }

    println!("magic number {}", vals.iter().product::<i64>());

    Ok(())
}

fn remove_rules(existing_rules: &[usize], to_remove: &[usize]) -> Vec<usize> {
    existing_rules.iter()
        .filter(|&&rid| !to_remove.iter().any(|&inv| inv == rid))
        .map(|&rid| rid)
        .collect()
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(i64, i64)>,
}

fn parse_input(lines: &[String]) -> (Vec<Rule>, Vec<i64>, Vec<Vec<i64>>) {
    enum ParseMode {
        Rules,
        YourTicketLabel,
        YourTicket,
        Space2,
        NearbyTicketLabel,
        NearbyTickets,
    }

    let mut parse_mode = ParseMode::Rules;
    let mut rules = Vec::new();
    let mut your_ticket = None;
    let mut nearby_tickets = Vec::new();
    for line in lines {
        match parse_mode {
            ParseMode::Rules => {
                if line == "" {
                    parse_mode = ParseMode::YourTicketLabel;
                } else {
                    let s: Vec<&str> = line.split(": ").collect();
                    assert!(s.len() == 2);
                    let name = s[0].to_string();
                    let mut ranges = Vec::new();
                    let range_strings: Vec<&str> = s[1].split(" or ").collect();
                    assert!(range_strings.len() == 2);
                    for rs in range_strings {
                        let range_split: Vec<i64> = rs.split('-').map(|ns| ns.parse().unwrap()).collect();
                        assert!(range_split.len() == 2);
                        ranges.push((range_split[0], range_split[1]));
                    }
                    rules.push(Rule{name, ranges});
                }
            }
            ParseMode::YourTicketLabel => parse_mode = ParseMode::YourTicket,
            ParseMode::YourTicket => {
                your_ticket = Some(line.split(',').map(|ns| ns.parse().unwrap()).collect());
                parse_mode = ParseMode::Space2;
            }
            ParseMode::Space2 => parse_mode = ParseMode::NearbyTicketLabel,
            ParseMode::NearbyTicketLabel => parse_mode = ParseMode::NearbyTickets,
            ParseMode::NearbyTickets => nearby_tickets.push(
                line.split(',').map(|ns| ns.parse().unwrap()).collect()
            ),
        }
    }
    (rules, your_ticket.unwrap(), nearby_tickets)
}
