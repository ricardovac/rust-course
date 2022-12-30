#![allow(unused)]
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::ops::{Add, BitAndAssign};
use std::rc::Rc;

mod boxes;
mod closures;
mod enums;
mod hash_maps;
mod restaurant;
mod threads;
mod vec;
use crate::boxes::box_example;
use crate::closures::example_closures;
use crate::enums::enums_example;
use crate::hash_maps::example_hash_maps;
use crate::restaurant::order_food;
use crate::threads::threads_example;
use crate::vec::vec_example;

/*
* Main function with the basics,
* Other topics separated by modules.
* */
fn main() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let _int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    enums_example();
    vec_example();
    println!("5 = 4 = {}", get_sum_generics(5, 4));
    println!("2.4 = 4.4 = {}", get_sum_generics(2.4, 4.4));
    let str1 = String::from("world");
    let str2 = str1.clone();
    println!("Hello {}", str1);

    let str3 = print_return_str(str1);
    println!("str3 = {}", str3);

    let mut str1 = String::from("Test");
    change_string(&mut str1);

    example_hash_maps();

    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main Street"),
        balance: 234.50,
    };
    bob.address = String::from("505 Main Street");

    struct Rectangle<T, U> {
        length: T,
        height: U,
    }

    let rec = Rectangle {
        length: 4,
        height: 10.5,
    };

    const PI: f32 = 3.141592;
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle2 {
        length: f32,
        width: f32,
    }

    struct Circle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle2 {
        fn new(length: f32, width: f32) -> Rectangle2 {
            return Rectangle2 { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle2 = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec Area : {}", rec.area());
    println!("Circle Area : {}", circ.area());

    order_food();

    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}", error)
        }
    };
    write!(output, "Just some\nRandom words").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap())
    }

    let output2 = File::create("rand.txt");
    let mut output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };

    iter();
    example_closures();
    box_example();
    threads_example();
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" Works");
    println!("Message : {}", name);
}

fn get_sum_generics<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn iter() {
    let mut arr_it = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val)
    }
    arr_it.into_iter();

    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next())
}
