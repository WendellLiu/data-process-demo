use process::read_raw_data;

mod entity;
mod process;

fn main() {
    read_raw_data("./data.txt");
    println!("Hello, world!");
}
