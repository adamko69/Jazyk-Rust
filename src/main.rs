use rand::prelude::*;
use std::io::stdin;

fn main() {
    let mut min = String::new();
    let mut max = String::new();

    println!("Zadaj minimalny rozsah: ");
    stdin().read_line(&mut min).expect("Chyba pri nacitani");

    println!();
    println!("Zadaj maximalny rozsah: ");
    stdin().read_line(&mut max).expect("Chyba pri nacitani");
    println!();

    let min: i32 = min.trim().parse().expect("Chyba pri parsovani");
    let max: i32 = max.trim().parse().expect("Chyba pri parsovani");

    if min > max {
        println!("Min musi byt mensie ako max");
        panic!();
    }

    let mut rng = rand::rng();
    let rnd_num = rng.random_range(min..max);
    println!("rnd_num: {rnd_num}");

    //let s: String = String::from("String");
    let s: String = "string".into();
    println!("{s}");

    //let a: i32 = 10;

    let a: bool = false;
    println!("{a}");

    let mut s: String = String::new();
    let value = stdin()
        .read_line(&mut s)
        .expect("Chyba pri nacitani z riadku");

    let numb: i32 = s.trim().parse().expect("Chyba pri parsovani na i32");

    // match value {
    //     Ok(v) => println!("{v}"),
    //     Err(e) => println!("{e}"),
    // }
    println!("{value}");
    println!("{s}");
    println!("{numb}")
}
