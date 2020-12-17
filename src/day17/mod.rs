use std::collections::HashMap;

pub fn run() {
    let mut game = Game::from(include_str!("input.txt"));

    for _ in 0..6 {
        game.next();
    }

    println!("Day17 - Part 1: {}", game.board.len());
}

type Cell = (i32,i32, i32);


struct Game {
    board: Vec<Cell>
}

impl Game {
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
                    board.push((x, y, 0));
                }
                y += 1;
            }
            x += 1;
        }
        
        Game {
            board: board
        }
    }

    fn next(&mut self) {
        let mut touched: HashMap<Cell, (usize, bool)> = HashMap::new();

        for (cx, cy, cz) in &self.board {
            for x in *cx-1..=*cx+1 {
                for y in *cy-1..=*cy+1 {
                    for z in *cz-1..=*cz+1 {
                        if x == *cx && y == *cy && z == *cz {
                            match touched.get_mut(&(x, y, z)) {
                                Some(n) => { n.1 = true; },
                                None => { touched.insert((x, y, z), (0, true)); }
                            }
                        }
                        else {
                            match touched.get_mut(&(x, y, z)) {
                                Some(n) => { n.0 += 1; },
                                None => { touched.insert((x, y, z), (1, false)); }
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

        game.board.push((1,0,0));
        game.board.push((2,1,0));
        game.board.push((0,2,0));
        game.board.push((1,2,0));
        game.board.push((2,2,0));

        game.next();

        assert_eq!(11, game.board.len());
    }

    #[test]
    fn test_six_generation() {
        let mut game = Game::new();

        game.board.push((1,0,0));
        game.board.push((2,1,0));
        game.board.push((0,2,0));
        game.board.push((1,2,0));
        game.board.push((2,2,0));

        for _ in 0..6 {
            game.next();
        }

        assert_eq!(112, game.board.len());
    }

    #[test]
    fn test_six_generation_with_parse() {
        let mut game = Game::from(".#.\n..#\n###");

        for _ in 0..6 {
            game.next();
        }

        assert_eq!(112, game.board.len());
    }
}