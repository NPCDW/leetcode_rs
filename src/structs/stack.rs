#[derive(Debug)]
#[allow(dead_code)]
struct Stack<T> {
    data: Vec<T>
}

#[allow(dead_code)]
impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            data: Default::default()
        }
    }

    fn push(&mut self, t: T) {
        self.data.push(t);
    }

    fn pop(&mut self) -> Option<T> {
        if self.data.len() > 0 {
            return self.data.pop();
        }
        None
    }

    fn peek(&self) -> Option<&T> {
        if self.data.len() > 0 {
            return self.data.last();
        }
        None
    }
}