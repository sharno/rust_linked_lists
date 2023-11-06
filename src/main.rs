use crate::first::List;

pub mod first;

fn main() {
    let mut list1 = List::new();
    let list = &list1.cons(1);
    println!("{:?}\n{:?}", list1, list);
}
