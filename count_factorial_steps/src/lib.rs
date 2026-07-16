pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial <= 1 {
        return 0;
    }

    let mut n = factorial;
    let mut step = 2;

    while n > 1 {
        if n % step != 0 {
            return 0;
        }

        n /= step;
        step += 1;
    }

    step - 1
}