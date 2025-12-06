#[test]
fn day_6_trash_compactor() {
    let input = include_str!("../6/input");
    let rows: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();
    let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let height = rows.len();
    let get_char = |x: usize, y: usize| -> u8 {
        *rows.get(y).and_then(|row| row.get(x)).unwrap_or(&b' ')
    };
    let is_col_empty = |x: usize| (0..height).all(|y| get_char(x, y) == b' ');
    let solve_chunk = |cols: &[usize]| {
        let start = *cols.first().unwrap();
        let end = *cols.last().unwrap() + 1;
        let op = (start..end)
            .map(|x| get_char(x, height - 1))
            .find(|&c| c != b' ')
            .unwrap();
        let nums = (0..height - 1).filter_map(|y| {
            let row = rows.get(y)?;
            let s = std::cmp::min(start, row.len());
            let e = std::cmp::min(end, row.len());
            if s >= e { return None; }
            let text = std::str::from_utf8(&row[s..e]).unwrap();
            text.trim().parse::<u64>().ok()
        });
        match op {
            b'+' => nums.sum::<u64>(),
            b'*' => nums.product::<u64>(),
            _ => 0,
        }
    };
    let col_indices = (0..width).chain(std::iter::once(width));
    let process_cols = |(mut current_chunk, mut total): (Vec<usize>, u64), x| {
        if x == width || is_col_empty(x) {
            if !current_chunk.is_empty() {
                total += solve_chunk(&current_chunk);
                current_chunk.clear();
            }
        } else {
            current_chunk.push(x);
        }
        (current_chunk, total)
    };
    let (_, total) = col_indices.fold((Vec::new(), 0), process_cols);
    println!("{}", total);
}

#[test]
fn day_6_trash_compactor_part_two() {
    use std::iter::once;
    let rows: Vec<&[u8]> = include_str!("../6/input").lines().map(|s| s.as_bytes()).collect();
    let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let height = rows.len();
    let get_char = |(x, y)| rows.get(y)
        .copied()
        .and_then(|row: &[u8]| row.get(x))
        .copied()
        .unwrap_or(b' ');
    let is_separator = |x| (0..height)
        .all(|y| get_char((x, y)) == b' ');
    let is_boundary = |x| x == width || is_separator(x);
    let extract_digits = |x| (0..height - 1)
        .map(|y| get_char((x, y)))
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as char)
        .collect::<String>();
    let parse_col = |x| extract_digits(x).parse::<u64>().ok();
    let find_op = |cols: &[usize]| cols.iter()
        .map(|&x| get_char((x, height - 1)))
        .find(|&c| c == b'+' || c == b'*')
        .unwrap_or(b'+');
    let calc_result = |op: u8, nums: Vec<u64>|
        (op == b'*').then(|| nums.iter().product()).unwrap_or_else(|| nums.iter().sum());
    let solve_chunk = |cols: Vec<usize>|
        calc_result(find_op(&cols), cols.iter().filter_map(|&x| parse_col(x)).collect());
    let solve_safe = |cols: Vec<usize>|
        (!cols.is_empty()).then(|| solve_chunk(cols)).unwrap_or(0);
    let append_col = |vec: Vec<usize>, x| vec.into_iter().chain(once(x)).collect();
    let branch_reset = |chunk: Vec<usize>, total: u64| (Vec::new(), total + solve_safe(chunk));
    let branch_accumulate = |chunk, total, x| (append_col(chunk, x), total);
    let process_step = |(chunk, total): (Vec<usize>, u64), x| is_boundary(x)
        .then(|| branch_reset(chunk.clone(), total))
        .unwrap_or_else(|| branch_accumulate(chunk, total, x));
    let (_, grand_total) = (0..width).chain(once(width)).fold((Vec::new(), 0), process_step);
    println!("{}", grand_total);
}