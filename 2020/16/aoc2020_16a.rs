use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let (rules, _, nearby_tickets) = parse_input(&lines);

    let mut invalids: Vec<i32> = Vec::new();

    for ticket in &nearby_tickets {
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
                invalids.push(val);
            }
                
        }
    }

    println!("error rate: {}", invalids.iter().sum::<i32>());

    Ok(())
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(i32, i32)>,
}

fn parse_input(lines: &[String]) -> (Vec<Rule>, Vec<i32>, Vec<Vec<i32>>) {
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
                        let range_split: Vec<i32> = rs.split('-').map(|ns| ns.parse().unwrap()).collect();
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
