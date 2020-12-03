pub fn run() {
    let map = convert_map(include_str!("input.txt"));
    let tree_encounters = count_tree_encounters(&map);

    println!("Day03 - Part 1: {}", tree_encounters);
}

fn count_tree_encounters(map: &Vec<Vec<i32>>) -> i32 {
    let map_length = map.len();
    let map_width = map[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut collision = 0;

    while y < map_length-1 {
        x += 3;
        y += 1;
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
    fn test_count_tree_encounters() {
        let map = convert_map(TEST_INPUT);
        let number_of_encounters = count_tree_encounters(&map);

        assert_eq!(7, number_of_encounters);
    }

}