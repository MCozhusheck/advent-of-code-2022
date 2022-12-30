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
        for _step in 0..steps {
            let next_head = match words[0] {
                "U" => (head.0, head.1 + 1),
                "R" => (head.0 + 1, head.1),
                "D" => (head.0, head.1 - 1),
                "L" => (head.0 - 1, head.1),
                _ => panic!("Invalid direction symbol!"),
            };
            tail = match distance(next_head, tail) {
                (0..=1, 0..=1) => tail,
                _ => head,
            };
            head = next_head;
            visited_points.insert(tail);
        }
        visited_points.insert(tail);
    });

    return visited_points.len();
}

pub fn part2(input: String) -> usize {
    let mut visited_points = HashSet::new();
    visited_points.insert((0, 0));

    let mut points = [(0, 0); 10];

    input.split('\n').for_each(|line| {
        let words = line.split(' ').collect::<Vec<&str>>();
        let steps = words[1].parse::<i32>().unwrap();
        for step in 0..steps {
            let mut head = points.first_mut().unwrap();
            let mut next_head = match words[0] {
                "U" => (head.0, head.1 + 1),
                "R" => (head.0 + 1, head.1),
                "D" => (head.0, head.1 - 1),
                "L" => (head.0 - 1, head.1),
                _ => *head,
            };
            *head = next_head;
            for i in 1..10 {
                if let Ok([tail, next]) = points.get_many_mut([i, i - 1]) {
                    let row_diff: i32 = next.0 - tail.0;
                    let col_diff: i32 = next.1 - tail.1;

                    if row_diff == 0 && col_diff.abs() > 1 {
                        *tail = (tail.0, tail.1 + col_diff.signum());
                    } else if col_diff == 0 && row_diff.abs() > 1 {
                        *tail = (tail.0 + row_diff.signum(), tail.1);
                    } else if row_diff.abs() > 1 || col_diff.abs() > 1 {
                        *tail = (tail.0 + row_diff.signum(), tail.1 + col_diff.signum());
                    }

                    if i == 9 {
                        visited_points.insert(tail.clone());
                    }
                }
            }
        }
    });

    return visited_points.len();
}
