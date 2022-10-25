use std::fs::File;

fn main() {
   let a = File::open("abc.txt").expect("Cannot find the file");
}