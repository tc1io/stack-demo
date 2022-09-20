pub struct StackI32 {
    data: [i32; 500],
    index: usize,
}

impl StackI32 {
    pub fn new_data() -> Self {
        StackI32 {data: [0; 500], index: 0}
    }

    pub fn push(&mut self, item: i32) {
        self.data[self.index] = item;
        self.index = self.index + 1
    }

    pub fn pop(&mut self) {
        if !self.is_empty() {
            self.index = self.index - 1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn length(&self) -> usize {
        self.index
    }

    pub fn peek(&self) -> Option<&usize> {
        if !self.is_empty() {
            Some(&self.index)
        } else {
            None
        }
    }

    pub fn check() {
        for i in 1..1000 {
            println!("The values are {:?}", i);
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = StackI32::new_data();
        stack.push(1);
        stack.push(3);
        assert_eq!(2, stack.get_value(0));
    }

    #[test]
    fn test_pop() {
        let mut stack = StackI32::new_data();
        stack.push(10);
        stack.push(11);
        stack.push(1);
        assert_eq!(stack.get_value(2), 1);
        stack.pop();
        assert_eq!(stack.length(), 2);
        stack.pop();
        assert_eq!(stack.length(), 1);
        stack.pop();
        assert_eq!(stack.length(), 0);
    }

    #[test]
    fn test_len() {
        let mut stack = StackI32::new_data();
        stack.push(1);
        stack.push(10);
        assert_eq!(2, stack.length());
    }

    #[test]
    fn test_peek() {
        let mut stack = StackI32::new_data();
        stack.push(1);
        stack.push(10);
        assert_eq!(stack.peek(), Some(&2))
    }
}

fn main() {
    pub fn check() {
        for i in 1..1000 {
            println!("The values are {:?}", i);
        }
    }
}