pub struct RingBuffer<T> {
    buf: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
}



impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Result<Self, &'static str> {
        if capacity == 0 {
            return Err("capacity must be > 0")
        }

        Ok(Self {
            buf: (0..capacity).map(|_| None).collect(),
            head: 0,
            tail: 0,
            size: 0,
        })
    }

    pub fn push(&mut self, val: T) -> Result<(), T> {
        debug_assert!(self.tail < self.buf.len(), "tail out of bounds" );
        debug_assert!(self.size <= self.buf.len(), "size exceeds capacity");

        if self.size == self.buf.len() {
            return Err(val); // error when ring buffer is full
        }

        self.buf[self.tail] = Some(val);
        self.tail = (self.tail + 1) % self.buf.len();
        self.size += 1;
        
        debug_assert!(self.tail < self.buf.len(), "tail out of bounds" );
        debug_assert!(self.size <= self.buf.len(), "size exceeds capacity");

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        debug_assert!(self.head < self.buf.len(), "head out of bounds" );
        debug_assert!(self.size <= self.buf.len(), "size exceeds capacity");
        
        if self.size == 0 {
            return None; // empty, nothing to pop
        }
        let out = self.buf[self.head].take();
        self.head = (self.head + 1) % self.buf.len();
        self.size -= 1;

        debug_assert!(self.head < self.buf.len(), "head out of bounds" );
        debug_assert!(self.size <= self.buf.len(), "size exceeds capacity");

        out
    }
}





// assert!(head < size)
// assert!(tail < size)
// assert!(len(buf) >= 0)
// assert!(len(buf) <= size)
