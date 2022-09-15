struct StackI32 {
    data: [i32; 500],
    index: i32,
}

impl StackI32 {
    fn new() -> Self {
        StackI32 {data: [0; 500], index: -1}
    }

    fn length(&self) -> i32 {
        self.index + 1
    }

    fn push(&self, item: i32) {
        self.index = self.index + 1;
        self.data[self.index] = item
    }
}

// impl<T> Stack<T> {
//     fn new() -> Self {
//         Stack { stack: Vec::new() }
//     }
//
//     fn length(&self) -> usize {
//         self.stack.len()
//     }
//
//     fn pop(&mut self) -> Option<T> {
//         self.stack.pop()
//     }
//
//     fn push(&mut self, item: T) {
//         self.stack.push(item)
//     }
//
//     fn is_empty(&self) -> bool {
//         self.stack.is_empty()
//     }
//
//     fn peek(&self) -> Option<&T> {
//         self.stack.last()
//     }
// }

fn main() {}