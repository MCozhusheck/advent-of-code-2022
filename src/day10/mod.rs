pub fn part1(input: String) -> i32 {
    let mut cycle = 0;
    let mut register = 1;
    let mut register_values: Vec<i32> = vec![];
    let cycles_strength = [20, 60, 100, 140, 180, 220];

    input.split('\n').for_each(|line| {
        let words = line.split(' ').collect::<Vec<&str>>();
        match words[0] {
            "noop" => {
                cycles_strength.iter().for_each(|x| {
                    if x > &cycle && x <= &(cycle + 1) {
                        register_values.push(register)
                    }
                });
                cycle += 1;
            }
            "addx" => {
                cycles_strength.iter().for_each(|x| {
                    if x > &cycle && x <= &(cycle + 2) {
                        register_values.push(register)
                    }
                });
                register += words[1].parse::<i32>().unwrap();
                cycle += 2;
            }
            _ => panic!("Unknown operation"),
        };
    });

    return register_values
        .iter()
        .zip(cycles_strength.iter())
        .fold(0, |acc, (left, right)| acc + left * right);
}

pub fn part2(input: String) {
    let mut cycle = 1;
    let mut position = 0;
    let mut register = 1;
    let mut crt_image: String = "".to_owned();

    input.split('\n').for_each(|line| {
        let words = line.split(' ').collect::<Vec<&str>>();
        println!("cycle: {}, register: {}", cycle, register);
        match words[0] {
            "noop" => {
                if ((register - 1)..=(register + 1)).contains(&position) {
                    crt_image.push_str("#");
                } else {
                    crt_image.push_str(".");
                }
                cycle += 1;
                position += 1;
                position %= 40;
            }
            "addx" => {
                if position == register - 2 {
                    crt_image.push_str(".#");
                } else if position == register - 1 || position == register {
                    crt_image.push_str("##");
                } else if position == register + 1 {
                    crt_image.push_str("#.");
                } else {
                    crt_image.push_str("..");
                }
                register += words[1].parse::<i32>().unwrap();
                cycle += 2;
                position += 2;
                position %= 40;
            }
            _ => panic!("Unknown operation"),
        };
    });

    crt_image
        .as_bytes()
        .chunks(40)
        .for_each(|x| println!("{}", String::from_utf8_lossy(x)));
}
