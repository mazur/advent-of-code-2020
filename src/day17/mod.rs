use std::collections::HashMap;

pub fn run() {
    let input = include_str!("input.txt");
    
    let mut game = Game::from(input);

    for _ in 0..6 {
        game.next3d();
    }

    println!("Day17 - Part 1: {}", game.board.len());

    let mut game = Game::from(input);

    for _ in 0..6 {
        game.next4d();
    }

    println!("Day17 - Part 2: {}", game.board.len());
}

type Cell = (i32,i32, i32, i32);


struct Game {
    board: Vec<Cell>
}

impl Game {
    #[allow(dead_code)]
    fn new() -> Self {
        Game {
            board: Vec::new()
        }
    }

    fn from(state: &str) -> Self {
        let mut board = Vec::new();
        
        let mut x = 0;
        for row in state.lines() {
            let mut y = 0;
            for cell in row.chars() {
                if cell == '#' {
                    board.push((x, y, 0, 0));
                }
                y += 1;
            }
            x += 1;
        }
        
        Game {
            board: board
        }
    }

    fn next3d(&mut self) {
        let mut touched: HashMap<Cell, (usize, bool)> = HashMap::new();

        for (cx, cy, cz, _) in &self.board {
            for x in *cx-1..=*cx+1 {
                for y in *cy-1..=*cy+1 {
                    for z in *cz-1..=*cz+1 {
                        if x == *cx && y == *cy && z == *cz {
                            match touched.get_mut(&(x, y, z, 0)) {
                                Some(n) => { n.1 = true; },
                                None => { touched.insert((x, y, z, 0), (0, true)); }
                            }
                        }
                        else {
                            match touched.get_mut(&(x, y, z, 0)) {
                                Some(n) => { n.0 += 1; },
                                None => { touched.insert((x, y, z, 0), (1, false)); }
                            }
                        }
                    }
                }
            }
        }

        let mut alive = Vec::new();
        for (cell, count) in touched {            
            if count.0 == 3 ||  (count.1 && count.0 == 2) {
                alive.push(cell);
            }
        }

        self.board = alive;
    }

    fn next4d(&mut self) {
        let mut touched: HashMap<Cell, (usize, bool)> = HashMap::new();

        for (cx, cy, cz, cw) in &self.board {
            for x in *cx-1..=*cx+1 {
                for y in *cy-1..=*cy+1 {
                    for z in *cz-1..=*cz+1 {
                        for w in *cw-1..=*cw+1 {
                            if x == *cx && y == *cy && z == *cz && w == *cw{
                                match touched.get_mut(&(x, y, z, w)) {
                                    Some(n) => { n.1 = true; },
                                    None => { touched.insert((x, y, z, w), (0, true)); }
                                }
                            }
                            else {
                                match touched.get_mut(&(x, y, z, w)) {
                                    Some(n) => { n.0 += 1; },
                                    None => { touched.insert((x, y, z, w), (1, false)); }
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut alive = Vec::new();
        for (cell, count) in touched {            
            if count.0 == 3 ||  (count.1 && count.0 == 2) {
                alive.push(cell);
            }
        }

        self.board = alive;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_generation() {
        let mut game = Game::new();

        game.board.push((1,0,0,0));
        game.board.push((2,1,0,0));
        game.board.push((0,2,0,0));
        game.board.push((1,2,0,0));
        game.board.push((2,2,0,0));

        game.next3d();

        assert_eq!(11, game.board.len());
    }

    #[test]
    fn test_six_generation() {
        let mut game = Game::new();

        game.board.push((1,0,0,0));
        game.board.push((2,1,0,0));
        game.board.push((0,2,0,0));
        game.board.push((1,2,0,0));
        game.board.push((2,2,0,0));

        for _ in 0..6 {
            game.next3d();
        }

        assert_eq!(112, game.board.len());
    }

    #[test]
    fn test_six_generation_with_parse() {
        let mut game = Game::from(".#.\n..#\n###");

        for _ in 0..6 {
            game.next3d();
        }

        assert_eq!(112, game.board.len());
    }

    #[test]
    fn test_six_generation_4d_with_parse() {
        let mut game = Game::from(".#.\n..#\n###");

        for _ in 0..6 {
            game.next4d();
        }

        assert_eq!(848, game.board.len());
    }
}