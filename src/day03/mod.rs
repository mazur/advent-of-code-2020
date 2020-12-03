use crate::util;

pub fn run() {
    let map = convert_map(include_str!("input.txt"));
    let slopes = [[1,1], [3,1], [5,1], [7,1], [1,2]];

    let tree_encounters: Vec<i32> = slopes.iter()
                                            .map(|s| count_tree_encounters(&map, s[0], s[1]))
                                            .collect();

    println!("Day03 - Part 1: {}", tree_encounters[1]);
    println!("Day03 - Part 2: {}", util::multiply(&tree_encounters));
}

fn count_tree_encounters(map: &Vec<Vec<i32>>, x_step: usize, y_step: usize) -> i32 {
    let map_length = map.len();
    let map_width = map[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut collision = 0;

    while y < map_length-1 {
        x += x_step;
        y += y_step;
        collision += map[y][x%map_width];
    }

    collision
}

fn convert_map(raw_map: &str) -> Vec<Vec<i32>> {
    raw_map.lines()
        .map(|l| l.chars().map(|c| match c {
            '#' => 1,
            _ => 0
        }).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#";

    #[test]
    fn test_read_raw_map() {
        let map = convert_map(TEST_INPUT);

        assert_eq!(map[0], [0,0,1,1,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0]);
        assert_eq!(map[10], [0,1,0,0,1,0,0,0,1,0,1,0,1,0,0,1,0,0,0,1,0,1,0,1,0,0,1,0,0,0,1,0,1,0,1,0,0,1,0,0,0,1,0,1,0,1,0,0,1,0,0,0,1,0,1,0,1,0,0,1,0,0,0,1,0,1]);

    }

    #[test]
    fn test_count_tree_encounters_3_1() {
        let map = convert_map(TEST_INPUT);
        let number_of_encounters = count_tree_encounters(&map, 3, 1);

        assert_eq!(7, number_of_encounters);
    }

    #[test]
    fn test_count_tree_encounters_1_1() {
        let map = convert_map(TEST_INPUT);
        let number_of_encounters = count_tree_encounters(&map, 1, 1);

        assert_eq!(2, number_of_encounters);
    }

    #[test]
    fn test_count_tree_encounters_5_1() {
        let map = convert_map(TEST_INPUT);
        let number_of_encounters = count_tree_encounters(&map, 5, 1);

        assert_eq!(3, number_of_encounters);
    }

    #[test]
    fn test_count_tree_encounters_7_1() {
        let map = convert_map(TEST_INPUT);
        let number_of_encounters = count_tree_encounters(&map, 7, 1);

        assert_eq!(4, number_of_encounters);
    }

    #[test]
    fn test_count_tree_encounters_1_2() {
        let map = convert_map(TEST_INPUT);
        let number_of_encounters = count_tree_encounters(&map, 1, 2);

        assert_eq!(2, number_of_encounters);
    }
}