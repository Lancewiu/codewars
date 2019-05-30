// (n -> [divisors of n])
fn divisors(n: u64) -> Vec<u64> {
    let nroot = (n as f64).sqrt() as u64;
    let res = Vec::with_capacity(n as usize);
    (1..=nroot)
        .filter(|i| 0 == n % i)
        .map(|i| {
            // (i -> (a, b) where a*b = n
            if i * i == n {
                (i, i)
            } else {
                (i, n / i)
            }
        })
        .fold(res, |mut res, (a, b)| {
            res.push(a);
            if a != b {
                res.push(b);
            }
            res
        })
}

// (m -> n -> [(num, root^2)]
// find all in [m..n] where sum(divisors(num)^2) == root^2
#[allow(dead_code)]
pub fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..n)
        .map(|i| {
            let sqr_div_sum = divisors(i).into_iter().map(|d| d * d).sum();
            (i, sqr_div_sum)
        })
        .filter(|(_, sds)| {
            //sqrt of the sum is a whole number
            (*sds as f64).sqrt().fract() < ::std::f64::EPSILON
        })
        .collect()
}
