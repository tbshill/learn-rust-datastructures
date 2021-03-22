// https://www.kirillvasiltsov.com/writing/how-to-write-a-stack-in-rust/

// Lessons:
//  - Vec has built in stack-like methods
//  - Options can be unwrapped with .unwrap()
//  - .unwrap() on a None will panic the application
//

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {stack: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    fn length(&self) -> usize {
        self.stack.len()
    }
    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

fn main(){

    let mut stack: Stack<f32> = Stack::new();

    stack.push(1.0f32);
    stack.push(2.2f32);

    println!("{}",stack.peek().unwrap());
    stack.pop();
    stack.pop();
    println!("{}",stack.peek().unwrap());

}
