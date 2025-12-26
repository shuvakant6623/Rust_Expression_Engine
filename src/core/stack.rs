
#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
    maxsize: usize,
}

impl<T> Stack<T> {
    pub fn new(maxsize: usize) -> Self {
        Stack { 
            data : Vec::with_capacity(maxsize),
            maxsize,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), String> {
        if self.data.len() == self.maxsize {
            Err("Stack Overflow!. cannot push, stack is full".to_string())
        } else {
            self.data.push(item);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<T, String> {
        if self.data.is_empty() {
            Err("Stack Underflow!. cannot pop, stack is empty".to_string())
        } else {
            Ok(self.data.pop().unwrap())
        }
    }

    pub fn peek(&self) -> Result<&T, String> {
        if self.data.is_empty() {
            Err("Error, stack is empty nothing to peak.".to_string())
        } else {
            Ok(self.data.last().unwrap())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_full(&self) -> bool {
    self.data.len() == self.maxsize
}

    pub fn len(&self) -> usize {
        self.data.len()
    }
}