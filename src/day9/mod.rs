use std::collections::HashSet;

fn distance(left: (i32, i32), right: (i32, i32)) -> (u32, u32) {
    return (left.0.abs_diff(right.0), left.1.abs_diff(right.1));
}

pub fn part1(input: String) -> usize {
    let mut visited_points = HashSet::new();
    visited_points.insert((0, 0));

    let mut head = (0, 0);
    let mut tail = (0, 0);

    input.split('\n').for_each(|line| {
        let words = line.split(' ').collect::<Vec<&str>>();
        let steps = words[1].parse::<i32>().unwrap();
        head = match words[0] {
            "U" => {
                let next_head = (head.0, head.1 + steps);
                println!(
                    "head: {:?} next_head: {:?}, tail: {:?}",
                    head, next_head, tail
                );
                tail = match distance(next_head, tail) {
                    (0..=1, 0..=1) => tail,
                    _ => (next_head.0, next_head.1 - 1),
                };

                println!("tail: {:?}", tail);
                next_head
            }
            "R" => {
                let next_head = (head.0 + steps, head.1);
                println!(
                    "head: {:?} next_head: {:?}, tail: {:?}",
                    head, next_head, tail
                );
                tail = match distance(next_head, tail) {
                    (0..=1, 0..=1) => tail,
                    _ => (next_head.0 - 1, next_head.1),
                };
                println!("tail: {:?}", tail);

                next_head
            }
            "D" => {
                let next_head = (head.0, head.1 - steps);
                println!(
                    "head: {:?} next_head: {:?}, tail: {:?}",
                    head, next_head, tail
                );
                tail = match distance(next_head, tail) {
                    (0..=1, 0..=1) => tail,
                    _ => (next_head.0, next_head.1 + 1),
                };
                println!("tail: {:?}", tail);

                next_head
            }
            "L" => {
                let next_head = (head.0 - steps, head.1);
                println!(
                    "head: {:?} next_head: {:?}, tail: {:?}",
                    head, next_head, tail
                );
                tail = match distance(next_head, tail) {
                    (0..=1, 0..=1) => tail,
                    _ => (next_head.0 + 1, next_head.1),
                };
                println!("tail: {:?}", tail);

                next_head
            }
            _ => head,
        };
        visited_points.insert(tail);
    });

    return visited_points.len();
}