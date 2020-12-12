pub fn run() {
    let nav = include_str!("input.txt").lines().collect();
    let cords = run_boat('E', nav);

    let manhattan = cords.0.abs() + cords.1.abs();

    println!("Day12 - Part 1: {}", manhattan);
}

fn run_boat(start: char, nav: Vec<&str>) -> (i32, i32) {
    let mut dir = start;
    let mut cords = (0,0);

    for n in nav {
        let mut move_dir: char = n[0..1].chars().next().unwrap();
        let step: i32 = n[1..n.len()].parse().expect("Not a number");

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
        let (x, y) = run_boat('E', vec!["F10", "N3", "F7", "R90", "F11"]);
    
        assert_eq!(-17, x);
        assert_eq!(-8, y);
    }
}
