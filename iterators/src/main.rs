fn main() {
    //iterators
    let array = [1, 2, 3];

    for num in array.iter(){
        println!("{}", num*2)
    }

    println!("Hello, world!");

    let mut c = Counter::new();
    c.next();
    c.next();
    c.next();
    let i = c.next();

    match i {
        Some(count) => println!("{:?}", count),
        None => {}
    }
}

struct Counter {
    count: i32
}

impl Counter {
    fn new() -> Counter{
        Counter { count: 0 }
    }
}

impl Iterator for Counter{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}