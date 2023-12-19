use std::io;
use std::collections::HashMap;
use regex::Regex;

enum Op { Lt, Gt }

struct Rule {
    param: usize,
    op: Op,
    amount: usize,
    instruction: String,
}

struct Workflow {
    rules: Vec<Rule>,
    default: String,
}

type Part = [usize; 4];

impl Workflow {
    fn instruction(&self, part: &Part) -> String {
        if let Some(rule) = self.rules.iter().find(|rule| {
            match rule.op {
                Op::Lt => part[rule.param] < rule.amount,
                Op::Gt => part[rule.param] > rule.amount,
            }
        }) {
            rule.instruction.clone()
        } else {
            self.default.clone()
        }
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let blank = lines.iter().position(|l| l.len() == 0).expect("blank line missing");

    let workflow_rx = Regex::new(r"^(\w+)\{(.+),(\w+)\}$").unwrap();
    let rule_rx = Regex::new(r"^(x|m|a|s)(>|<)(\d+):(\w+)$").unwrap();

    let workflows: HashMap<String, Workflow> = lines[0..blank].iter().map(|line| {
        let (_, [name, rules, default]) = workflow_rx.captures(line)
            .expect("invalid workflow")
            .extract();

        let name = name.to_string();
        let default = default.to_string();
        let rules: Vec<Rule> = rules.split(',').map(|s| {
            let (_, [param, op, amount, instruction]) = rule_rx.captures(s)
                .expect("invalid rule")
                .extract();

            let param: usize = match param {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => panic!("unknown param")
            };
            let op: Op = match op { "<" => Op::Lt, ">" => Op::Gt, _ => panic!() };
            let amount = amount.parse::<usize>().unwrap();
            let instruction = instruction.to_string();
            Rule { param, op, amount, instruction }
        }).collect();
        (name, Workflow { rules, default })
    }).collect();

    let part_rx = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();

    let parts: Vec<Part> = lines[(blank + 1)..].iter().map(|line| {
        let (_, [x, m, a, s]) = part_rx.captures(line)
            .expect("invalid part")
            .extract();

        [
            x.parse::<usize>().unwrap(),
            m.parse::<usize>().unwrap(),
            a.parse::<usize>().unwrap(),
            s.parse::<usize>().unwrap(),
        ]
    }).collect();

    let start = workflows.get(&"in".to_string()).expect("in workflow not found");

    let accepted = parts.into_iter().filter(|part| {
        let mut workflow = start;
        loop {
            let instruction = workflow.instruction(part);
            if instruction == "A" { return true }
            if instruction == "R" { return false }

            workflow = workflows.get(&instruction).expect("workflow not found");
        }
    });

    let result: usize = accepted.map(|part| part.iter().sum::<usize>()).sum();
    println!("{result}");
}
