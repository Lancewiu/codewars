#[allow(dead_code)]
pub fn solution(num: i32) -> i32 {
    (3..num).filter(|i| 0 == i % 3 || 0 == i % 5).sum()
}
