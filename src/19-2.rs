use std::io;
use std::cmp;
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

struct WorkflowMap {
    map: HashMap<String, Workflow>,
}

#[derive(Clone)]
struct Constraint {
    min: [usize; 4],
    max: [usize; 4],
}

impl Constraint {
    // Restrict given constraint by rule into then-constraint and else-constraint
    fn restrict(&self, rule: &Rule) -> (Option<Constraint>, Option<Constraint>) {
        let rule_overlaps = match rule.op {
            Op::Gt => self.min[rule.param] <= rule.amount && rule.amount < self.max[rule.param],
            Op::Lt => self.min[rule.param] < rule.amount && rule.amount <= self.max[rule.param]
        };

        if rule_overlaps {
            let mut then_con = self.clone();
            let mut else_con = self.clone();
            match rule.op {
                Op::Gt => {
                    then_con.min[rule.param] = cmp::max(then_con.min[rule.param], rule.amount + 1);
                    else_con.max[rule.param] = cmp::min(else_con.max[rule.param], rule.amount);
                },
                Op::Lt => {
                    then_con.max[rule.param] = cmp::min(then_con.max[rule.param], rule.amount - 1);
                    else_con.min[rule.param] = cmp::max(else_con.min[rule.param], rule.amount);
                }
            }

            let then_con = if then_con.min[rule.param] <= then_con.max[rule.param] { Some(then_con) } else { None };
            let else_con = if else_con.min[rule.param] <= else_con.max[rule.param] { Some(else_con) } else { None };

            (then_con, else_con)
        } else {
            (None, Some(self.clone()))
        }
    }

    // Check if two constraints overlap on all params
    fn overlaps(&self, other: &Constraint) -> bool {
        (0..4).all(|i| self.min[i] <= other.max[i] && self.max[i] >= other.min[i])
    }

    // Merge two overlapping constraints.
    // Undefined behavior when constraints don't overlap.
    fn merge(&self, other: Constraint) -> Constraint {
        let mut min = [0; 4];
        let mut max = [0; 4];

        for i in 0..4 {
            min[i] = cmp::min(self.min[i], other.min[i]);
            max[i] = cmp::max(self.max[i], other.max[i]);
        }

        Constraint { min: min, max: max }
    }

    // Merge constraint into given union of constraints
    fn merge_with(&self, union: Vec<Constraint>) -> Vec<Constraint> {
        if let Some(pos) = union.iter().position(|other| self.overlaps(other)) {
            let overlapping = union[pos].clone();

            let mut union = union;
            union.remove(pos);

            self.merge(overlapping).merge_with(union)
        } else {
            let mut result = union;
            result.push(self.clone());
            result
        }
    }

    fn merge_unions(union1: Vec<Constraint>, union2: Vec<Constraint>) -> Vec<Constraint> {
        let mut result = union1;
        for con in union2 {
            result = con.merge_with(result);
        }
        result
    }
}

impl WorkflowMap {
    // Returns union of constraints covering all possible matches for specified workflow.
    // Overlapping constraints are merged to prevent possibilities being counted multiple times.
    fn apply_constraint(&self, con: Constraint, workflow_name: &String) -> Vec<Constraint> {
        if workflow_name == "A" { return vec![con] }
        if workflow_name == "R" { return vec![] }

        let workflow = self.map.get(workflow_name).expect("workflow not found");

        let mut remaining = Some(con);
        let mut result: Vec<Constraint> = vec![];

        // Iterate through all rules or until exausted the original constraint
        for rule in workflow.rules.iter() {
            if let Some(rem) = remaining {
                let (then_con, else_con) = rem.restrict(rule);

                if let Some(then_con) = then_con {
                    result = Constraint::merge_unions(result, self.apply_constraint(then_con, &rule.instruction));
                }
                remaining = else_con;
            } else {
                break
            }
        }

        if let Some(rem) = remaining {
            result = Constraint::merge_unions(result, self.apply_constraint(rem, &workflow.default));
        }

        result
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let blank = lines.iter().position(|l| l.len() == 0).expect("blank line missing");

    let workflow_rx = Regex::new(r"^(\w+)\{(.+),(\w+)\}$").unwrap();
    let rule_rx = Regex::new(r"^(x|m|a|s)(>|<)(\d+):(\w+)$").unwrap();

    let map: HashMap<String, Workflow> = lines[0..blank].iter().map(|line| {
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

    let map = WorkflowMap { map };
    let c = Constraint { min: [1; 4], max: [4000; 4] };
    let union = map.apply_constraint(c, &"in".to_string());

    let result: usize = union.into_iter().map(|u|{
        u.max.iter().zip(u.min.iter()).map(|(max, min)| max - min + 1).fold(1, |acc, a| acc * a)
    }).sum();

    println!("{result}");
}
