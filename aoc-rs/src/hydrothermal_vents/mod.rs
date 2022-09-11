extern crate regex;

use crate::helpers::{read_file, Point};
use regex::Regex;
use std::{collections::HashMap, io::Result, iter::FromIterator};

#[derive(Debug, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Debug, Clone)]
struct Line<'a> {
    start: &'a Point,
    end: &'a Point,
    direction: Direction,
}

impl Point {
    fn x_eq(&self, p2: &Point) -> bool {
        (*self).x == (*p2).x
    }

    fn y_eq(&self, p2: &Point) -> bool {
        (*self).y == (*p2).y
    }

    fn is_left(&self, p2: &Point) -> bool {
        (*self).x < (*p2).x
    }

    fn is_above(&self, p2: &Point) -> bool {
        (*self).y > (*p2).y
    }

    fn is_diagonal(&self, p2: &Point) -> bool {
        let above = self.is_above(&p2);
        let left = self.is_left(&p2);
        if above && left {
            return self.y.abs_diff(p2.y) == p2.x.abs_diff(self.x);
        } else if above {
            return self.y.abs_diff(p2.y) == self.x.abs_diff(p2.x);
        } else if left {
            return p2.y.abs_diff(self.y) == p2.x.abs_diff(self.x);
        } else {
            return p2.y.abs_diff(self.y) == self.x.abs_diff(p2.x);
        }
    }

    fn get_points_on_line(line: &Line) -> Vec<Point> {
        use Direction::*;
        type Tup = (i32, i32);
        let Line {
            end: Point { x: e1, y: e2 },
            start: Point { x: s1, y: s2 },
            direction: d,
        } = line;

        let ((x1, y1), (x2, y2)): (Tup, Tup) = ((*s1, *e1), (*s2, *e2));

        return match d {
            Horizontal => Vec::from_iter((x1..x2 + 1).map(|v: i32| Point { x: v, y: y1 })),
            Vertical => Vec::from_iter((y1..y2 + 1).map(|v: i32| Point { x: x1, y: v })),
            Diagonal => {
                let mut points: Vec<Point> = Vec::new();
                for (x, y) in (x1..x2 + 1, y1..y2 + 1) {
                    points.push(Point { x, y })
                }

                return points;
            }
        };
    }

    pub fn get_point_line(&self, p2: Point) -> Vec<Point> {
        let direction: Direction;
        let horizontal = self.y_eq(&p2);
        let vertical = self.x_eq(&p2);
        let diagonal = self.is_diagonal(&p2);

        if !horizontal && !vertical && !diagonal {
            return vec![];
        }

        let line: Line = match horizontal {
            true => {
                direction = Direction::Horizontal;
                if self.is_left(&p2) {
                    Line {
                        start: self,
                        end: &p2,
                        direction,
                    }
                } else {
                    Line {
                        start: &p2,
                        end: self,
                        direction,
                    }
                }
            }
            false => {
                direction = Direction::Vertical;
                if self.is_above(&p2) {
                    Line {
                        start: &p2,
                        end: self,
                        direction,
                    }
                } else {
                    Line {
                        start: self,
                        end: &p2,
                        direction,
                    }
                }
            }
        };

        return Point::get_points_on_line(&line);
    }
}

fn get_input() -> String {
    read_file("hydrothermal_vents.txt")
}

fn parse_capture_into_point(x: &str, y: &str) -> Point {
    let parse = |v: &str| -> i32 { v.parse::<i32>().expect("value must be a valid number.") };
    let x = parse(x);
    let y = parse(y);

    Point { x, y }
}

pub fn hydrothermal_vents() -> Result<i32> {
    let mut dangerous_areas = 0;
    let mut intersections: HashMap<String, i32> = HashMap::new();
    let re = Regex::new(r"(?P<x1>\d*),(?P<y1>\d*)\s->\s(?P<x2>\d*),(?P<y2>\d*)").unwrap();

    let input = &get_input()[..];

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let (p1, p2): (Point, Point) = (
            parse_capture_into_point(
                caps.name("x1").unwrap().as_str(),
                caps.name("y1").unwrap().as_str(),
            ),
            parse_capture_into_point(
                caps.name("x2").unwrap().as_str(),
                caps.name("y2").unwrap().as_str(),
            ),
        );

        let point_line = p1.get_point_line(p2);
        println!("{:#?}", point_line);
        for p in point_line {
            let key = format!("{},{}", p.x, p.y);
            let value = intersections.get(&key).or(Some(&0));
            match value {
                Some(&v) => {
                    let occs = v + 1;
                    intersections.insert(key, occs);
                    if occs == 2 {
                        dangerous_areas += 1;
                    }
                }
                None => {
                    intersections.insert(key, 0);
                }
            }
        }
    }

    // Part A => 7085 :)
    // Part B => ???? :)
    Ok(dangerous_areas)
}
