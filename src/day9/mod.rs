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
        match words[0] {
            "U" => {
                for step in 0..steps {
                    let next_head = (head.0, head.1 + 1);
                    tail = match distance(next_head, tail) {
                        (0..=1, 0..=1) => tail,
                        _ => (next_head.0, next_head.1 - 1),
                    };
                    head = next_head;
                    visited_points.insert(tail);
                }
            }
            "R" => {
                for step in 0..steps {
                    let next_head = (head.0 + 1, head.1);
                    tail = match distance(next_head, tail) {
                        (0..=1, 0..=1) => tail,
                        _ => (next_head.0 - 1, next_head.1),
                    };
                    head = next_head;
                    visited_points.insert(tail);
                }
            }
            "D" => {
                for step in 0..steps {
                    let next_head = (head.0, head.1 - 1);
                    tail = match distance(next_head, tail) {
                        (0..=1, 0..=1) => tail,
                        _ => (next_head.0, next_head.1 + 1),
                    };
                    head = next_head;
                    visited_points.insert(tail);
                }
            }
            "L" => {
                for step in 0..steps {
                    let next_head = (head.0 - 1, head.1);
                    tail = match distance(next_head, tail) {
                        (0..=1, 0..=1) => tail,
                        _ => (next_head.0 + 1, next_head.1),
                    };
                    head = next_head;
                    visited_points.insert(tail);
                }
            }
            _ => panic!("err"),
        };
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
        match words[0] {
            "U" => {
                for step in 0..steps {
                    for points in points.chunks_mut(2) {
                        let [prev, next] = points;
                        let next_next = (next.0, next.1 + 1);
                        let next_prev = match distance(next.clone(), prev.clone()) {
                            (0..=1, 0..=1) => prev,
                            _ => &mut (next_next.0.clone(), next_next.1.clone() - 1),
                        };
                        visited_points.insert(prev.clone());
                    }
                }
            }
            "R" => {
                for step in 0..steps {
                    for [prev, next] in points.chunks_exact_mut(2) {
                        next = &mut (next.0 + 1, next.1);
                        prev = match distance(*next, *prev) {
                            (0..=1, 0..=1) => prev,
                            _ => &mut (next.0 - 1, next.1),
                        };
                        visited_points.insert(prev.clone());
                    }
                    // let next_head = (head.0 + 1, head.1);
                    // tail = match distance(next_head, tail) {
                    //     (0..=1, 0..=1) => tail,
                    //     _ => (next_head.0 - 1, next_head.1),
                    // };
                    // head = next_head;
                    // visited_points.insert(tail);
                }
            }
            "D" => {
                for step in 0..steps {
                    for [prev, next] in points.chunks_exact_mut(2) {
                        next = &mut (next.0, next.1 - 1);
                        prev = match distance(*next, *prev) {
                            (0..=1, 0..=1) => prev,
                            _ => &mut (next.0, next.1 + 1),
                        };
                        visited_points.insert(prev.clone());
                    }
                    // let next_head = (head.0, head.1 - 1);
                    // tail = match distance(next_head, tail) {
                    //     (0..=1, 0..=1) => tail,
                    //     _ => (next_head.0, next_head.1 + 1),
                    // };
                    // head = next_head;
                    // visited_points.insert(tail);
                }
            }
            "L" => {
                for step in 0..steps {
                    for [prev, next] in points.chunks_exact_mut(2) {
                        next = &mut (next.0 - 1, next.1);
                        prev = match distance(*next, *prev) {
                            (0..=1, 0..=1) => prev,
                            _ => &mut (next.0 + 1, next.1),
                        };
                        visited_points.insert(prev.clone());
                    }
                    // let next_head = (head.0 - 1, head.1);
                    // tail = match distance(next_head, tail) {
                    //     (0..=1, 0..=1) => tail,
                    //     _ => (next_head.0 + 1, next_head.1),
                    // };
                    // head = next_head;
                    // visited_points.insert(tail);
                }
            }
            _ => panic!("err"),
        };
        // visited_points.insert(tail);
    });

    return visited_points.len();
}
