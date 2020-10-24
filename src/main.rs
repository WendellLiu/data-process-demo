use entity::{to_history_map, HistoryMap};
use process::read_raw_data;

mod entity;
mod process;

fn main() {
    let history_map: HistoryMap;
    match read_raw_data("./data.txt") {
        Ok(r) => {
            history_map = to_history_map(r);
            println!("{:?}", history_map)
        }
        Err(e) => println!("{}", e),
    };

    println!("Hello, world!");
}
