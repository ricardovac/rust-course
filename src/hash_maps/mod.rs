mod hash_maps {
    use std::collections::HashMap;
    pub fn example() {
        let mut heroes = HashMap::new();
        heroes.insert("Superman", "Clark Kent");
        heroes.insert("Batman", "Bruce Wayne");
        heroes.insert("The Flash", "Barry Allen");

        for (k, v) in heroes.iter() {
            println!("{} = {}", k, v)
        }

        if heroes.contains_key(&"Batman") {
            let the_batman = heroes.get(&"Batman");
            match the_batman {
                Some(x) => println!("Batman is a hero"),
                None => println!("Batman is not a hero"),
            }
        }
    }
}

pub fn example_hash_maps() {
    crate::hash_maps::hash_maps::example();
}
