#[cfg(test)]
mod tests {
    #[test]
    fn day_1_secret_entrance() {
        let (_, ans) = include_str!("../1/input").lines().fold((50, 0), |(pos, count), s| {
            let val = s[1..].parse::<i32>().unwrap() * if s.starts_with('R') { 1 } else { -1 };
            let pos = (pos + val).rem_euclid(100);
            (pos, count + (pos == 0) as i32)
        });
        println!("{}", ans);
    }

    #[test]
    fn day_1_secret_entrance_part_two() {
        let (_, answer) = include_str!("../1/input").lines().fold((50, 0), |(pos, count), line| {
            let val = line[1..].parse::<i32>().unwrap();
            let rem = val % 100;
            let is_r = line.starts_with('R');
            let hit = if is_r { pos + rem >= 100 } else { pos > 0 && rem >= pos };
            let next_pos = (pos + if is_r { val } else { -val }).rem_euclid(100);
            (next_pos, count + val / 100 + hit as i32)
        });
        println!("{}", answer);
    }
}