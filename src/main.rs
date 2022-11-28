use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher{
    name:String,
    left:usize,
    right:usize,
}

struct Table{
    forks: Vec<Mutex<()>>,
}

impl Philosopher{
    fn new(name:&str, left:usize, right:usize) ->Philosopher{
        Philosopher{
            name:name.to_string(),
            left:left,
            right:right,
        }
    }

    fn eat(&self, table: &Table){
        let left_ = table.forks[self.left].lock().unwrap();
        let right_ = table.forks[self.right].lock().unwrap();

        println!("{} started eating", self.name);
        thread::sleep_ms(1000);
        println!("{} stopped eating", self.name);
    }
}


fn main() {
    let table = Arc::new(Table {forks:vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});


    let philosophers = vec![
        Philosopher::new("p1", 0, 1),
        Philosopher::new("p2", 1, 2),
        Philosopher::new("p3", 2, 3),
        Philosopher::new("p4", 3, 4),
        Philosopher::new("p5", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p|{
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles{
        h.join().unwrap();
    }
}
