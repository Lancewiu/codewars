// (x -> [sqr_divisors])
// [sqr_divisors] == sum((divisors(x)^2)
fn square_div_sum(x: u64) -> u64 {
    let xroot = (x as f64).sqrt() as u64;
    (1..=xroot)
        .filter_map(|i| {
            if 0 == x % i {
                let sqr_i = i.pow(2);
                Some(if sqr_i == x {
                    (sqr_i, sqr_i)
                } else {
                    (sqr_i, (x / i).pow(2))
                })
            } else {
                None
            }
        })
        .fold(
            Vec::with_capacity(xroot as usize),
            |mut res, (div_1, div_2)| {
                res.push(div_1);
                if div_1 != div_2 {
                    res.push(div_2);
                }
                res
            },
        )
        .iter()
        .sum()
}

// (m -> n -> [(num, root^2)]
// find all in [m..n] where sum(divisors(num)^2) == root^2
#[allow(dead_code)]
pub fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..n)
        .filter_map(|num| {
            let sds = square_div_sum(num);
            // essentially zero
            if (sds as f64).sqrt().fract() < std::f64::EPSILON {
                Some((num, sds))
            } else {
                None
            }
        })
        .collect()
}
