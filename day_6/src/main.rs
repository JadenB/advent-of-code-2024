use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotated(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Entity {
    Empty,
    Guard(Direction),
    Obstacle,
    OutOfBounds,
}

struct Board {
    entities: HashMap<(i32, i32), Entity>,
    width: i32,
    height: i32,
}

impl Board {
    fn from_string(string: &str) -> Self {
        let mut entities: HashMap<(i32, i32), Entity> = HashMap::new();
        for (y, line) in string.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let (x, y) = (x as i32, y as i32);
                let entity = match char {
                    '#' => Some(Entity::Obstacle),
                    '^' => Some(Entity::Guard(Direction::Up)),
                    'v' => Some(Entity::Guard(Direction::Down)),
                    '<' => Some(Entity::Guard(Direction::Left)),
                    '>' => Some(Entity::Guard(Direction::Right)),
                    _ => None,
                };
                if let Some(entity) = entity {
                    entities.insert((x, y), entity);
                }
            }
        }

        let width = string.lines().map(|l| l.len()).max().unwrap() as i32;
        let height = string.lines().count() as i32;

        Self {
            entities,
            width,
            height,
        }
    }

    fn at(&self, pos: &(i32, i32)) -> Entity {
        if !(0..self.width).contains(&pos.0) || !(0..self.height).contains(&pos.1) {
            return Entity::OutOfBounds;
        }

        self.entities.get(pos).cloned().unwrap_or(Entity::Empty)
    }

    fn guard(&self) -> ((i32, i32), Direction) {
        for (pos, entity) in self.entities.iter() {
            if let Entity::Guard(dir) = entity {
                return (*pos, dir.clone());
            }
        }

        panic!("No guard found!");
    }

    fn walk(&self, pos: &(i32, i32), dir: &Direction) -> ((i32, i32), Direction) {
        let next_pos = match dir {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        };

        match self.at(&next_pos) {
            Entity::Obstacle => (*pos, dir.rotated()),
            _ => (next_pos, dir.clone()),
        }
    }

    fn get_visited(&self) -> HashSet<(i32, i32)> {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let (mut pos, mut dir) = self.guard();
        while !matches!(self.at(&pos), Entity::OutOfBounds) {
            visited.insert(pos);
            (pos, dir) = self.walk(&pos, &dir);
        }

        visited
    }

    fn would_enter_into_loop(
        &self,
        pos: &(i32, i32),
        dir: &Direction,
        new_obstacle: &(i32, i32),
    ) -> bool {
        assert_ne!(pos, new_obstacle);

        let mut entities = self.entities.clone();
        entities.insert(*new_obstacle, Entity::Obstacle);
        let board_with_obstacle = Board {
            entities,
            width: self.width,
            height: self.height,
        };

        let (mut slow_pos, mut slow_dir) = (*pos, dir.clone());
        let (mut fast_pos, mut fast_dir) = (*pos, dir.clone());
        while !matches!(board_with_obstacle.at(&fast_pos), Entity::OutOfBounds) {
            (slow_pos, slow_dir) = board_with_obstacle.walk(&slow_pos, &slow_dir);
            (fast_pos, fast_dir) = board_with_obstacle.walk(&fast_pos, &fast_dir);
            (fast_pos, fast_dir) = board_with_obstacle.walk(&fast_pos, &fast_dir);
            if fast_dir == slow_dir && fast_pos == slow_pos {
                return true;
            }
        }

        false
    }

    fn stuck_in_loop_obstruction_positions(&self) -> HashSet<(i32, i32)> {
        let (guard_pos, guard_dir) = self.guard();

        self.get_visited()
            .into_iter()
            .filter(|v| *v != guard_pos && self.would_enter_into_loop(&guard_pos, &guard_dir, v))
            .collect()
    }
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let board = Board::from_string(&input);

    let result = board.get_visited().len();
    println!("{result}");

    let result = board.stuck_in_loop_obstruction_positions().len();
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        let board = Board::from_string(input);
        assert_eq!(board.stuck_in_loop_obstruction_positions().len(), 6);
    }
}
