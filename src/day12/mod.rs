pub fn run() {
    let nav = include_str!("input.txt").lines().collect();
    
    let cords_p1 = run_boat('E', &nav);
    let manhattan_p1 = cords_p1.0.abs() + cords_p1.1.abs();

    println!("Day12 - Part 1: {}", manhattan_p1);

    let cords_p2 = run_boat_waypoint((-10,1), &nav);
    let manhattan_p2 = cords_p2.x.abs() + cords_p2.y.abs();

    println!("Day12 - Part 1: {}", manhattan_p2);
}

struct Cords {
    x: i32,
    y: i32
}

impl Cords {
    fn new (x:i32, y:i32) -> Self{
        Cords { x: x, y: y }
    }

    fn move_to(&mut self, c: Cords, i: i32) {
        self.x += i * c.x;
        self.y += i * c.y;
    }
}

impl From<(i32, i32)> for Cords {
    fn from(n: (i32, i32)) -> Self {
        Cords { x: n.0, y: n.1 }
    }
}

fn run_boat_waypoint(start: (i32, i32), nav: &Vec<&str>) -> Cords {
    let mut waypoint = Cords::from(start);
    let mut cords = Cords::new(0,0);

    for n in nav {
        let (move_dir, step) = parse_move(n);

        if move_dir == 'F' {
            cords.move_to(waypoint, step);
        } 
        else {
            match move_dir {
                'N' => waypoint.y += step,
                'S' => waypoint.y -= step,
                'E' => waypoint.x -= step,
                'W' => waypoint.x += step,
                'L' => waypoint = turn_waypoint(waypoint, true, step),
                'R' => waypoint = turn_waypoint(waypoint, false, step),
                _ => panic!("Unknown move!")
            }
        }
    }

    cords
}

fn turn_waypoint(wp: Cords, rev: bool, deg: i32) -> Cords{
    let mut ticks = deg/90;
    let mut new_wp = wp;

    if rev {
        ticks = 4 - ticks;
    }

    for _ in 0..ticks {
        new_wp = Cords::new(-new_wp.y, new_wp.x);
    }

    new_wp
}

fn run_boat(start: char, nav: &Vec<&str>) -> (i32, i32) {
    let mut dir = start;
    let mut cords = (0,0);

    for n in nav {
        let (mut move_dir, step) = parse_move(n);

        if move_dir == 'F' {
            move_dir = dir;
        }

        match move_dir {
            'N' => cords.1 += step,
            'S' => cords.1 -= step,
            'E' => cords.0 -= step,
            'W' => cords.0 += step,
            'L' => dir = turn(dir, true, step),
            'R' => dir = turn(dir, false, step),
            _ => panic!("Unknown move!")
        }
    }

    cords
}

fn parse_move(n: &str) -> (char, i32) {
    (n[0..1].chars().next().unwrap(), n[1..n.len()].parse().expect("Not a number"))
}

fn turn(curr: char, rev: bool, deg: i32) -> char {
    let mut ticks = deg/90;
    let mut new_dir = curr;

    if rev {
        ticks = 4 - ticks;
    }

    for _ in 0..ticks {
        new_dir = match new_dir {
            'N' => 'E',
            'S' => 'W',
            'E' => 'S',
            'W' => 'N',
            _ => panic!("Unknown move!")
        }
    }

    new_dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_boat() {
        let (x, y) = run_boat('E', &vec!["F10", "N3", "F7", "R90", "F11"]);
    
        assert_eq!(-17, x);
        assert_eq!(-8, y);
    }

    #[test]
    fn test_run_boat_waypoint() {
        let cords = run_boat_waypoint((-10, 1), &vec!["F10", "N3", "F7", "R90", "F11"]);
    
        assert_eq!(-214, cords.x);
        assert_eq!(-72, cords.y);
    }
}
