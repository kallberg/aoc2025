type Expression = (Vec<u64>, bool);

fn parse_operations(input: &str) -> Vec<bool> {
    let mut operations = vec![];

    for line in input.lines() {
        for word in line.split_whitespace() {
            match word {
                "+" => operations.push(true),
                "*" => operations.push(false),
                _ => {
                    break;
                }
            };
        }
    }

    operations
}

fn parse_numbers(input: &str) -> Vec<Vec<u64>> {
    let mut problems: Vec<Vec<u64>> = vec![];

    for line in input.lines() {
        for (x, word) in line.split_whitespace().enumerate() {
            if problems.len() <= x {
                problems.push(vec![])
            }

            let Ok(value) = word.parse::<u64>() else {
                break;
            };

            let problem_values = &mut problems[x];

            problem_values.push(value);
        }
    }

    problems
}

fn eval(expression: Expression) -> u64 {
    let (values, operation) = expression;
    match operation {
        true => values.iter().sum(),
        false => values.iter().product(),
    }
}

pub fn part_1(input: &str) -> String {
    let numbers = parse_numbers(input);
    let operations = parse_operations(input);
    let mut sum: u64 = 0;

    assert!(numbers.len() == operations.len());

    let problems = numbers.into_iter().zip(operations);

    for (values, operation) in problems {
        sum += eval((values, operation));
    }

    sum.to_string()
}

pub fn cephalopod_from_lines(input: &str) -> Vec<Expression> {
    let mut formatted = vec![];

    for line in input.lines() {
        for (x, char) in line.chars().enumerate() {
            if formatted.len() <= x {
                formatted.push(String::new());
            }
            if char == ' ' {
                continue;
            }
            let row = &mut formatted[x];

            row.push(char);
        }
    }

    let mut values: Vec<u64> = vec![];
    let mut operation = true;
    let mut parsed = vec![];

    formatted.reverse();

    for line in &mut formatted {
        if line.trim().is_empty() {
            continue;
        }
        let mut group_done = false;
        if line.ends_with("+") {
            line.pop();
            group_done = true;
            operation = true;
        } else if line.ends_with("*") {
            line.pop();
            group_done = true;
            operation = false;
        }
        values.push(line.parse().unwrap());
        if group_done {
            parsed.push((values.clone(), operation));
            values.clear();
        }
    }

    parsed
}

pub fn part_2(input: &str) -> String {
    let mut sum = 0;
    for (values, operation) in cephalopod_from_lines(input) {
        let result = eval((values.clone(), operation));
        sum += result;
    }
    sum.to_string()
}
