#![allow(unused)]
use std::{io, os::raw::c_uchar};

fn main() {
    // <-- First --!>

    println!("Hello, world!");

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    println!("{input}");

    // <-- Second --!>

    // let mut num = 10.0_f32;\
    // println!("{num}");
    // let mut num = 0b_10_u32;
    // println!("{num}");
    // let mut num = 0o_10;
    // println!("{num}");
    // let mut num = true;
    // println!("{num}");
    // let mut num = "print";
    // println!("{num}");
    // let num = [1, 2, 3];
    // println!("{num:?}");
    // let num = [1; 3];
    // println!("{}", num[0]);
    // let num = (0x_100, '2', 'r', "Word", (0o_12, 5));
    // println!("{num:?}");
    // println!("{}qww{}1+1+1{:?}", num.1, num.0, num);

    // let mut object = Pascal {
    //     field1: 1,
    //     field2: 2,
    // };

    // println!("{}", object.field1 + 1);
    // object.field2 = 10;
    // println!("{}", object.field2 as f32 / 2.0);

    // let Pascal { field1, field2 } = object;
    // println!("{}", field1);

    let mut object = Pascal {
        field1: 1,
        field2: 2,
    };

    println!("{}", object.sum());

    let num = Pascal::multipl(3, 2);
    println!("{num}");

    let num = multipl_global(1, 2);
    println!("{num}");

    // <-- Third --!>

    let mut array = [1, 2, 3];
    println!("{:?}", &mut array[0..1]);

    let mut vector = vec![1, 2, 3]; // Vec::new();
    println!("{:?}{}", &mut vector[1..3], 1);

    // let i1 = 1;
    // let i2: &i32 = &i1;

    // i1 += 1;
    // println!("{}", i2);

    let obj = Example { f1: 16, f2: 47 };
    let num = calculate(obj);
}

struct Pascal {
    field1: i32,
    field2: u16,
}

impl Pascal {
    fn sum(&self) -> i32 {
        let s = self.field1 + self.field2 as i32;
        s
    }
    fn multipl(x: i32, y: i32) -> i32 {
        x * y
    }
}

fn multipl_global(x: i32, y: i32) -> i32 {
    let s = x * y;
    s
}

struct Example {
    f1: i32,
    f2: i32,
}

fn calculate(obj: Example) -> i32 {
    obj.f1 + obj.f2
}

const NUMBER_A: isize = 10;
const NUMBER_B: usize = 10;
const NUMBER_C: i32 = 10;
const NUMBER_D: f32 = 10.0;
const NUMBER_E: char = 'e';
const NUMBER_F: u8 = b'f';
