use std::collections::HashMap;

pub fn run() {
    let mut ferry = FerrySeats::from(include_str!("input.txt"));

    while !ferry.stable {
        ferry.next();
    }

    println!("Day11 - Part 1: {}", ferry.taken());

    let mut ferry = FerrySeats::with_visability_from(include_str!("input.txt"));

    while !ferry.stable {
        ferry.next();
    }

    println!("Day11 - Part 2: {}", ferry.taken());
}

struct FerrySeats {
    seats: HashMap<(isize, isize), bool>,
    stable: bool,
    max_x: isize,
    max_y: isize,
    get_occupied: &'static dyn Fn(&FerrySeats, (isize, isize)) -> usize,
    occupied_threshold: usize
}

impl FerrySeats {
    fn taken(&self) -> usize {
        self.seats.iter().filter(|(_, &v)| v).count()
    }

    fn next(&mut self) {
        let mut next = HashMap::new();
        self.stable = true;

        for (&seat, &taken) in &self.seats {
            let occupied = &(self.get_occupied)(self, seat);
            next.insert(seat, if !taken && *occupied == 0 {
                self.stable = false;
                true
            }
            else if taken && occupied >= &self.occupied_threshold {
                self.stable = false;
                false
            }
            else {
                taken
            });
        }

        self.seats = next;
    }

    fn get_occupied_neighbours(&self, seat: (isize, isize)) -> usize {
        let (x, y) = seat;
        let mut occupied = 0;

        for current_y in y-1..=y+1 {
            for current_x in x-1..=x+1 {
                if current_x == x && current_y == y {
                    continue;
                }

                match self.seats.get(&(current_x, current_y)) {
                    Some(&v) => if v { occupied += 1 }
                    None => {}
                }
            }
        }
        occupied
    }

    fn get_occupied_visible_neighbours(&self, seat: (isize, isize)) -> usize {
        let mut occupied = 0;

        for ydiff in -1..=1 {
            for xdiff in -1..=1 {
                if ydiff == 0 && xdiff == 0 {
                    continue;
                }

                if self.check_visible_occupied(seat, (xdiff, ydiff)) { 
                    occupied += 1 
                }
                
            }
        }
        occupied
    }

    fn check_visible_occupied(&self, seat: (isize, isize), diff: (isize, isize)) -> bool{
        let (mut x, mut y) = seat;

        loop {
            x += diff.0;
            y += diff.1;

            if x < 0 || y < 0 || x > self.max_x || y > self.max_y {
                return false;
            }

            match self.seats.get(&(x, y)) {
                Some(&v) => return v,
                None => {}
            }
        }
    }

    fn parse(seats_str: &str) -> (HashMap<(isize, isize), bool>, isize, isize) {
        let mut seats = HashMap::new();
        let mut y = 0;
        let mut x = 0;

        for row in seats_str.lines() {
            x = 0;
            for seat in row.chars() {
                if seat != '.' {
                    seats.insert((x,y), seat == '#');    
                }
                x += 1;
            }
            y += 1;
        }

        (seats, x-1, y-1)
    }

    fn from(seats_str: &str) -> Self {
        let (seats, x, y) = Self::parse(seats_str);
        Self::new(seats, x, y, &FerrySeats::get_occupied_neighbours, 4)
    }

    fn with_visability_from(seats_str: &str) -> Self {
        let (seats, x, y) = Self::parse(seats_str);
        Self::new(seats, x, y, &FerrySeats::get_occupied_visible_neighbours, 5)
    }

    fn new(seats: HashMap<(isize, isize), bool>, max_x: isize, max_y: isize,get_occupied: &'static dyn Fn(&Self, (isize, isize)) -> usize, occupied_threshold: usize) -> Self {
        Self { 
            seats: seats, 
            stable: false, 
            max_x: max_x, 
            max_y: max_y,
            get_occupied: get_occupied,
            occupied_threshold: occupied_threshold
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str =
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    pub fn test_build_seats() {
        let ferry = FerrySeats::from(TEST_INPUT);

        assert_eq!(71, ferry.seats.len());
    }

    #[test]
    pub fn test_count_taken_seats() {
        let ferry = FerrySeats::from("L.##L..L##.\n");

        assert_eq!(7, ferry.seats.len());
        assert_eq!(4, ferry.taken());
    }

    #[test]
    pub fn test_next_seats() {
        let mut ferry = FerrySeats::from(TEST_INPUT);

        ferry.next();

        assert_eq!(71, ferry.taken());
    }

    #[test]
    pub fn test_next_twice_seats() {
        let mut ferry = FerrySeats::from(TEST_INPUT);

        ferry.next();
        ferry.next();

        assert_eq!(20, ferry.taken());
    }

    #[test]
    pub fn test_next_until_stable_seats() {
        let mut ferry = FerrySeats::from(TEST_INPUT);

        while !ferry.stable {
            ferry.next();
        }

        assert_eq!(37, ferry.taken());
    }

    #[test]
    pub fn test_next_seats_with_visible_neighbours() {
        let mut ferry = FerrySeats::with_visability_from(TEST_INPUT);

        ferry.next();
        ferry.next();

        assert_eq!(7, ferry.taken());
    }
}
