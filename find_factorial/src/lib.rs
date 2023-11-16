pub fn factorial(n: u64) -> u64 {
    let mut res = 1;
    for i in 2..n {
        res *= i
    }
    res
}