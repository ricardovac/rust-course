use std::ops::Add;

fn main() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let _int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    enums();
    vec();
    println!("5 = 4 = {}", get_sum_generics(5, 4));
    println!("2.4 = 4.4 = {}", get_sum_generics(2.4, 4.4));
    let str1 = String::from("world");
    let str2 = str1.clone();
    println!("Hello {}", str1)
}

fn enums() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }

        fn is_monday(&self) -> bool {
            match self {
                Day::Monday => true,
                _ => false,
            }
        }
    }
    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("today is monday"),
        Day::Tuesday => println!("fuck Tuesday"),
        Day::Wednesday => println!("fuck Wednesday"),
        Day::Thursday => println!("fuck Thursday"),
        Day::Friday => println!("fuck Friday"),
        Day::Saturday => println!("fuck Saturday"),
        Day::Sunday => println!("fuck Sunday"),
    }

    println!("Is today the weekend {}", today.is_weekend());
    println!("today is monday? {}", today.is_monday());
}

fn vec() {
    let _vec1: Vec<i32> = Vec::new(); // cant push nothing to this, not mutable

    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
    let _second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    }
    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i)
    }

    println!("Vec length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());

    // down example
    let (_val_1, _val_2) = get_2(3);
    println!("Nums : {} {}", _val_1, _val_2);

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(num_list))
}

fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn sum_list(list: Vec<i32>) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn get_sum_generics<T: Add<Output = T>>(x: T, y: T) -> T {
    return x +y;
}
