use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    for (i, path) in args.iter().enumerate() {
        if i != 0 {
            if fs::metadata(path).is_ok() {
                let data = fs::read_to_string(path);
                if data.is_ok() {
                    println!("{}", data.unwrap());
                }
            }
        }
    }
}
