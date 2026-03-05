pub struct RingBuffer<T> {
    buf: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
}



impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: (0..capacity).map(|_| None).collect(),
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn push(&mut self, val: T) -> Result<(), T> {
        if self.size == self.buf.len() {
            return Err(val) // error when ring buffer is full
        }

        self.push(val)
    }

    pub fn pop(&mut self) -> Self {

    }
}




// insert (push) at tail
// extract (pop) at head

// assert!(head < size)
// assert!(tail < size)
// assert!(len(buf) >= 0)
// assert!(len(buf) <= size)
