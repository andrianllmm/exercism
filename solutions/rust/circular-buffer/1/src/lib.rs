pub struct CircularBuffer<T> {
    vec: Vec<T>,
    capacity: usize,
    size: usize,
    front: usize,
    rear: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            capacity,
            size: 0,
            front: 0,
            rear: 1
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.vec[self.rear] = _element;
        self.rear += 1;
        self.size += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> where T: Copy {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        Ok(self.vec[self.rear])
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.front = 0;
        self.rear = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        self.vec[self.rear] = _element;
        self.rear += 1;
        self.size += 1;
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}
