struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }
    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> Option<T> {

        Some(self.queue.remove(0))// remove(0) will panic if there is nothing to remove. Wrapping it in Some wont help.
    }

    fn length(&self) -> usize {
        self.queue.len()
    }
    fn peek(&self) -> Option<&T> {
        self.queue.last()
    }
}

fn main() {
    let mut queue: Queue<f32> = Queue::new();
    queue.enqueue(1.0f32);
    queue.enqueue(2.0f32);
    queue.enqueue(3.0f32);
    println!("{}", queue.dequeue().unwrap());
    println!("{}", queue.dequeue().unwrap());
    println!("{}", queue.dequeue().unwrap());
    match queue.dequeue() {
        None => println!("None!"),
        _ => println!("There was a serious error"),
    }
}
