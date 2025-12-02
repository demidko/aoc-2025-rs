fn gift_ids() -> impl Iterator<Item = u64> {
    include_str!("../2/input").trim().split(',').flat_map(|s| {
        let (start, end) = s.split_once('-').unwrap();
        start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
    })
}

#[test]
fn day_2_gift_shop() {
    let sum: u64 = gift_ids()
        .filter(|&n| {
            let len = n.ilog10() + 1;
            len % 2 == 0 && n % (10u64.pow(len / 2) + 1) == 0
        })
        .sum();
    println!("{}", sum);
}

#[test]
fn day_2_gift_shop_part_two() {
    let sum: u64 = gift_ids()
        .filter(|&n| {
            let len = n.ilog10() + 1;
            (1..=len / 2).any(|k| {
                len % k == 0 && n as u128 % ((10u128.pow(len) - 1) / (10u128.pow(k) - 1)) == 0
            })
        })
        .sum();
    println!("{}", sum);
}
