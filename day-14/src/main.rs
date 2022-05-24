const INPUT: &str = include_str!("../input");

fn main() {
    let mut input_lines = INPUT.lines();
    let polymer_template = String::from(input_lines.next().unwrap());
    input_lines.next();
    let mut pair_insertion_rules: [[Option<char>; 26]; 26] = [[None; 26]; 26];
    for line in input_lines {
        let mut parts = line.split(" -> ");
        let mut pair_chars = parts.next().unwrap().chars();
        let first_pair_char_index = char_to_index(pair_chars.next().unwrap());
        let second_pair_char_index = char_to_index(pair_chars.next().unwrap());
        let element = parts.next().unwrap().chars().next().unwrap();
        pair_insertion_rules[first_pair_char_index][second_pair_char_index] = Some(element);
    }
    println!("{}", part_one(&polymer_template, &pair_insertion_rules));
    println!("{}", part_two(&polymer_template, &pair_insertion_rules));
}

fn char_to_index(c: char) -> usize {
    (c as u32 - 'A' as u32) as usize
}

fn part_one(polymer_template: &str, pair_insertion_rules: &[[Option<char>; 26]; 26]) -> u64 {
    solve(polymer_template, pair_insertion_rules, 10)
}

fn part_two(polymer_template: &str, pair_insertion_rules: &[[Option<char>; 26]; 26]) -> u64 {
    solve(polymer_template, pair_insertion_rules, 40)
}

fn solve(
    polymer_template: &str,
    pair_insertion_rules: &[[Option<char>; 26]; 26],
    step_count: usize,
) -> u64 {
    let mut char_counts = [0; 26];
    let polymer_chars: Vec<_> = polymer_template.chars().collect();
    for pair in polymer_chars.windows(2) {
        let mut polymer = String::from_iter(pair);
        for _step in 1..=step_count {
            let mut new_polymer = String::new();
            let polymer_chars: Vec<_> = polymer.chars().collect();
            for pair in polymer_chars.windows(2) {
                let first_char_index = char_to_index(pair[0]);
                let second_char_index = char_to_index(pair[1]);
                let new_c = pair_insertion_rules[first_char_index][second_char_index].unwrap();
                new_polymer.push(pair[0]);
                new_polymer.push(new_c);
            }
            new_polymer.push(pair[1]);
            polymer = new_polymer;
        }
        for c in polymer[..polymer.len() - 1].chars() {
            char_counts[char_to_index(c)] += 1;
        }
    }
    let c = polymer_chars[polymer_chars.len() - 1];
    char_counts[char_to_index(c)] += 1;
    let max_count = char_counts.iter().max().unwrap();
    let min_count = char_counts.iter().filter(|&&n| n > 0).min().unwrap();
    max_count - min_count
}
