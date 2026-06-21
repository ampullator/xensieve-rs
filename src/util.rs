/// Find the greatest common divisor.
fn gcd<T>(mut n: T, mut m: T, zero: T) -> Result<T, &'static str>
where
    T: std::ops::Rem<Output = T> + std::cmp::Ord + Copy,
{
    if n <= zero || m <= zero {
        return Err("zero or negative values not supported");
    }
    while m != zero {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m = m % n;
    }
    Ok(n)
}

/// This is a brute-force implementation of modular inverse. The Extended Euclidian Algorithm might be a better choice.
fn meziriac(a: u64, b: u64) -> Result<u64, &'static str> {
    let mut g: u64 = 1;
    if b == 1 {
        g = 1;
    } else if a == b {
        g = 0;
    } else {
        while g < u64::MAX {
            if ((g * a) % b) == 1 {
                break;
            }
            g += 1;
        }
    }
    Ok(g)
}

/// Core implementation of intersection of two residual classes.
pub(crate) fn intersection(
    m1: u64,
    m2: u64,
    mut s1: u64,
    mut s2: u64,
) -> Result<(u64, u64), &'static str> {
    if m1 == 0 || m2 == 0 {
        // intersection of null and anything is null
        return Ok((0, 0));
    }
    // normalize shifts
    s1 %= m1;
    s2 %= m2;

    // use common divisor
    let d = gcd(m1, m2, 0)?;
    let md1 = m1 / d;
    let md2 = m2 / d;
    let span: u64 = (s2 as i128 - s1 as i128).abs().try_into().unwrap();

    if d != 1 && !span.is_multiple_of(d) {
        return Ok((0, 0)); // no intersection
    }
    // NOTE: though this case was specified, it seems impossible to replicate
    // if d != 1 && (span % d == 0) && (s1 != s2) && (md1 == md2) {
    //     return Ok((d, s1));
    // }

    // d might be 1
    let m = md1 * md2 * d;
    Ok((m, (s1 + (meziriac(md1, md2).unwrap() * span * md1)) % m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_a() {
        assert_eq!(gcd(14, 15, 0).unwrap(), 1);
    }

    #[test]
    fn test_gcd_b() {
        assert_eq!(gcd(12, 8, 0).unwrap(), 4);
    }

    #[test]
    fn test_gcd_c() {
        let a = 2 * 3 * 5 * 11 * 17;
        let b = 3 * 7 * 11 * 13 * 19;
        assert_eq!(gcd(a, b, 0).unwrap(), 3 * 11);
    }

    #[test]
    fn test_gcd_d() {
        assert_eq!(gcd(12, 0, 0).is_err(), true);
    }

    #[test]
    fn test_gcd_e() {
        assert_eq!(gcd(0, 3, 0).is_err(), true);
    }

    #[test]
    fn test_intersection_a() {
        assert_eq!(intersection(0, 0, 2, 3).unwrap(), (0, 0));
    }

    #[test]
    fn test_intersection_b() {
        assert_eq!(intersection(45, 40, 11, 1).unwrap(), (360, 101));
    }

    #[test]
    fn test_meziriac_a() {
        assert_eq!(meziriac(1, 1).unwrap(), 1);
        assert_eq!(meziriac(10, 1).unwrap(), 1);
        assert_eq!(meziriac(10, 10).unwrap(), 0);
        assert_eq!(meziriac(12, 12).unwrap(), 0);
        assert_eq!(meziriac(3, 11).unwrap(), 4);
        assert_eq!(meziriac(20, 9).unwrap(), 5);
        assert_eq!(meziriac(101, 13).unwrap(), 4);
    }
}
