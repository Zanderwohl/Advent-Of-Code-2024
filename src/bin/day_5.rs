use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use itertools::Itertools;
use crate::util::parsing;
use crate::util::parsing::{comma_split, convert_strings_matrix, pipe_split, transpose};
use crate::util::vecstuff::center;

mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = parsing::file_into_vec("files/day_5_input.txt")?;
    let (rules, changes) = parse_input(&lines);
    if let Some(rules) = rules {
        if let Ok(changes) = changes {
            let middles: Vec<Vec<u32>> = changes.iter().map(|changeset| {
                let violated_rules = rule_violations(&changeset, &rules);
                if violated_rules.is_empty() {
                    vec![*center(changeset), 0]
                } else {
                    let relevant = relevant_rules(&changeset, &rules);
                    let relevant = relevant.iter().map(|rule| {(**rule).clone()}).collect();
                    let attempt = shake_da_cocktail(&changeset, &relevant);
                    vec![0, *center(&attempt)]
                }
            }).collect();
            let middles = transpose(&middles).expect("");
            let total_good: u32 = middles[0].iter().sum();
            let total_bad: u32 = middles[1].iter().sum();
            println!("The total of center page numbers for rule-following changes is:\n{}", total_good);
            println!("The total of center page numbers for fixed changes is:\n{}", total_bad);
        } else {
            eprintln!("Couldn't parse changes.")
        }
    } else {
        eprintln!("Couldn't parse rules.");
    }

    Ok(())
}

fn rule_violations<'a>(changeset: &Vec<u32>, rules: &'a Vec<Rule>) -> Vec<&'a Rule> {
    let mut violated_rules: Vec<&Rule> = Vec::new();
    for page in changeset {
        let rules: Vec<&Rule> = rules.iter().filter(|rule| rule.before == *page).collect();

        for rule in rules {
            if !follows_rule(&changeset, &rule) {
                violated_rules.push(rule);
            }
        }
    }
    violated_rules
}

fn relevant_rules<'a>(changeset: &Vec<u32>, rules: &'a Vec<Rule>) -> Vec<&'a Rule> {
    let mut relevant = Vec::new();
    for rule in rules {
        let a = rule.after;
        let b = rule.before;
        let a= changeset.iter().find_position(|x| { **x == a}).into_iter().len() > 0;
        let b= changeset.iter().find_position(|x| **x == b).into_iter().len() > 0;
        if a && b {
            relevant.push(rule)
        }
    }
    relevant
}

fn shake_da_cocktail(changeset: &Vec<u32>, rules: &Vec<Rule>) -> Vec<u32> {
    let mut changeset = changeset.clone();
    let len = changeset.len();
    'a: while rule_violations(&changeset, &rules).len() > 0 {
        for idx in 0..len-2 {
            if wrong_order(changeset[idx], changeset[idx+1], rules) {
                changeset.swap(idx, idx+1);
            }
        }
        for idx in (1..=len-1).rev() {
            if wrong_order(changeset[idx - 1], changeset[idx], rules) {
                changeset.swap(idx, idx-1);
            }
        }
    }
    changeset
}

pub fn wrong_order(first: u32, second: u32, rules: &Vec<Rule>) -> bool {
    //let a= rules.iter().find_position(|rule| { **rule.before == first && **rule.after == second}).into_iter().len() > 0;
    let b= rules.iter().find_position(|rule| { (**rule).after == first && (**rule).before == second}).into_iter().len() > 0;
    b
}

fn fix_changeset(changeset: &Vec<u32>, rules: &Vec<Rule>, violated_rules: &Vec<&Rule>) -> Vec<u32> {
    let mut new_changeset = changeset.clone();
    for rule in violated_rules {
        let (before_index, before_value) = changeset.iter().find_position(|page| **page == rule.before).unwrap();
        let (after_index, after_value) = changeset.iter().find_position(|page| **page == rule.after).unwrap();
        println!("{} at {} falsely preceeds {} at {}", after_value, after_index, before_value, before_index);
        //new_changeset.remove(before_index);
        //new_changeset.insert(after_index, *before_value);
        new_changeset.swap(before_index, after_index)
    }


    new_changeset
}

pub fn parse_input(vec: &Vec<String>) -> (Option<Vec<Rule>>, Result<Vec<Vec<u32>>, ParseIntError>) {
    let capacity = vec.len();
    let mut pairs: Vec<String> = Vec::with_capacity(capacity);
    let mut updates: Vec<String> = Vec::with_capacity(capacity);

    let mut idx: usize = 0;
    'a: loop {
        let line = vec.get(idx);
        if line.is_none() {
            break 'a;
        }
        let line = line.unwrap();
        if line == "" {
            break 'a;
        }
        pairs.push(line.clone());
        idx += 1;
    }
    idx += 1;
    'b: loop {
        let line = vec.get(idx);
        if line.is_none() {
            break 'b;
        }
        let line = line.unwrap();
        if line == "" {
            break 'b;
        }
        updates.push(line.clone());
        idx += 1;
    }

    let pairs = pipe_split(&pairs);
    let requirements = convert_strings_matrix::<u32>(&pairs).map(|x| {
        Rule::structure_matrix(&x)
    }).ok().flatten();
    let updates = comma_split(&updates);
    (requirements, convert_strings_matrix::<u32>(&updates))
}

fn follows_rule(vec: &Vec<u32>, rule: &Rule) -> bool {
    let before_index = vec.iter().find_position(|page| **page == rule.before);
    let after_index = vec.iter().find_position(|page| **page == rule.after);
    match (before_index, after_index) {
        (Some((before_index, _)), Some((after_index, _))) => {
            before_index < after_index
        },
        (_, _) => true,
    }
}

#[derive(Clone)]
struct Rule {
    before: u32,
    after: u32,
}

impl Rule {
    fn structure(vec: &Vec<u32>) -> Option<Self> {
        if vec.len() != 2 {
            return None;
        }
        Some(Rule {
            before: vec[0],
            after: vec[1],
        })
    }

    fn structure_matrix(vec: &Vec<Vec<u32>>) -> Option<Vec<Rule>> {
        vec.into_iter().map(|requirement| {
            Rule::structure(requirement)
        }).collect()
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} before {}]", self.before, self.after)
    }
}

#[cfg(test)]
mod tests {
    use crate::{rule_violations, follows_rule, parse_input, Rule, fix_changeset, relevant_rules, shake_da_cocktail};
    use crate::util::parsing::transpose;
    use crate::util::vecstuff::center;

    const SIMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn simple() {
        let lines: Vec<String> = SIMPLE.split("\n").map(String::from).collect();
        let (pairs, updates) = parse_input(&lines);

        println!("-------------");
        if let Some(pairs) = pairs {
            for req in pairs {
                println!("{}", req);
            }
        }
        println!("-------------");
        if let Ok(updates) = updates {
            for update in updates {
                println!("{:?}", update);
            }
        }
        println!("-------------");
    }

    #[test]
    fn test_follows_requirement() {
        let rule = Rule {
            before: 1,
            after: 2,
        };
        let good = vec![1, 2];
        let bad = vec![2, 1];
        let not_applicable = vec![4, 4];

        assert!(follows_rule(&good, &rule));
        assert!(!follows_rule(&bad, &rule));
        assert!(follows_rule(&not_applicable, &rule));
    }

    #[test]
    fn test_follows_all_rules() {
        let rules = vec![
            Rule {
                before: 1,
                after: 2,
            },
            Rule {
                before: 2,
                after: 3,
            }
        ];
        let good = vec![1, 2, 3];
        let bad = vec![1, 3, 2];
        let not_applicable = vec![4, 4, 2];
        let another_good = vec![4, 1, 2, 3];

        assert!(rule_violations(&good, &rules).is_empty());
        assert!(!rule_violations(&bad, &rules).is_empty());
        assert!(rule_violations(&not_applicable, &rules).is_empty());
        assert!(rule_violations(&another_good, &rules).is_empty());
    }

    #[test]
    fn test_simple() {
        let test_input = "1|2
2|3

1,2,3
1,3,2
4,4,2
4,1,2,3,4";
        let follows_rules_expected = [true, false, true, true];

        let lines = test_input.split("\n").map(|x| x.to_string()).collect();
        let (rules, changes) = parse_input(&lines);
        if let Some(rules) = rules {
            if let Ok(changes) = changes {
                let total: u32 = changes.iter().enumerate().map(|(idx, changeset)| {
                    let follows_rules = rule_violations(&changeset, &rules);
                    println!("#{} {:?} follows rules? {} (expected {})", idx, changeset, follows_rules.is_empty(), follows_rules_expected[idx]);
                    assert_eq!(follows_rules.is_empty(), follows_rules_expected[idx]);
                    if follows_rules.is_empty() { *center(changeset) } else { 0 }
                }).sum();
                println!("The total of center page numbers for rule-following changes is:\n{}", total);
                assert_eq!(total, 8);
            } else {
                eprintln!("Couldn't parse changes.")
            }
        } else {
            eprintln!("Couldn't parse rules.");
        }
    }

    #[test]
    fn test_everything() {
        let test_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let follows_rules_expected = [true, true, true, false, false, false];

        let lines = test_input.split("\n").map(|x| x.to_string()).collect();
        let (rules, changes) = parse_input(&lines);
        if let Some(rules) = rules {
            if let Ok(changes) = changes {
                let total: u32 = changes.iter().enumerate().map(|(idx, changeset)| {
                    let follows_rules = rule_violations(&changeset, &rules);
                    println!("#{} {:?} follows rules? {} (expected {})", idx, changeset, follows_rules.is_empty(), follows_rules_expected[idx]);
                    assert_eq!(follows_rules.is_empty(), follows_rules_expected[idx]);
                    if follows_rules.is_empty() { *center(changeset) } else { 0 }
                }).sum();
                println!("The total of center page numbers for rule-following changes is:\n{}", total);
                assert_eq!(total, 143);
            } else {
                eprintln!("Couldn't parse changes.")
            }
        } else {
            eprintln!("Couldn't parse rules.");
        }
    }

    #[test]
    fn test_fix_changeset() {
        let rules = vec![
            Rule {
                before: 1,
                after: 2,
            },
            Rule {
                before: 2,
                after: 3,
            }
        ];
        let bad = vec![1, 3, 2];

        let violated_rules = rule_violations(&bad, &rules);
        println!("Changeset: {:?}", bad);
        for rule in &violated_rules {
            println!("\t{}", rule);
        }
        let fixed = fix_changeset(&bad, &rules, &violated_rules);
        println!("Fixed changeset: {:?}", fixed);
    }

    #[test]
    pub fn test_fix_test_input() {
        let test_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let corrected_changesets: Vec<Vec<u32>> = vec![
            vec![],
            vec![],
            vec![],
            vec![97,75,47,61,53],
            vec![61,29,13],
            vec![97,75,47,29,13],
        ];

        let lines = test_input.split("\n").map(|line| line.to_string()).collect();
        let (rules, changes) = parse_input(&lines);
        if let Some(rules) = rules {
            if let Ok(changes) = changes {
                let middles: Vec<Vec<u32>> = changes.iter().enumerate().map(|(idx, changeset)| {
                    let violated_rules = rule_violations(&changeset, &rules);
                    if violated_rules.is_empty() {
                        vec![*center(changeset), 0]
                    } else {
                        let relevant = relevant_rules(&changeset, &rules);
                        let relevant = relevant.iter().map(|rule| {(**rule).clone()}).collect();
                        let attempt = shake_da_cocktail(&changeset, &relevant);
                        println!("---");
                        println!("{:?} <- original", changeset);
                        println!("{:?} <- correct", corrected_changesets[idx]);
                        println!("{:?} <- attempt", attempt);
                        println!("---");

                        let mut correct = true;
                        for i in 0..changeset.len() {
                            if corrected_changesets[idx][i] != attempt[i] {
                                correct = false;
                            }
                        }
                        if !correct {
                            for rule in &relevant {
                                println!("{}", rule);
                            }
                        }
                        assert!(correct);

                        vec![0, *center(&attempt)]
                    }
                }).collect();
                let middles = transpose(&middles).expect("");
                let total_good: u32 = middles[0].iter().sum();
                let total_bad: u32 = middles[1].iter().sum();
                assert_eq!(total_bad, 123);
                println!("The total of center page numbers for rule-following changes is:\n{}", total_good);
                println!("The total of center page numbers for fixed changes is:\n{}", total_bad);
            }
        }
    }
}
