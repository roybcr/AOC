extern crate regex;

use crate::helpers::{self, read_file, Point};
use regex::Regex;
use std::{collections::HashMap, io::Result, iter::FromIterator};

#[derive(Debug, Clone)]
pub enum Direction {
    Horizontal(i32),
    Vertical(i32),
}

#[derive(Debug, Clone)]
pub struct Line<'a> {
    pub start: i32,
    pub end: i32,
    pub direction: &'a Direction,
}

impl helpers::Point {
    fn x_eq(&self, p2: &helpers::Point) -> bool {
        (*self).x == (*p2).x
    }

    fn y_eq(&self, p2: &helpers::Point) -> bool {
        (*self).y == (*p2).y
    }

    fn is_left(&self, p2: &helpers::Point) -> bool {
        (*self).x < (*p2).x
    }

    fn is_above(&self, p2: &helpers::Point) -> bool {
        (*self).y > (*p2).y
    }

    pub fn get_points_on_line(line: &Line) -> Vec<helpers::Point> {
        let Line {
            end: e,
            start: s,
            direction: d,
        } = *line;
        let points: Vec<helpers::Point> = Vec::from_iter((s..e + 1).map(|v: i32| {
            return match *d {
                Direction::Horizontal(y) => helpers::Point { x: v, y },
                Direction::Vertical(x) => helpers::Point { x, y: v },
            };
        }));

        return points;
    }

    pub fn get_point_line(&self, p2: helpers::Point) -> Vec<helpers::Point> {
        let direction: Direction;
        let horizontal = self.y_eq(&p2);
        let vertical = self.x_eq(&p2);
        if !horizontal && !vertical {
            return vec![];
        }

        let line: Line = match horizontal {
            true => {
                direction = Direction::Horizontal(self.y);
                if self.is_left(&p2) {
                    Line {
                        start: self.x,
                        end: p2.x,
                        direction: &direction,
                    }
                } else {
                    Line {
                        start: p2.x,
                        end: self.x,
                        direction: &direction,
                    }
                }
            }
            false => {
                direction = Direction::Vertical(self.x);
                if self.is_above(&p2) {
                    Line {
                        start: p2.y,
                        end: self.y,
                        direction: &direction,
                    }
                } else {
                    Line {
                        start: self.y,
                        end: p2.y,
                        direction: &direction,
                    }
                }
            }
        };

        return helpers::Point::get_points_on_line(&line);
    }
}

fn get_input() -> String {
    read_file("hydrothermal_vents/input.txt")
}

fn parse_capture_into_point(x: &str, y: &str) -> helpers::Point {
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
        let (p1, p2): (helpers::Point, helpers::Point) = (
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

    // 7085 :)
    Ok(dangerous_areas)
}
