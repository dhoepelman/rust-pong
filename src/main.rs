extern crate rug;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use rug::{Assign, Integer};

fn main() {
    //println!("Guess the number!");

    //let secret_number = rand::thread_rng().gen_range(1, 101);

//    loop {
//        println!("Please input your guess.");
//
//        let mut guess = String::new();
//
//        io::stdin()
//            .read_line(&mut guess)
//            .expect("Failed to read line");
//
//        let guess: u32 = match guess.trim().parse() {
//            Ok(num) => num,
//            Err(_) => {
//                println!("Please type a number");
//                continue;
//            }
//        };
//
//        println!("You guessed: {}", guess);
//
//        match guess.cmp(&secret_number) {
//            Ordering::Less => println!("Too small!"),
//            Ordering::Greater => println!("Too big!"),
//            Ordering::Equal => {
//                println!("You win!");
//                break;
//            }
//        }
//    }
    let mut i = 1;
    let mut cache: Vec<Integer> = Vec::new();
    loop {
        let r = fib(i, &mut cache);
        println!("{}\t{}", i, r);
        i += 1;
    }
}

fn fib(n: usize, cache: &mut Vec<Integer>) -> &Integer {
    match cache.get(n) {
        Some(n) => n,
        None => {
            let res = match n {
                0 => Integer::from(1),
                1 => Integer::from(1),
                m => fib(m-2, cache).clone() + fib(m - 1, cache).clone()
            };
            cache.push(res);
            return &cache[n].clone();
        }
    }

}