mod vec {
    pub fn example() {
        fn sum_list(list: Vec<i32>) -> i32 {
            let mut sum = 0;
            for &val in list.iter() {
                sum += &val;
            }
            sum
        }

        fn get_2(x: i32) -> (i32, i32) {
            return (x + 1, x + 2);
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
    }
}

pub fn vec_example() {
    crate::vec::vec::example();
}
