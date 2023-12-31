use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::RangeInclusive,
    rc::Rc,
    str::FromStr,
};

#[derive(Debug)]
struct Ratings(HashMap<String, u32>);

impl Ratings {
    fn sum_of_ratings(&self) -> u32 {
        self.0.values().sum()
    }
}

impl FromStr for Ratings {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rest = &input[1..input.len() - 1];

        let mut map = HashMap::new();
        for part in rest.split(",") {
            let (rating, value) = part.split_once("=").unwrap();

            map.insert(rating.to_owned(), value.parse().unwrap());
        }

        Ok(Ratings(map))
    }
}

#[derive(Debug)]
enum Cmp {
    LT,
    GT,
}

impl Cmp {
    fn evaluate(&self, a: u32, b: u32) -> bool {
        match self {
            Self::LT => a < b,
            Self::GT => a > b,
        }
    }

    fn branches(&self, a: u32) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
        match self {
            Self::LT => (1..=a - 1, a..=4000),
            Self::GT => (a + 1..=4000, 1..=a),
        }
    }
}

#[derive(Debug)]
enum Rule {
    Condition(String, Cmp, u32, Rc<Rule>, Rc<Rule>),
    Accept,
    Reject,
    Redirect(String),
}

impl Rule {
    fn evaluate(&self, part: &Ratings) -> RuleEvaluation {
        match self {
            Self::Accept => RuleEvaluation::Accepted,
            Self::Reject => RuleEvaluation::Rejected,
            Self::Redirect(s) => RuleEvaluation::Redirected(s.clone()),
            Self::Condition(rating, cmp, n, left, right) => {
                let rating = part.0.get(rating).unwrap();
                if cmp.evaluate(*rating, *n) {
                    left.evaluate(part)
                } else {
                    right.evaluate(part)
                }
            }
        }
    }
}

fn assert_string<'a>(input: &'a str, s: &str) -> Result<(String, &'a str), String> {
    if &input[0..s.len()] == s {
        Ok((s.to_string(), &input[s.len()..]))
    } else {
        Err(format!("can't find '{}' in '{}'", s, input))
    }
}

fn parse_label(input: &str) -> Result<(String, &str), String> {
    let label = input
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .collect::<String>();

    if label.is_empty() {
        Err(format!(
            "couldn't find label, found '{}' instead",
            &input[0..1]
        ))
    } else {
        let len = label.len();
        Ok((label, &input[len..]))
    }
}

fn parse_cmp(input: &str) -> Result<(Cmp, &str), String> {
    let cmp = match input.get(..1) {
        Some(">") => Cmp::GT,
        Some("<") => Cmp::LT,
        _ => return Err(format!("expected > or <, got '{}'", &input[0..])),
    };

    Ok((cmp, &input[1..]))
}

fn parse_number(input: &str) -> Result<(u32, &str), String> {
    let number_text = input
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    let number: u32 = number_text
        .parse()
        .map_err(|_| format!("couldn't parse number"))?;

    Ok((number, &input[number_text.len()..]))
}

fn parse_rule(input: &str) -> Result<(Rule, &str), String> {
    fn parse_condition(input: &str) -> Result<(Rule, &str), String> {
        let (label, rest) = parse_label(input)?;
        let (cmp, rest) = parse_cmp(rest)?;
        let (n, rest) = parse_number(rest)?;

        let (_, rest) = assert_string(rest, ":")?;
        let (true_case, rest) = parse_rule(rest)?;
        let (_, rest) = assert_string(rest, ",")?;
        let (false_case, rest) = parse_rule(rest)?;

        Ok((
            Rule::Condition(label, cmp, n, Rc::new(true_case), Rc::new(false_case)),
            rest,
        ))
    }
    fn parse_accept(input: &str) -> Result<(Rule, &str), String> {
        assert_string(input, "A").map(|(_, rest)| (Rule::Accept, rest))
    }
    fn parse_reject(input: &str) -> Result<(Rule, &str), String> {
        assert_string(input, "R").map(|(_, rest)| (Rule::Reject, rest))
    }
    fn parse_redirect(input: &str) -> Result<(Rule, &str), String> {
        parse_label(input).map(|(s, rest)| (Rule::Redirect(s), rest))
    }

    let (rule, rest) = parse_condition(input)
        .or(parse_accept(input))
        .or(parse_reject(input))
        .or(parse_redirect(input))?;

    Ok((rule, rest))
}

#[derive(Debug)]
struct Workflow {
    label: String,
    rule: Rule,
}

enum RuleEvaluation {
    Accepted,
    Rejected,
    Redirected(String),
}

impl Workflow {
    fn evaluate(&self, part: &Ratings) -> RuleEvaluation {
        self.rule.evaluate(part)
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (label, rest) = input.split_once("{").unwrap();

        let (rule, _) = parse_rule(&rest[..rest.len() - 1]).unwrap();

        Ok(Workflow {
            label: label.to_owned(),
            rule,
        })
    }
}

#[derive(Debug)]
struct Puzzle {
    workflows: Vec<Workflow>,
    parts: Vec<Ratings>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (workflows, parts) = input.split_once("\n\n").unwrap();

        let workflows = workflows
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<_>>();
        let parts = parts.lines().map(|line| line.parse().unwrap()).collect();

        Ok(Puzzle { workflows, parts })
    }
}

fn part_1(puzzle: &Puzzle) -> u32 {
    // find "in" rule
    let in_workflow = puzzle.workflows.iter().find(|w| w.label == "in").unwrap();

    let mut accepted_parts = vec![];

    for part in puzzle.parts.iter() {
        let mut workflow = in_workflow;

        loop {
            match workflow.evaluate(part) {
                RuleEvaluation::Accepted => {
                    accepted_parts.push(part);
                    break;
                }
                RuleEvaluation::Rejected => {
                    break;
                }
                RuleEvaluation::Redirected(label) => {
                    workflow = puzzle.workflows.iter().find(|w| w.label == label).unwrap();
                }
            }
        }
    }

    accepted_parts.iter().map(|p| p.sum_of_ratings()).sum()
}

/// For each rule retuns the possible part combinations represented as a map from rating to set of
/// possible values.
fn combinations(rule: &Rule, workflows: &[Workflow]) -> Vec<PossibleParts> {
    match rule {
        Rule::Condition(label, cmp, n, left_branch, right_branch) => {
            let left = combinations(left_branch, workflows);
            let right = combinations(right_branch, workflows);

            let (left_range, right_range) = cmp.branches(*n);

            let mut result = Vec::with_capacity(left.len() + right.len());
            for l in left {
                result.push(l.intersect(label, &left_range));
            }
            for r in right {
                result.push(r.intersect(label, &right_range));
            }
            result
        }
        Rule::Accept => vec![all_combinations()],
        Rule::Reject => vec![no_combinations()],
        Rule::Redirect(label) => {
            let w = workflows.iter().find(|w| w.label == *label).unwrap();

            combinations(&w.rule, workflows)
        }
    }
}

struct PossibleParts(HashMap<String, HashSet<u32>>);

impl PossibleParts {
    fn intersect(mut self, label: &str, range: &RangeInclusive<u32>) -> Self {
        let values = self.0.get_mut(label).unwrap();
        values.retain(|n| range.contains(n));
        self
    }

    fn combinations_count(&self) -> u64 {
        self.0.values().map(|v| v.len() as u64).product()
    }
}

fn all_combinations() -> PossibleParts {
    let all: HashSet<u32> = (1..=4000).collect();
    PossibleParts(HashMap::from([
        ("x".to_string(), all.clone()),
        ("m".to_string(), all.clone()),
        ("a".to_string(), all.clone()),
        ("s".to_string(), all.clone()),
    ]))
}

fn no_combinations() -> PossibleParts {
    let none = HashSet::new();
    PossibleParts(HashMap::from([
        ("x".to_string(), none.clone()),
        ("m".to_string(), none.clone()),
        ("a".to_string(), none.clone()),
        ("s".to_string(), none.clone()),
    ]))
}

fn part_2(puzzle: &Puzzle) -> u64 {
    let in_workflow = puzzle.workflows.iter().find(|w| w.label == "in").unwrap();

    combinations(&in_workflow.rule, &puzzle.workflows)
        .iter()
        .map(|p| p.combinations_count())
        .sum()
    //combinations.iter().map(|p| p.combinations_count()).sum()
}

fn main() {
    let input = read_to_string("inputs/day19.txt").expect("file not found");

    let puzzle: Puzzle = input.parse().unwrap();

    println!("Part 1: {}", part_1(&puzzle));
    println!("Part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_parts_intersection() {
        let mut parts = all_combinations();
        parts = parts.intersect("a", &(1..=200));
        parts = parts.intersect("x", &(1..=200));
        parts = parts.intersect("m", &(1..=200));
        parts = parts.intersect("s", &(1..=100));

        assert_eq!(parts.combinations_count(), 200 * 200 * 200 * 100);
    }

    #[test]
    fn test_combining_conditions() {
        let workflows = vec![Workflow {
            label: "in".to_string(),
            rule: Rule::Condition(
                "a".to_string(),
                Cmp::LT,
                200,
                Rc::new(Rule::Accept),
                Rc::new(Rule::Condition(
                    "x".to_string(),
                    Cmp::GT,
                    300,
                    Rc::new(Rule::Reject),
                    Rc::new(Rule::Accept),
                )),
            ),
        }];
        let combinations = combinations(&workflows[0].rule, &workflows);

        assert_eq!(combinations.len(), 3);
        assert_eq!(combinations[1].combinations_count(), 0);
        assert_eq!(
            combinations[0].combinations_count(),
            4000 * 4000 * 4000 * 199
        );
        assert_eq!(
            combinations[2].combinations_count(),
            4000 * 4000 * (4000 - 199) * 300
        );
    }
}
