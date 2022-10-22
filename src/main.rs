use std::str::FromStr;
use std::fmt::Debug;

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

fn geta() -> i32 {
    get_line().split_whitespace()
        .filter_map(|k| k.parse().ok()).collect::<Vec<i32>>()[0]
}

fn carry_and_fix(res: &mut Vec<i32>) {

}

fn main() {

}