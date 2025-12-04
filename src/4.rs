#[test]
fn day_4_printing_department() {
    let input = include_str!("../4/input");
    let w = input.find('\n').unwrap() as isize + 1;
    let bytes = input.as_bytes();
    let count_neighbors = |idx: isize, grid: &[u8]| -> usize {
        [-w - 1, -w, -w + 1, -1, 1, w - 1, w, w + 1]
            .iter()
            .filter(|&&off| {
                let n_idx = idx + off;
                n_idx >= 0 && n_idx < grid.len() as isize && grid[n_idx as usize] == b'@'
            })
            .count()
    };

    println!(
        "{}",
        (0..bytes.len())
            .filter(|&i| bytes[i] == b'@' && count_neighbors(i as isize, bytes) < 4)
            .count()
    );
}

#[test]
fn day_4_printing_department_part_two() {
    use std::iter::successors;

    let input = include_str!("../4/input");
    let width = input.find('\n').unwrap() as isize + 1;
    let limit = input.len() as isize;
    let offsets = [-width - 1, -width, -width + 1, -1, 1, width - 1, width, width + 1];

    let is_active = |grid: &[u8], idx: isize|
        idx >= 0 && idx < limit && grid[idx as usize] == b'@';
    let count_neighbors = |grid: &[u8], idx: usize|
        offsets.iter().map(|&o| idx as isize + o).filter(|&pos| is_active(grid, pos)).count();
    let transform_cell = |grid: &Vec<u8>, i: usize, c: u8|
        (c == b'@' && count_neighbors(grid, i) < 4).then_some(b'.').unwrap_or(c);
    let transform_grid = |grid: &Vec<u8>|
        grid.iter().enumerate().map(|(i, &c)| transform_cell(grid, i, c)).collect::<Vec<_>>();
    let evolve = |grid: &Vec<u8>|
        Some(transform_grid(grid)).filter(|next| next != grid);
    let count_paper = |grid: &Vec<u8>|
        grid.iter().filter(|&&c| c == b'@').count();
    let update_bounds = |acc: Option<(usize, usize)>, c|
        acc.map(|(f, _)| (f, c)).or(Some((c, c)));

    let history = successors(Some(input.as_bytes().to_vec()), evolve);
    let stats = history.map(|g| count_paper(&g));
    let (start, end) = stats.fold(None, update_bounds).unwrap_or((0, 0));

    println!("{}", start - end);
}
