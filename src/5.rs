#[test]
fn day_5_cafeteria() {
    use std::ops::RangeInclusive;
    type Range = RangeInclusive<u64>;
    let input = include_str!("../5/input");
    let (ranges_part, ids_part) = input.split_once("\n\n").unwrap();
    let parse_range = |line: &str| {
        let (start, end) = line.split_once('-').unwrap();
        start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
    };
    let fresh_ranges: Vec<Range> = ranges_part.lines().map(parse_range).collect();
    let is_fresh = |id: &u64| fresh_ranges.iter().any(|r| r.contains(id));
    let part1_count = ids_part
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(is_fresh)
        .count();
    println!("Part 1: {}", part1_count);
}

#[test]
fn day_5_cafeteria_part_two() {
    use std::cmp::max;
    use std::iter::once;
    use std::ops::RangeInclusive;
    type Range = RangeInclusive<u64>;
    let (ranges_txt, _) = include_str!("../5/input").split_once("\n\n").unwrap();
    let is_connected = |prev: &Range, next: &Range| *next.start() <= *prev.end() + 1;
    let calc_union = |prev: &Range, next: &Range| *prev.start()..=max(*prev.end(), *next.end());
    let partition_vec = |vec: Vec<Range>, pivot: &Range| vec.into_iter().partition(|r| *r.start() < *pivot.start());
    let take_head = |vec: Vec<Range>| vec.split_last().map(|(_, head)| head.to_vec()).unwrap_or_default();
    let append_to_vec = |vec: Vec<Range>, item: Range| vec.into_iter().chain(once(item)).collect();
    let join_mid = |(left, right): (Vec<Range>, Vec<Range>), mid: Range| left.into_iter().chain(once(mid)).chain(right).collect();
    let insert_ordered = |vec: Vec<Range>, item: Range| join_mid(partition_vec(vec, &item), item);
    let replace_last = |item: Range, vec: Vec<Range>| append_to_vec(take_head(vec), item);
    let check_merge = |acc: &Vec<Range>, next: &Range| acc.last().filter(|last| is_connected(last, next)).is_some();
    let get_union_item = |acc: &Vec<Range>, next: &Range| acc.last().map(|last| calc_union(last, next)).unwrap();
    let branch_merge = |acc: Vec<Range>, next: Range| replace_last(get_union_item(&acc, &next), acc);
    let merge_step = |acc: Vec<Range>, next: Range| check_merge(&acc, &next).then(|| branch_merge(acc.clone(), next.clone())).unwrap_or_else(|| append_to_vec(acc, next));
    let parse_line = |s: &str| s.split_once('-').map(|(a, b)| a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()).unwrap();
    let calc_len = |r: &Range| r.end() - *r.start() + 1;
    let total = ranges_txt.lines().map(parse_line).fold(Vec::new(), insert_ordered).into_iter().fold(Vec::new(), merge_step).iter().map(calc_len).sum::<u64>();
    println!("{}", total);
}
