const INPUT: &str = include_str!("../input");

fn main() {
    let risk_levels: Vec<Vec<u32>> = INPUT
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    println!("{}", solve(&risk_levels));
    let size = risk_levels.len();
    let new_size = size * 5;
    let mut expanded_risk_levels = vec![vec![0; new_size]; new_size];
    for y in 0..new_size {
        for x in 0..new_size {
            let d = (y / size + x / size) as u32;
            let mut v = risk_levels[y % size][x % size] + d;
            while v > 9 {
                v -= 9;
            }
            expanded_risk_levels[y][x] = v;
        }
    }
    println!("{}", solve(&expanded_risk_levels));
}

fn solve(costs: &Vec<Vec<u32>>) -> u32 {
    let size = costs.len();
    let start = (0, 0);
    let end = (size - 1, size - 1);
    let mut lowest_costs = vec![vec![u32::MAX; size]; size];
    let mut heap = BinaryHeap::default();
    heap.insert((0, start));
    lowest_costs[0][0] = 0;
    while let Some((cost, position)) = heap.pop() {
        let (x, y) = position;
        if cost != lowest_costs[y][x] {
            continue;
        }
        if position == end {
            return cost;
        }
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x + 1 < size {
            neighbors.push((x + 1, y));
        }
        if y + 1 < size {
            neighbors.push((x, y + 1));
        }
        for n in neighbors {
            let new_cost = cost + costs[n.1][n.0];
            if new_cost < lowest_costs[n.1][n.0] {
                lowest_costs[n.1][n.0] = new_cost;
                heap.insert((new_cost, n));
            }
        }
    }
    panic!("no path found")
}

#[derive(Debug, Default, Clone)]
struct BinaryHeap<T: Ord>(Vec<T>);

impl<T: Ord> BinaryHeap<T> {
    pub fn insert(&mut self, item: T) {
        self.0.push(item);
        let mut current_i = self.0.len() - 1;
        let mut parent_i = if current_i % 2 == 0 {
            current_i / 2
        } else {
            current_i - 1
        };
        while self.0[current_i] < self.0[parent_i] {
            self.0.swap(current_i, parent_i);
            current_i = parent_i;
            parent_i = if current_i % 2 == 0 {
                current_i / 2
            } else {
                current_i - 1
            };
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.0.len() == 0 {
            return None;
        }
        let swap_i = self.0.len() - 1;
        self.0.swap(0, swap_i);
        let v = self.0.pop();
        let mut current_i = 0;
        loop {
            let left_i = current_i * 2;
            let right_i = current_i * 2 + 1;
            let mut smallest_i = current_i;
            if left_i < self.0.len() && self.0[left_i] < self.0[smallest_i] {
                smallest_i = left_i;
            }
            if right_i < self.0.len() && self.0[right_i] < self.0[smallest_i] {
                smallest_i = right_i;
            }
            if smallest_i == current_i {
                break;
            }
            self.0.swap(current_i, smallest_i);
            current_i = smallest_i;
        }
        v
    }
}
