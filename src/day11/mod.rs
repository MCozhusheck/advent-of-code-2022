use std::collections::VecDeque;

#[derive(Default, Debug, Clone)]
struct Monkey {
    items: VecDeque<i32>,
    op: Operation,
    test: i32,
    dest: (usize, usize),
    total_inspections: i32,
}

#[derive(Default, Debug, Clone)]
enum Operation {
    #[default]
    Noop,
    Mul(i32),
    MulSelf,
    Add(i32),
    AddSelf,
}

pub fn part1(input: String) -> i32 {
    let mut monkeys = vec![];
    let rounds = 20;

    input.split("\n\n").for_each(|data| {
        let mut monkey = Monkey::default();

        data.split("\n").skip(1).for_each(|line| {
            let words = line.trim().split(' ').collect::<Vec<&str>>();
            match words[0] {
                "Starting" => {
                    monkey.items = line
                        .split_once("items: ")
                        .unwrap()
                        .1
                        .split(",")
                        .map(|item| item.trim().parse().unwrap())
                        .collect();
                }
                "Operation:" => {
                    monkey.op = if words[4] == "+" {
                        if words[5] == "old" {
                            Operation::AddSelf
                        } else {
                            Operation::Add(words[5].parse().unwrap())
                        }
                    } else {
                        if words[5] == "old" {
                            Operation::MulSelf
                        } else {
                            Operation::Mul(words[5].parse().unwrap())
                        }
                    }
                }
                "Test:" => {
                    monkey.test = words[3].parse().unwrap();
                }
                "If" => {
                    if words[1] == "true:" {
                        monkey.dest.0 = words[5].parse().unwrap();
                    } else {
                        monkey.dest.1 = words[5].parse().unwrap();
                    }
                }
                _ => panic!("Parsing error!"),
            }
        });

        monkeys.push(monkey);
    });

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let new_item = match &monkeys[i].op {
                    Operation::Noop => panic!("No operation"),
                    Operation::Mul(x) => item * x,
                    Operation::MulSelf => item * item,
                    Operation::Add(x) => item + x,
                    Operation::AddSelf => item + item,
                };

                let dest = if new_item % &monkeys[i].test == 0 {
                    monkeys[i].dest.0
                } else {
                    monkeys[i].dest.1
                };
                monkeys[dest].items.push_back(new_item);
                monkeys[i].total_inspections += 1;
            }
        }
    }

    let mut monkey_business = monkeys
        .iter()
        .map(|m| m.total_inspections)
        .collect::<Vec<i32>>();
    monkey_business.sort_by(|a, b| b.cmp(a));

    return monkey_business[0] * monkey_business[1];
}
