use crate::UnsignedInt;

/// Find the greatest common divisor.
fn gcd(mut n: u128, mut m: u128) -> Result<u128, &'static str> {
    if n == 0 || m == 0 {
        return Err("zero or negative values not supported");
    }
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    Ok(n)
}

/// This is a brute-force implementation of modular inverse. The Extended Euclidian Algorithm might be a better choice.
fn meziriac(a: u128, b: u128) -> Result<u128, &'static str> {
    let mut g: u128 = 1;
    if b == 1 {
        g = 1;
    } else if a == b {
        g = 0;
    } else {
        while g < u128::MAX {
            if ((g * a) % b) == 1 {
                break;
            }
            g += 1;
        }
    }
    Ok(g)
}

/// Core implementation of intersection of two residual classes.
pub(crate) fn intersection<T>(m1: T, m2: T, s1: T, s2: T) -> Result<(T, T), &'static str>
where
    T: UnsignedInt,
{
    let m1: u128 = m1.into();
    let m2: u128 = m2.into();
    let mut s1: u128 = s1.into();
    let mut s2: u128 = s2.into();

    if m1 == 0 || m2 == 0 {
        // intersection of null and anything is null
        return Ok((
            T::try_from(0u128).map_err(|_| "intersection conversion failure")?,
            T::try_from(0u128).map_err(|_| "intersection conversion failure")?,
        ));
    }
    // normalize shifts
    s1 %= m1;
    s2 %= m2;

    // use common divisor
    let d = gcd(m1, m2)?;
    let md1 = m1 / d;
    let md2 = m2 / d;
    let span = s2.abs_diff(s1);

    if d != 1 && !span.is_multiple_of(d) {
        return Ok((
            T::try_from(0u128).map_err(|_| "intersection conversion failure")?,
            T::try_from(0u128).map_err(|_| "intersection conversion failure")?,
        )); // no intersection
    }
    // NOTE: though this case was specified, it seems impossible to replicate
    // if d != 1 && (span % d == 0) && (s1 != s2) && (md1 == md2) {
    //     return Ok((d, s1));
    // }

    // d might be 1
    let m = md1
        .checked_mul(md2)
        .and_then(|v| v.checked_mul(d))
        .ok_or("intersection overflow")?;
    let shift = s1
        .checked_add(
            meziriac(md1, md2)?
                .checked_mul(span)
                .and_then(|v| v.checked_mul(md1))
                .ok_or("intersection overflow")?,
        )
        .ok_or("intersection overflow")?
        % m;
    Ok((
        T::try_from(m).map_err(|_| "intersection conversion failure")?,
        T::try_from(shift).map_err(|_| "intersection conversion failure")?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_a() {
        assert_eq!(gcd(14, 15).unwrap(), 1);
    }

    #[test]
    fn test_gcd_b() {
        assert_eq!(gcd(12, 8).unwrap(), 4);
    }

    #[test]
    fn test_gcd_c() {
        let a = 2 * 3 * 5 * 11 * 17;
        let b = 3 * 7 * 11 * 13 * 19;
        assert_eq!(gcd(a, b).unwrap(), 3 * 11);
    }

    #[test]
    fn test_gcd_d() {
        assert_eq!(gcd(12, 0).is_err(), true);
    }

    #[test]
    fn test_gcd_e() {
        assert_eq!(gcd(0, 3).is_err(), true);
    }

    #[test]
    fn test_intersection_a() {
        assert_eq!(intersection::<u64>(0, 0, 2, 3).unwrap(), (0, 0));
    }

    #[test]
    fn test_intersection_b() {
        assert_eq!(intersection::<u64>(45, 40, 11, 1).unwrap(), (360, 101));
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
