pub fn reverse_it(v: i32) -> String {
    let s1 = v.abs().to_string();
    let s2: String = s1.chars().rev().collect();

    if v < 0 {
        format!("-{}{}", s2, s1)
    } else {
        format!("{}{}", s2, s1)
    }
}