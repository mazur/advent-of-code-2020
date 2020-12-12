pub fn run() {

}

fn count_jolts(mut adaptors: Vec<i32>) -> (i32, i32, i32) {
    adaptors.sort();

    let mut res = (0,0,0);


    for i in 1..adaptors.len() {
        match adaptors[i] - (adaptors[i] - 1) {
            1 => res.0 += 1,
            2 => res.1 += 1,
            3 => res.2 += 1,
            _ => panic!("Too big diff")
        }

    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_jolts() {
        let jolts = count_jolts(vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3]);

        assert_eq!(11, jolts.0);
        assert_eq!(22, jolts.2);
    }
}