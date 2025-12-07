#[test]
fn day_7_laboratories() {
    use std::collections::HashSet;
    let input = include_str!("../7/input");
    let grid: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();
    let height = grid.len();
    let width = grid.first().map(|r| r.len()).unwrap_or(0);
    let start_pos = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .find(|&(x, y)| grid[y][x] == b'S')
        .expect("No start found");
    let process_level = |(beams, splits): (HashSet<usize>, usize), y: usize| {
        if beams.is_empty() {
            return (beams, splits);
        }
        let mut next_beams = HashSet::new();
        let mut new_splits = 0;
        for &x in &beams {
            let cell = grid[y][x];

            match cell {
                b'^' => {
                    new_splits += 1;
                    if x > 0 { next_beams.insert(x - 1); }
                    if x + 1 < width { next_beams.insert(x + 1); }
                },
                _ => {
                    next_beams.insert(x);
                }
            }
        }
        (next_beams, splits + new_splits)
    };
    let initial_beams: HashSet<usize> = [start_pos.0].into();
    let levels_to_scan = (start_pos.1 + 1)..height;
    let (_, total_splits) = levels_to_scan.fold((initial_beams, 0), process_level);
    println!("{}", total_splits);
}

#[test]
fn day_7_laboratories_part_two() {
    let grid = include_str!("../7/input").lines().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
    let width = grid.first().map(|r| r.len()).unwrap_or(0);
    let is_source_for = |from_x: usize, to_x: usize, cell: u8| match cell {
        b'^' => (from_x > 0 && to_x == from_x - 1) || (from_x + 1 < width && to_x == from_x + 1),
        _    => to_x == from_x
    };
    let contribute = |src_x: usize, target_x: usize, prev_y: usize, counts: &Vec<u64>|
        is_source_for(src_x, target_x, grid[prev_y][src_x])
            .then(|| counts[src_x])
            .unwrap_or(0);
    let calc_next_val = |target_x: usize, prev_y: usize, prev_row: &Vec<u64>|
        [target_x.wrapping_sub(1), target_x, target_x + 1].iter()
            .filter(|&&src| src < width)
            .map(|&src| contribute(src, target_x, prev_y, prev_row))
            .sum();
    let evolve_row = |prev_row, current_y| (0..width)
        .map(|x| calc_next_val(x, current_y - 1, &prev_row))
        .collect();
    let is_start_pos = |x: usize| grid.iter().any(|r| r[x] == b'S');
    let start_x = (0..width).find(|&x| is_start_pos(x)).unwrap_or(0);
    let start_y = grid.iter().position(|r| r.contains(&b'S')).unwrap_or(0);
    let initial_row = (0..width).map(|x| if x == start_x { 1 } else { 0 }).collect();
    println!("{}", ((start_y + 1)..grid.len()).fold(initial_row, evolve_row).iter().sum::<u64>());
}