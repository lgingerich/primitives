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
        debug_assert!(self.tail < self.capacity(), "tail out of bounds" );
        debug_assert!(self.size <= self.capacity(), "size exceeds capacity");

        if self.is_full() {
            return Err(val); // error when ring buffer is full
        }

        self.buf[self.tail] = Some(val);
        self.tail = (self.tail + 1) % self.capacity();
        self.size += 1;
        
        debug_assert!(self.tail < self.capacity(), "tail out of bounds" );
        debug_assert!(self.size <= self.capacity(), "size exceeds capacity");

        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        debug_assert!(self.head < self.capacity(), "head out of bounds" );
        debug_assert!(self.size <= self.capacity(), "size exceeds capacity");
        
        if self.is_empty() {
            return None; // empty, nothing to pop
        }
        let out = self.buf[self.head].take();
        self.head = (self.head + 1) % self.capacity();
        self.size -= 1;

        debug_assert!(self.head < self.capacity(), "head out of bounds" );
        debug_assert!(self.size <= self.capacity(), "size exceeds capacity");

        out
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None; // empty, nothing to peek
        }
        self.buf[self.head].as_ref()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None; // empty, nothing to peek
        }
        self.buf[self.head].as_mut()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.buf.len()
    }

    pub fn clear(&mut self) {
        self.buf.iter_mut().for_each(|slot| *slot = None);
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    }
}


#[cfg(test)]
mod tests {
    use super::RingBuffer;

    #[test]
    fn new_zero_capacity_returns_error() {
        let rb: Result<RingBuffer<i32>, _> = RingBuffer::new(0);
        assert!(rb.is_err());
    }

    #[test]
    fn push_then_pop_single_value() {
        let mut rb = RingBuffer::new(2).unwrap();

        assert_eq!(rb.push(42), Ok(()));
        assert_eq!(rb.pop(), Some(42));
        assert_eq!(rb.pop(), None);
    }

    #[test]
    fn push_full_returns_err_with_value() {
        let mut rb = RingBuffer::new(2).unwrap();

        assert_eq!(rb.push(1), Ok(()));
        assert_eq!(rb.push(2), Ok(()));
        assert_eq!(rb.push(3), Err(3));
    }

    #[test]
    fn pop_empty_returns_none() {
        let mut rb: RingBuffer<i32> = RingBuffer::new(3).unwrap();
        assert_eq!(rb.pop(), None);
    }

    #[test]
    fn wraparound_preserves_fifo_order() {
        let mut rb = RingBuffer::new(3).unwrap();

        assert_eq!(rb.push(10), Ok(()));
        assert_eq!(rb.push(20), Ok(()));
        assert_eq!(rb.push(30), Ok(()));

        assert_eq!(rb.pop(), Some(10));
        assert_eq!(rb.pop(), Some(20));

        assert_eq!(rb.push(40), Ok(()));
        assert_eq!(rb.push(50), Ok(()));

        assert_eq!(rb.pop(), Some(30));
        assert_eq!(rb.pop(), Some(40));
        assert_eq!(rb.pop(), Some(50));
        assert_eq!(rb.pop(), None);
    }
}