// #![allow(unused)]
//
// struct Stack<T> {
//     data: Link<T>,
//     // index: T,
//     // next: Link <T>,
// }
//
// // type Link<T> = [T; 500];
//
// type Link<T> = Option<Box<Node<T>>>;
//
// // type Link<T> = Option<[T]>;
// // Option<Node<T>>;
//
// struct Node<T> {
//     elem: T,
//     index: Link<T>,
// }
//
// impl<T> Stack<T> {
//     pub fn new() -> Self {
//         Stack { data: None }
//     }
//
//     fn push(&mut self, item: T) {
//         let new_node = Box::new(Node {
//             elem: item,
//             index: self.data.take(),
//         });
//         self.data = Some(new_node);
//         // self.data[&self.] = item;
//         // self.data = &self.data + 1
//     }
//
//     //CHECK WITH JAN IF RUST HELPS WITH REMOVING FIXED SIZED ARRAY
//
//     fn pop(&mut self) -> Option<T> {
//         self.data.take().map(|node| {
//             self.data = node.index;
//             node.elem
//         })
//     }
//
//     fn peek(&self) -> Option<&T> {
//         self.data.as_ref().map(|node|{
//             &node.elem
//         })
//     }
//
//     fn is_empty(&self) -> bool {
//         self.data.is_none()
//     }
//
//     // fn length(&self) -> Link<T> {
//     //     self.data
//     // }
//
//     // fn check() {
//     //     for i in 1..1000 {
//     //         println!("The values are {:?}", i);
//     //     }
//     // }
// }
//
// #[cfg(test)]
//
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_push() {
//         let mut stack = Stack::new();
//
//         assert_eq!(stack.pop(), None);
//
//         stack.push(1);
//         stack.push(2);
//         stack.push(3);
//
//         assert_eq!(stack.pop(), Some(3));
//         assert_eq!(stack.pop(), Some(2));
//
//         stack.push(4);
//         stack.push(5);
//
//         assert_eq!(stack.peek(), Some(&5))
//     }
//     //
//     // #[test]
//     // fn test_pop() {
//     //     let mut stack = StackI32::new_data();
//     //     stack.push(10);
//     //     stack.push(11);
//     //     stack.push(1);
//     //     stack.pop();
//     //     assert_eq!(stack.length(), 2);
//     //     stack.pop();
//     //     assert_eq!(stack.length(), 1);
//     //     stack.pop();
//     //     assert_eq!(stack.length(), 0);
//     // }
//     //
//     // #[test]
//     // fn test_len() {
//     //     let mut stack = StackI32::new_data();
//     //     stack.push(1);
//     //     stack.push(10);
//     //     assert_eq!(2, stack.length());
//     // }
//
//     // #[test]
//     // fn test_peek() {
//     //     let mut stack = StackI32::new_data();
//     //     stack.push(1);
//     //     stack.push(10);
//     //     assert_eq!(stack.peek(), Some(&2))
//     // }
// }

// enum Option<T> {
//      Some(T),
//      None
// }


struct Stack<T> {
    data: [Option<T>; 500],
    index: usize,
}

impl<T: Copy> Stack<T> {

    fn new() -> Self {
        Self { data: [None; 500]
            , index: 0
        }
    }


    fn push(&mut self, item: T) {
        if self.index >= 500 { panic!("out of bounds"); }

        self.data[self.index as usize] = Some(item);
        self.index = self.index + 1;
    }
    //
    fn pop(&mut self) -> T {
        let val:T = self.data[self.index-1].expect("a used stack element cannot be None");
        self.index = &self.index - 1;
        return val
    }

    fn peek(&self) -> Option<&T> {
        if !self.is_empty() {
            self.data[self.index-1].as_ref()
        } else {
            None
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.index == 0
    }
    //
    // fn length(&self) -> T {
    //     &self.index
    // }
    //
    // // fn check() {
    // //     for i in 1..1000 {
    // //         println!("The values are {:?}", i);
    // //     }
    // // }
}

#[derive(Default, Clone, Copy)]
struct User {
    name: i32,
}

fn main() {
    // fn check() {
    //     for i in 1..1000 {
    //         println!("The values are {:?}", i);
    //     }
    // }

    let mut stack: Stack<User> = Stack::new();
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);
    // stack.pop();
    // stack.pop();
    // stack.push(4);
    // stack.push(5);
    // stack.push(6);
    // stack.push(7);
    //
    // println!("The stack length is {:?}", stack.length());
    // println!("The last added value is {:?}", stack.peek());
}