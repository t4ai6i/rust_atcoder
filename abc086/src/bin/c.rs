use std::ops::Sub;
use num_integer::Integer;

fn abs_difference<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}
#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: u32,
    y: u32,
}
impl Vec2 {
    fn dist(&self, other: &Vec2) -> u32 {
        abs_difference(self.x, other.x) + abs_difference(self.y, other.y)
    }
}
#[derive(Debug, Copy, Clone)]
struct CheckPoint {
    time: u32,
    pos: Vec2,
}

fn accumulator(check_points: &Vec<CheckPoint>) -> bool {
    fn accumulator(index: usize, current: CheckPoint, prev_result: bool, check_points: &Vec<CheckPoint>) -> bool {
        let next: Option<&CheckPoint> = check_points.get(index);
        let result = next.map(|next| {
            let dt = next.time - current.time;
            let dist = next.pos.dist(&current.pos);
            let is_reached = dt.is_even() == dist.is_even() && dist <= dt;
            (is_reached, *next)
        });
        match result {
            Some((true, next)) => accumulator(index + 1, next, true, check_points),
            Some((false, _)) => false,
            None => prev_result,
        }
    }
    let first = CheckPoint {
        time: 0,
        pos: Vec2 {
            x: 0,
            y: 0
        },
    };
    accumulator(0, first, false, check_points)
}

fn main() {
    proconio::input! {
        n: u32,
        xs: [(u32, u32, u32); n],
    }

    let check_points = xs.iter().map(|&v| CheckPoint {
        time: v.0,
        pos: Vec2 {
            x: v.1,
            y: v.2,
        },
    }).collect::<Vec<CheckPoint>>();
    let is_reached = accumulator(&check_points);
    let actual = if is_reached { "Yes" } else { "No" };
    println!("{}", actual);
}
