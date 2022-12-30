mod threads {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    pub fn example() {
        let thread1 = thread::spawn(|| {
            for i in 1..25 {
                println!("Spawned thread : {}", i);
                thread::sleep(Duration::from_millis(1))
            }
        });

        for i in 1..20 {
            println!("Main thread : {}", i);
            thread::sleep(Duration::from_millis(1))
        }

        thread1.join().unwrap();

        pub struct Bank {
            balance: f32,
        }

        fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
            let mut bank_ref = the_bank.lock().unwrap();
            if (bank_ref.balance < 5.00) {
                println!(
                    "Current balance : {} Withdrawal a smaller amount",
                    bank_ref.balance
                )
            } else {
                bank_ref.balance -= amt;
                println!(
                    "Custumer withdrew {}\n Current Balance {}",
                    amt, bank_ref.balance
                );
            }
        }

        fn customer(the_bank: Arc<Mutex<Bank>>) {
            withdraw(&the_bank, 5.00)
        }

        let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));
        let handles = (0..10).map(|_| {
            let bank_ref = bank.clone();
            thread::spawn(|| customer(bank_ref))
        });

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Total : {}", bank.lock().unwrap().balance)
    }
}

pub fn threads_example() {
    crate::threads::threads::example();
}
