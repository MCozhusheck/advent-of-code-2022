pub fn part1(input: String) -> i32 {
    let mut cycle = 0;
    let mut register = 1;
    let mut register_values: Vec<i32> = vec![];
    let cycles_strength = [20, 60, 100, 140, 180, 220];
    input.split('\n').for_each(|line| {
        let words = line.split(' ').collect::<Vec<&str>>();
        match words[0] {
            "noop" => {
                println!("noop - cycle: {}, register: {}", cycle, register);
                cycles_strength.iter().for_each(|x| {
                    if ((cycle + 1)..=(cycle + 1)).contains(x) {
                        println!("noop push {:?} {}", cycle..(cycle + 2), x);
                        register_values.push(register)
                    }
                });
                cycle += 1;
            }
            "addx" => {
                println!("addx - cycle: {}, register: {}", cycle, register);
                cycles_strength.iter().for_each(|x| {
                    if ((cycle + 1)..=(cycle + 2)).contains(x) {
                        println!("addx push {:?} {}", cycle..(cycle + 2), x);
                        register_values.push(register)
                    }
                });
                register += words[1].parse::<i32>().unwrap();
                cycle += 2;
            }
            _ => panic!("Unknown operation"),
        };
    });
    println!("register values: {:?}", register_values);
    return register_values
        .iter()
        .zip(cycles_strength.iter())
        .fold(0, |acc, (left, right)| acc + left * right);
}
