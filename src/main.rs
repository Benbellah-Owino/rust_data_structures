use data_structures::queue::Queue;
fn main() {
    let mut queue = Queue::new(5);
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.list();
    let a = queue.dequeue().unwrap();
    println!("deque: {}", queue.size());
    match queue.peek() {
        Ok(v) => println!("{v}"),
        Err(e) => println!("Error"),
    }
    let b = queue.dequeue().unwrap();
    println!("deque: {}", queue.size());
    match queue.peek() {
        Ok(v) => println!("{v}"),
        Err(e) => println!("Error"),
    }
    let c = queue.dequeue().unwrap();
    println!("deque: {}", queue.size());

    println!("data {a}");
    println!("data {b}");
    println!("data {c}");

    queue.enqueue(3);
    match queue.peek() {
        Ok(v) => println!("{v}"),
        Err(e) => println!("Error"),
    }
}
