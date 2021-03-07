#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lcm() {
        assert_eq!(lcm(5, 0), 0);
    }
}




fn gcd(a: i64, b:i64) -> i64 {
    if a == b {
        return a;
    }
    if a > b {
        return gcd(a - b, b);

    } else {
        return gcd(b  - a, a)
    }
}

fn lcm(a: i64, b:i64) -> i64{
    return (a*b.abs() /  gcd(a,b))
}