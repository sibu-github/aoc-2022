// use num_bigint::BigUint;

#[derive(Debug, Clone)]
struct Item(u128);

impl Item {
    fn new(val: u128) -> Self {
        Self(val)
    }
}
impl From<u128> for Item {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default, Clone)]
enum Ops {
    #[default]
    Noop,
    Add(u128),
    Mul(u128),
    Div(u128),
    Modulo(u128),
    Square,
}

impl Ops {
    fn execute(&self, val: u128) -> u128 {
        match self {
            Self::Noop => val,
            Self::Add(v) => val + v,
            Self::Mul(v) => val * v,
            Self::Div(v) => val / v,
            Self::Modulo(v) => val % v,
            Self::Square => val * val,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    id: u128,
    items: Vec<Item>,
    inspection_count: u128,
    before_inspection: Ops,
    after_inspection: Ops,
    throw_test: Ops,
    throw_when_true: u128,
    throw_when_false: u128,
}

impl Monkey {
    fn new(chunk: &[&str]) -> Self {
        assert!(chunk.len() >= 6);
        let id = chunk[0]
            .strip_prefix("Monkey")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .trim()
            .parse::<u128>()
            .unwrap();
        let items = chunk[1]
            .trim()
            .strip_prefix("Starting items:")
            .unwrap()
            .split(',')
            .map(|s| s.trim().parse::<u128>().unwrap())
            .map(Item::from)
            .collect();
        let mut s = chunk[2]
            .trim()
            .strip_prefix("Operation:")
            .unwrap()
            .trim()
            .strip_prefix("new = old")
            .unwrap()
            .trim()
            .split_whitespace();
        let op = s.next().unwrap();
        let val = s.next().unwrap();
        let before_inspection = if op == "*" {
            if val == "old" {
                Ops::Square
            } else {
                let val = val.parse().unwrap();
                Ops::Mul(val)
            }
        } else if op == "+" {
            let val = val.parse().unwrap();
            Ops::Add(val)
        } else {
            Ops::default()
        };
        let after_inspection = Ops::Div(3);
        let throw_test = chunk[3]
            .trim()
            .strip_prefix("Test:")
            .unwrap()
            .trim()
            .strip_prefix("divisible by")
            .unwrap()
            .trim()
            .parse::<u128>()
            .unwrap();
        let throw_test = Ops::Modulo(throw_test);
        let throw_when_true = chunk[4]
            .trim()
            .strip_prefix("If true:")
            .unwrap()
            .trim()
            .strip_prefix("throw to monkey")
            .unwrap()
            .trim()
            .parse::<u128>()
            .unwrap();
        let throw_when_false = chunk[5]
            .trim()
            .strip_prefix("If false:")
            .unwrap()
            .trim()
            .strip_prefix("throw to monkey")
            .unwrap()
            .trim()
            .parse::<u128>()
            .unwrap();
        Self {
            id,
            items,
            inspection_count: 0,
            before_inspection,
            after_inspection,
            throw_test,
            throw_when_true,
            throw_when_false,
        }
    }

    fn inspect_and_throw(&mut self) -> Option<(u128, Item)> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items.remove(0);
        let worry_level = item.0;
        self.inspection_count += 1;
        let worry_level = self.before_inspection.execute(worry_level);
        let worry_level = self.after_inspection.execute(worry_level);
        let item = Item::new(worry_level);
        if self.throw_test.execute(worry_level) == 0 {
            Some((self.throw_when_true, item))
        } else {
            Some((self.throw_when_false, item))
        }
    }

    fn receive_throw(&mut self, item: Item) {
        self.items.push(item)
    }
}

#[derive(Debug)]
struct MonkeyBusiness {
    round_no: u128,
    curr_monkey: u128,
    total_monkeys: u128,
    monkeys: Vec<Monkey>,
}

impl MonkeyBusiness {
    fn new(monkeys: Vec<Monkey>) -> Self {
        Self {
            round_no: 0,
            curr_monkey: 0,
            total_monkeys: monkeys.len() as u128,
            monkeys,
        }
    }

    fn execute_round(&mut self) {
        // println!("Executing round {}", self.round_no);
        loop {
            self.execute_monkeys_turn();
            self.curr_monkey += 1;
            if self.curr_monkey == self.total_monkeys {
                self.curr_monkey = 0;
                break;
            }
        }
        // println!("Finished round {}", self.round_no);
        self.round_no += 1;
    }

    fn execute_monkeys_turn(&mut self) {
        loop {
            let curr_monkey = self
                .monkeys
                .iter_mut()
                .find(|m| m.id == self.curr_monkey)
                .unwrap();
            if let Some((id, item)) = curr_monkey.inspect_and_throw() {
                let monkey = self.monkeys.iter_mut().find(|m| m.id == id).unwrap();
                monkey.receive_throw(item);
            } else {
                break;
            }
        }
    }
}

fn parse_input(s: &str) -> Vec<Monkey> {
    let s = s.lines().collect::<Vec<_>>();
    s.chunks(7).map(Monkey::new).collect()
}

fn main() {
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).unwrap();
    let monkeys = parse_input(&content);
    {
        let mut monkey_business = MonkeyBusiness::new(monkeys.clone());
        while monkey_business.round_no < 20 {
            monkey_business.execute_round();
        }
        let mut inspection_counts = monkey_business
            .monkeys
            .iter()
            .map(|m| m.inspection_count)
            .collect::<Vec<_>>();
        inspection_counts.sort_unstable();
        let t1 = inspection_counts.pop().unwrap();
        let t2 = inspection_counts.pop().unwrap();
        let level_of_monkey_business = t1 * t2;
        println!("{level_of_monkey_business}");
    }
    // ------------------------------------
    {
        let mut monkey_business = MonkeyBusiness::new(monkeys);
        let val = monkey_business
            .monkeys
            .iter()
            .map(|m| match m.throw_test {
                Ops::Modulo(v) => v,
                _ => 1,
            })
            .product::<u128>();
        monkey_business
            .monkeys
            .iter_mut()
            .for_each(|m| m.after_inspection = Ops::Modulo(val));
        while monkey_business.round_no < 10000 {
            monkey_business.execute_round();
        }
        let mut inspection_counts = monkey_business
            .monkeys
            .iter()
            .map(|m| m.inspection_count)
            .collect::<Vec<_>>();
        inspection_counts.sort_unstable();
        let t1 = inspection_counts.pop().unwrap();
        let t2 = inspection_counts.pop().unwrap();
        let level_of_monkey_business = t1 * t2;
        println!("{level_of_monkey_business}");
    }
}
