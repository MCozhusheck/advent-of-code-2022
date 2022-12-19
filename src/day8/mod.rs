fn split_vec_index(index: usize, vec: &Vec<u32>) -> (&[u32], &[u32]) {
    let vec_split = vec.split_at(index);
    let first = vec_split.0;
    let second = vec_split.1.split_first().unwrap().1;

    return (first, second);
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

            if left.iter().max().unwrap() < val
                || right.iter().max().unwrap() < val
                || up.iter().max().unwrap() < val
                || down.iter().max().unwrap() < val
            {
                visible_trees += 1
            }
        }
    }

    return visible_trees;
}
