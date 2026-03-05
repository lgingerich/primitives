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
            return Err(val); // error when ring buffer is full
        }

        self.buf[self.tail] = Some(val);
        self.tail = (self.tail + 1) % self.buf.len();
        self.size += 1;
        // assert!(head != tail)

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None; // empty, nothing to pop
        }
        let out = self.buf[self.head].take();
        self.head = (self.head + 1) % self.buf.len();
        self.size -= 1;
        out
    }
}




// insert (push) at tail
// extract (pop) at head

// assert!(head < size)
// assert!(tail < size)
// assert!(len(buf) >= 0)
// assert!(len(buf) <= size)
