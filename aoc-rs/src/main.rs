#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn get_input() -> &'static str {
    return "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
}

fn parse_line(line: &str) -> Point {
    
    let (dir, amount) = line.split_once(" ").expect("must contains a whitespace");
    let amount: i32 = str::parse::<i32>(amount).expect("second arg must be an integer");

    match dir {
        "forward" => Point { x: amount, y: 0  },
        "up"      => Point { x: 0, y: -amount },
        "down"    => Point { x: 0, y: amount  },
        _         => Point { x: 0, y: 0       },
    }
}

fn main() {

    const POINT: Point = Point { x: 0, y: 0 };
    let result:  Point =
        get_input()
          .lines()
            .map(parse_line)
              .fold(POINT, |mut acc, point| {
                    acc.x += point.x;
                    acc.y += point.y;
                    return acc;
              });

    println!("{:#?}, X: {}, Y: {}", result, result.x, result.y);

}
