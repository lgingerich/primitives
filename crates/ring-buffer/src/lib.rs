/// A fixed-capacity FIFO ring buffer.
///
/// Values are pushed at the tail and popped from the head.
/// When full, `push` returns `Err(value)` and does not overwrite data.
pub struct RingBuffer<T> {
    buf: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> RingBuffer<T> {
    /// Creates a new ring buffer with fixed `capacity`.
    ///
    /// Returns an error when `capacity == 0`.
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

    /// Inserts `val` at the tail.
    ///
    /// Returns `Err(val)` if the buffer is full.
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

    /// Removes and returns the head value, or `None` if empty.
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

    /// Returns a shared reference to the head value without removing it.
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None; // empty, nothing to peek
        }
        self.buf[self.head].as_ref()
    }

    /// Returns a mutable reference to the head value without removing it.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None; // empty, nothing to peek
        }
        self.buf[self.head].as_mut()
    }

    /// Returns the current number of stored elements.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns the maximum number of elements the buffer can hold.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// Returns true when the buffer holds no elements.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns true when the buffer has reached capacity.
    pub fn is_full(&self) -> bool {
        self.size == self.buf.len()
    }

    /// Removes all elements while preserving capacity.
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

    #[test]
    fn len_capacity_and_full_empty_transitions() {
        let mut rb = RingBuffer::new(2).unwrap();

        assert_eq!(rb.capacity(), 2);
        assert_eq!(rb.len(), 0);
        assert!(rb.is_empty());
        assert!(!rb.is_full());

        assert_eq!(rb.push(1), Ok(()));
        assert_eq!(rb.len(), 1);
        assert!(!rb.is_empty());
        assert!(!rb.is_full());

        assert_eq!(rb.push(2), Ok(()));
        assert_eq!(rb.len(), 2);
        assert!(rb.is_full());

        assert_eq!(rb.pop(), Some(1));
        assert_eq!(rb.len(), 1);
        assert!(!rb.is_full());
    }

    #[test]
    fn peek_does_not_remove() {
        let mut rb = RingBuffer::new(2).unwrap();
        assert_eq!(rb.push(7), Ok(()));

        assert_eq!(rb.peek(), Some(&7));
        assert_eq!(rb.len(), 1);
        assert_eq!(rb.pop(), Some(7));
    }

    #[test]
    fn peek_mut_can_modify_head() {
        let mut rb = RingBuffer::new(2).unwrap();
        assert_eq!(rb.push(String::from("a")), Ok(()));

        if let Some(value) = rb.peek_mut() {
            value.push('b');
        }

        assert_eq!(rb.pop(), Some(String::from("ab")));
    }

    #[test]
    fn clear_resets_state_and_keeps_capacity() {
        let mut rb = RingBuffer::new(3).unwrap();
        assert_eq!(rb.push(1), Ok(()));
        assert_eq!(rb.push(2), Ok(()));

        rb.clear();

        assert_eq!(rb.capacity(), 3);
        assert_eq!(rb.len(), 0);
        assert!(rb.is_empty());
        assert_eq!(rb.pop(), None);
        assert_eq!(rb.push(9), Ok(()));
    }
}