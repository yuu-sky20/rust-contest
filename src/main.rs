use std::str::FromStr;
use std::fmt::Debug;
// use std::collections::VecDeque;

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

}


// #[test]
// fn test_main() {
// }