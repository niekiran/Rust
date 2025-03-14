/*
Exercise-diy-13 : Implement a simple generic stack data structure
Requirements
    ==> Define a generic Stack struct that can store items of any type.
Implement the following methods for the Stack:
    ==> new: Creates a new, empty stack.
    ==> push: Adds an item to the top of the stack.
    ==> pop: Removes and returns the item from the top of the stack.
    ==> peek: Returns a reference to the top item without removing it.
    ==> is_empty: Checks if the stack is empty.
    ==> size: Returns the number of items in the stack.
    ==> clear: Removes all items from the stack.

Hints:
    ==> Define a generic struct whose name is 'Stack' generic over T
    ==> USe a 'Vector' to store the items
 */


 // Define the generic Stack struct
struct Stack<T> {
    items: Vec<T>,
}

 //Create a generic impl block 

 impl<T> Stack<T> {
    // Creates a new, empty stack
    fn new()  -> Self {
        Stack { items: Vec::new() }
    }

    // Adds an item to the top of the stack
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // Removes and returns the item from the top of the stack
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Returns a reference to the top item without removing it
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // Checks if the stack is empty
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // Returns the number of items in the stack
     fn size(&self) -> usize {
        self.items.len()
    }

    // Removes all items from the stack
     fn clear(&mut self) {
        self.items.clear();
    }

}


fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    println!("Top element is: {:?}", stack.peek());
    println!("Stack size: {}", stack.size());
}

#[cfg(test)]
mod test {
    use crate::Stack;

    #[test]
    fn test_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(true, stack.is_empty());
    }

    #[test]
    fn test_stack_is_not_empty() {
        let mut stack: Stack<&str> = Stack::new();
        stack.push("Rose");
        assert_eq!(false, stack.is_empty());
    }

    #[test]
    fn test_stack_clear() {
        let mut stack_of_temperatures = Stack::new();
        stack_of_temperatures.push(33.3);
        stack_of_temperatures.push(34.9);
        stack_of_temperatures.clear();
        assert_eq!(true, stack_of_temperatures.is_empty());
        assert_eq!(0, stack_of_temperatures.size());
        assert_eq!(None, stack_of_temperatures.pop());
        assert_eq!(None, stack_of_temperatures.peek());
    }

    //write more test cases. 
}
