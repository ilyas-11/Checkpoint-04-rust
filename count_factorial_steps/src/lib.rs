pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial <= 1 {
        return 0;
    }
    let mut result = 1;
    let mut n: u64 = 1;
    while result < factorial {
        n += 1;
        result *= n;
    }
    if result == factorial {
        n
    } else {
        0
    }
}