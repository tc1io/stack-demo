struct StackI32 {
    data: [i32; 500],
    index: usize,
}

impl StackI32 {
    fn new_data() -> Self {
        StackI32 {data: [0; 500], index: 0}
    }

    fn length(&self) -> usize {
        self.index
    }

    fn push(&mut self, item: i32) {
        self.index = self.index + 1;
        self.data[self.index] = item
    }

    fn pop(&mut self) -> Option<usize> {
        self.pop()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
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
        assert_eq!(2, stack.length());
    }

    #[test]
    fn test_pop() {
        let mut stack = StackI32::new_data();
        stack.push(10);
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn test_len() {
        let mut stack = StackI32::new_data();
        stack.push(1);
        stack.push(10);
        assert_eq!(2, stack.length());
    }
}

fn main() {}