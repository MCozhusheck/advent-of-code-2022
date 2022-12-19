use std::slice::Iter;

fn split_vec_index(index: usize, vec: &Vec<u32>) -> (&[u32], &[u32]) {
    let vec_split = vec.split_at(index);
    let first = vec_split.0;
    let second = match vec_split.1.split_first() {
        Some(vec) => vec.1,
        None => &[],
    };

    return (first, second);
}

fn view_distance<'a, T>(iter: T, height: &u32) -> u32
where
    T: Iterator<Item = &'a u32>,
{
    let mut distance = 0;
    for current_tree in iter {
        distance += 1;
        if current_tree >= height {
            break;
        }
    }
    return distance;
}

pub fn part1(input: String) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|tree_height| tree_height.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut visible_trees = 0;
    for (i, row) in grid.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
            if i == 0 || i == grid.len() - 1 || y == 0 || y == row.len() - 1 {
                visible_trees += 1;
                continue;
            }

            let column: Vec<u32> = grid
                .iter()
                .map(|row| row.get(y).unwrap().to_owned())
                .collect();

            let (left, right) = split_vec_index(i, &column);
            let (up, down) = split_vec_index(y, &row);

            if left.iter().max().unwrap_or(&0) < val
                || right.iter().max().unwrap_or(&0) < val
                || up.iter().max().unwrap_or(&0) < val
                || down.iter().max().unwrap_or(&0) < val
            {
                visible_trees += 1
            }
        }
    }

    return visible_trees;
}

pub fn part2(input: String) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|tree_height| tree_height.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut higheset_score = 0;
    for (i, row) in grid.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
            let column: Vec<u32> = grid
                .iter()
                .map(|row| row.get(y).unwrap().to_owned())
                .collect();

            let (left, right) = split_vec_index(i, &column);
            let (up, down) = split_vec_index(y, &row);

            let left_score = view_distance(left.iter().rev(), val);
            let right_score = view_distance(right.iter(), val);
            let up_score = view_distance(up.iter().rev(), val);
            let down_score = view_distance(down.iter(), val);

            let scenic_score = left_score * right_score * up_score * down_score;
            if higheset_score < scenic_score {
                higheset_score = scenic_score
            }
        }
    }

    return higheset_score;
}
