pub fn run() {
    let nav = include_str!("input.txt").lines().collect();
    
    let cords_p1 = run_boat('E', &nav);
    let manhattan_p1 = cords_p1.0.abs() + cords_p1.1.abs();

    println!("Day12 - Part 1: {}", manhattan_p1);

    let cords_p2 = run_boat_waypoint((-10,1), &nav);
    let manhattan_p2 = cords_p2.0.abs() + cords_p2.1.abs();

    println!("Day12 - Part 1: {}", manhattan_p2);
}

fn run_boat_waypoint(start: (i32, i32), nav: &Vec<&str>) -> (i32, i32) {
    let mut waypoint = start;
    let mut cords = (0,0);

    for n in nav {
        let (move_dir, step) = parse_move(n);

        if move_dir == 'F' {
            cords.0 += step * waypoint.0;
            cords.1 += step * waypoint.1;
        } 
        else {
            match move_dir {
                'N' => waypoint.1 += step,
                'S' => waypoint.1 -= step,
                'E' => waypoint.0 -= step,
                'W' => waypoint.0 += step,
                'L' => waypoint = turn_waypoint(waypoint, true, step),
                'R' => waypoint = turn_waypoint(waypoint, false, step),
                _ => panic!("Unknown move!")
            }
        }
    }

    cords
}

fn turn_waypoint(wp: (i32, i32), rev: bool, deg: i32) -> (i32, i32) {
    let mut ticks = deg/90;
    let mut new_wp = wp;

    if rev {
        ticks = 4 - ticks;
    }

    for _ in 0..ticks {
        new_wp = (-new_wp.1, new_wp.0);
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
        let (x, y) = run_boat_waypoint((-10, 1), &vec!["F10", "N3", "F7", "R90", "F11"]);
    
        assert_eq!(-214, x);
        assert_eq!(-72, y);
    }
}
