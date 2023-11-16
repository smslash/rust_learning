pub fn factorial(num: u64) -> u64 {
    if num == 1 || num == 0 {
        1
    } else {
        let mut res = 1;
        for i in 2..=num {
            res *= i;
        }
        res
    }
}