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
    println!(
        "{}",
        std::iter::successors(
            Some((
                include_str!("../4/input").as_bytes().to_vec(),
                include_str!("../4/input").find('\n').unwrap() as isize + 1
            )),
            |(grid, w)| std::iter::once(
                grid.iter()
                    .enumerate()
                    .map(|(i, &c)| {
                        if c == b'@' && [
                    -w-1, -w, -w+1,
                    -1,       1,
                    *w-1, *w, *w+1
                ]
                    .iter()
                    .filter(|&&o| matches!(i as isize + o,
                    n if n >= 0 && n < grid.len() as isize && grid[n as usize] == b'@'
                ))
                    .count() < 4 { b'.' } else { c }
                    })
                    .collect::<Vec<_>>()
            )
            .filter(|next| next != grid)
            .map(|next| (next, *w))
            .next()
        )
        .map(|(g, _)| g.iter().filter(|&&c| c == b'@').count())
        .fold(None, |acc, count| acc
            .map(|(first, _)| (first, count))
            .or(Some((count, count))))
        .map_or(0, |(first, last)| first - last)
    );
}
