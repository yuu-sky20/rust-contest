use std::str::FromStr;
use std::fmt::Debug;
// use std::collections::VecDeque;
// use std::collections::HashMap;
// use std::collections::HashSet;

macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

fn get_line() -> String {
    let mut _ret = String::new();
    std::io::stdin().read_line(&mut _ret).ok();
    _ret.trim().to_string()
}

fn read_line<T>() -> Vec<T>
where T: FromStr, <T as FromStr>::Err : Debug {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|e| T::from_str(e).unwrap()).collect()
}

fn geta() -> i64 {
    get_line().split_whitespace()
        .filter_map(|k| k.parse().ok()).collect::<Vec<i64>>()[0]
}

fn main() {
    let line: Vec<usize> = read_line();
    let (h, w) = (line[0], line[1]);
    const n: usize = 30_usize;
    let mut dp = [[0i64; n]; n];

    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..n {
            if i > 0 {
                dp[i][j] += dp[i-1][j];
            }
            if j > 0 {
                dp[i][j] += dp[i][j-1];
            }
        }
    }

    println!("{}", dp[h-1][w-1]);
}


// #[test]
// fn test_main() {
// }