#[derive(Debug, Clone)]
struct Stack {
    stack_no: usize,
    crates: Vec<char>,
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_stack_data(s: &&str) -> Vec<String> {
    let mut data = vec![];
    let mut idx = 0;
    while idx < s.len() {
        let d = &s[idx..idx + 3];
        let d = d.trim().replace('[', "");
        let d = d.trim().replace(']', "");
        data.push(d);
        idx += 4;
    }
    data
}

fn get_all_stacks(lines: &[&str]) -> Vec<Stack> {
    let mut stack_data = lines
        .iter()
        .take_while(|s| !s.is_empty())
        .map(parse_stack_data)
        .collect::<Vec<_>>();
    let stack_numbers = stack_data.pop().unwrap();
    let stack_numbers = stack_numbers
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    stack_data.reverse();
    let mut all_stacks = vec![];
    for (idx, &stack_no) in stack_numbers.iter().enumerate() {
        let crates = stack_data
            .iter()
            .map(|v| &v[idx])
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().next().unwrap())
            .collect::<Vec<_>>();
        let stack = Stack { stack_no, crates };
        all_stacks.push(stack);
    }

    all_stacks
}

fn parse_move_data(s: &&str) -> Move {
    let s = s.strip_prefix("move ").unwrap();
    let (count, rest) = s.split_once(' ').unwrap();
    let count = count.parse::<usize>().unwrap();
    let s = rest.strip_prefix("from ").unwrap();
    let (from, rest) = s.split_once(' ').unwrap();
    let from = from.parse::<usize>().unwrap();
    let s = rest.strip_prefix("to ").unwrap();
    let to = s.parse::<usize>().unwrap();
    Move { count, from, to }
}

fn get_all_moves(lines: &[&str]) -> Vec<Move> {
    lines
        .iter()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(parse_move_data)
        .collect()
}

fn perform_moves(mut stacks: Vec<Stack>, moves: &[Move]) -> Vec<Stack> {
    for mv in moves {
        for _ in 0..mv.count {
            let from_stack = stacks
                .iter_mut()
                .find(|stack| stack.stack_no == mv.from)
                .unwrap();
            let item = from_stack.crates.pop().unwrap();
            let to_stack = stacks
                .iter_mut()
                .find(|stack| stack.stack_no == mv.to)
                .unwrap();
            to_stack.crates.push(item);
        }
    }
    stacks
}

fn perform_moves_9001(mut stacks: Vec<Stack>, moves: &[Move]) -> Vec<Stack> {
    for mv in moves {
        let from_stack = stacks
            .iter_mut()
            .find(|stack| stack.stack_no == mv.from)
            .unwrap();
        let mut temp = vec![];
        for _ in 0..mv.count {
            let item = from_stack.crates.pop().unwrap();
            temp.push(item);
        }
        temp.reverse();
        let to_stack = stacks
            .iter_mut()
            .find(|stack| stack.stack_no == mv.to)
            .unwrap();
        for item in temp {
            to_stack.crates.push(item);
        }
    }
    stacks
}

fn top_crates(stacks: &[Stack]) -> String {
    stacks
        .iter()
        .map(|stack| stack.crates.iter().last().cloned().unwrap_or_default())
        .collect()
}

fn main() {
    // let file_name = "test-input.txt";
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).unwrap();
    let lines = content.lines().collect::<Vec<_>>();
    let stacks = get_all_stacks(&lines);
    let moves = get_all_moves(&lines);
    let stacks_9000 = perform_moves(stacks.clone(), &moves);
    let stacks_9001 = perform_moves_9001(stacks, &moves);
    println!("{}", top_crates(&stacks_9000));
    println!("{}", top_crates(&stacks_9001));
}
