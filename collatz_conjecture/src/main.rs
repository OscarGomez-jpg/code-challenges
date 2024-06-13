use std::collections::HashMap;

use macroquad::prelude::*;

fn collatz(relations: &mut HashMap<usize, Vec<usize>>, num: usize) {
    let mut index = num;
    let mut last_index;

    while index != 1 {
        last_index = index;
        if index % 2 == 0 {
            index /= 2;
        } else {
            index = 3 * index + 1;
        }

        relations.entry(last_index).or_default().push(index);
    }
}

#[macroquad::main("Collatz conjecture")]
async fn main() {
    let mut relations: HashMap<usize, Vec<usize>> = HashMap::new();

    collatz(&mut relations, 10);

    collatz(&mut relations, 30);

    for (key, values) in &relations {
        print!("{} relations: ", key);
        for value in values {
            print!("{} ", value);
        }
        println!();
    }

    loop {
        next_frame().await;
    }
}
