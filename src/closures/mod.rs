mod closures {
    pub fn example() {
        let can_vote = |age: i32| age >= 18;

        println!("Can vote : {}", can_vote(4));

        let mut sample1 = 5;
        let print_var = || println!("sample1 = {}", sample1);
        print_var();

        sample1 = 10;
        let mut change_var = || sample1 += 1;
        change_var();
        println!("samp1 = {}", sample1);
        sample1 = 10;
        println!("samp1 = {}", sample1);

        fn use_func<T>(a: i32, b: i32, func: T) -> i32
        where
            T: Fn(i32, i32) -> i32,
        {
            func(a, b)
        }
        let sum = |a, b| a + b;
        let prod = |a, b| a * b;
        println!("5 + 4 = {}", use_func(5, 4, sum));
        println!("5 * 4 = {}", use_func(5, 4, prod));
    }
}

pub fn example_closures() {
    crate::closures::closures::example();
}
