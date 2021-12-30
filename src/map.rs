use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::point::Point;

pub struct Map {
    walls: Vec<Point>,
    floors: Vec<Point>,
    start: Point,
}

impl Map {
    pub fn new(walls: Vec<Point>, floors: Vec<Point>, start: Point) -> Map {
        Map { floors, walls, start }
    }

    pub fn dfs_maze() -> Map {
        dfs_maze(21, 21)
    }

    pub fn get_walls(&self) -> &Vec<Point> {
        &self.walls
    }

    pub fn get_floors(&self) -> &Vec<Point> {
        &self.floors
    }

    pub fn get_start(&self) -> &Point {
        &self.start
    }
}

pub fn dfs_maze(x_size: i32, y_size: i32) -> Map {
    let mut floors = Vec::new();
    let mut walls = Vec::new();
    for x in 0..x_size {
        for y in 0..y_size {
            walls.push(Point::new(x, y));
        }
    }

    let mut current = Point::new(1, 1);
    let start = current;
    let mut visited = Vec::new();
    visited.push(current);

    while !visited.is_empty() {
        floors.push(current);
        walls.retain(|x| *x != current);
        current = visited.pop().unwrap();

        let mut candidates = [
            (current.up(), current.up().up()),
            (current.down(), current.down().down()),
            (current.left(), current.left().left()),
            (current.right(), current.right().right())
        ];

        candidates.shuffle(&mut thread_rng());

        for (_, candidate) in candidates.iter().enumerate() {
            let corridoor = candidate.0;
            let dest = candidate.1;
            // If in map
            if walls.contains(&dest) {
                // If not already visited
                if !floors.contains(&dest) && !visited.contains(&dest) {
                    visited.push(dest);
                    floors.push(corridoor);
                    walls.retain(|x| *x != corridoor);
                }
            }
        }
    }

    Map::new(walls, floors, start)
}
