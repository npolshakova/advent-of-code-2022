use std::fs;
use std::collections::BinaryHeap;

fn main() {
    let file_path = "/Users/ninapolshakova/advent-code/day1/hello-rust/input2.txt";

    let elf_file = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut heap = BinaryHeap::new();
    for elf in elf_file.split("\n\n").filter(|&x| !x.is_empty()) {

        let filter_elves = elf.split("\n").filter(|&x| !x.is_empty());
        let fold_weight: i32 = filter_elves.map(|x| x.parse::<i32>().unwrap()).sum();
        println!("{}", fold_weight);
        heap.push(fold_weight);
    }

    println!("max 1: {:?}.", heap.pop());
    println!("max 2: {:?}.", heap.pop());
    println!("max 3: {:?}.", heap.pop());
}
