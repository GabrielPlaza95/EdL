use std::env;

fn main() {
    let mut args: Vec<i32> = env::args()
        .skip(1)
        .filter_map(|arg| arg.parse::<i32>().ok())
        .collect();
    args.sort_unstable();
    println!("{:?}", args);
}
