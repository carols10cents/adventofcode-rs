use std::collections::{VecDeque, HashSet};

pub fn puzzle(input: &str) -> u32 {
    let start_point = Point { x: 1, y: 1};
    let end_point = Point { x: 31, y: 39 };
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((start_point, 0));
    seen.insert(start_point);

    while let Some((point, distance)) = queue.pop_front() {
        if distance > 50 {
            println!("{} unique locations seen", seen.len() - queue.len() - 1);
        }
        for &neighbor in point.neighbors().iter().filter(|p| is_empty(p)) {
            if neighbor == end_point {
                return distance + 1;
            }
            if !seen.contains(&neighbor) {
                queue.push_back((neighbor, distance + 1));
                seen.insert(neighbor);
            }
        }
    }
    panic!("never got to the end!");
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        let changed_xs = [self.x.checked_sub(1), self.x.checked_add(1)];
        let changed_xs = changed_xs.iter()
            .flat_map(|x| x)
            .map(|&x| Point { x: x, y: self.y });
        let changed_ys = [self.y.checked_sub(1), self.y.checked_add(1)];
        let changed_ys = changed_ys.iter()
            .flat_map(|y| y)
            .map(|&y| Point { x: self.x, y: y });

        changed_xs.chain(changed_ys).collect()
    }
}

fn is_empty(point: &Point) -> bool {
    let &Point { x, y } = point;
    (x*x + 3*x + 2*x*y + y + y*y + 1358).count_ones() % 2 == 0
}

#[cfg(test)]
mod test {
    use super::*;

}
