use std::fmt::Error;

#[derive(Clone, Copy, Debug)]
enum QueValue {
    Value(i32),
    None,
}

type QueueReturn = Result<i32, QueueErrors>;

#[derive(Debug)]
pub enum QueueErrors {
    FULL,
    EMPTY,
    None,
}
pub struct Queue {
    store: Vec<QueValue>,
    rear: i32,
    front: i32,
    max_size: i32,
    size: i32,
}

impl Queue {
    pub fn new(max_size: i32) -> Self {
        return Queue {
            store: Vec::with_capacity(max_size as usize),
            rear: -1,
            front: -1,
            max_size,
            size: 0,
        };
    }

    pub fn size(&self) -> i32 {
        return self.size;
    }

    pub fn is_empty(&self) -> bool {
        match self.size() {
            0 => return true,
            _ => return false,
        }
    }

    pub fn enqueue(&mut self, data: i32) {
        let size = self.size();
        if self.size() == self.max_size {
            println!("Queue is full!");
            return;
        }
        if self.rear >= self.front && size == 0 {
            self.front = -1;
            self.rear = 0;
        } else {
            self.rear += 1;
        }

        if self.front == -1 {
            self.front = 0;
        }

        self.store.push(QueValue::Value(data));
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> QueueReturn {
        if self.size() == 0 {
            return Err(QueueErrors::FULL);
        }

        if (self.front == -1) {
            return Err(QueueErrors::EMPTY);
        }

        if self.rear >= self.front && self.size() == 0 {
            self.front = -1;
            self.rear = -1;
            return Err(QueueErrors::EMPTY);
        } else {
            let front = self.front as usize;
            let temp = self.store[front];
            self.store[front] = QueValue::None;
            self.front += 1;
            if let QueValue::Value(v) = temp {
                self.size -= 1;
                return Ok(v);
            } else {
                return Err(QueueErrors::None);
            }
        }
    }

    pub fn peek(&mut self) -> QueueReturn {
        if self.front == -1 {
            return Err(QueueErrors::EMPTY);
        } else {
            if let QueValue::Value(v) = self.store[self.front as usize] {
                return Ok(v);
            } else {
                return Err(QueueErrors::None);
            }
        }
    }

    pub fn list(&self) {
        for i in self.store.iter() {
            if let QueValue::Value(v) = *i {
                print!("{v}, ");
            }
        }
        println!("\n");
    }
}
