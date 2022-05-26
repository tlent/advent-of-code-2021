use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input");

fn main() {
    let bounds = get_bounds(INPUT);
    println!("{}", part_one(bounds));
}

fn get_bounds(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let s: String = input.trim().chars().skip(15).collect();
    let mut parts = s.split(", y=");
    let x_part = parts.next().unwrap();
    let mut x_values = x_part.split("..");
    let min_x = x_values.next().and_then(|v| v.parse().ok()).unwrap();
    let max_x = x_values.next().and_then(|v| v.parse().ok()).unwrap();
    let y_part = parts.next().unwrap();
    let mut y_values = y_part.split("..");
    let min_y = y_values.next().and_then(|v| v.parse().ok()).unwrap();
    let max_y = y_values.next().and_then(|v| v.parse().ok()).unwrap();
    (min_x..=max_x, min_y..=max_y)
}

fn part_one((_, y_range): (RangeInclusive<i32>, RangeInclusive<i32>)) -> i32 {
    let mut max_vy = 0;
    for initial_vy in 0..1000 {
        let mut y = 0;
        let mut vy = initial_vy;
        while vy > 0 || y > *y_range.start() {
            y += vy;
            vy -= 1;
        }
        if y_range.contains(&y) {
            max_vy = initial_vy;
        }
    }
    ((max_vy + 1) * max_vy) / 2
}
