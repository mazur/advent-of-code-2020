pub fn multiply(numbers: &[i32]) -> i64 {
    numbers.iter().fold(1, | prod, i | prod * i64::from(*i))
}