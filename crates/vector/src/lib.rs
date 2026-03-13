pub struct MyVec<T> {
    buf: std::ptr::NonNull<T>,
    length: usize,
    capacity: usize
}

impl<T> MyVec<T> {
    pub fn new() -> MyVec<T> {
        MyVec {
            buf: std::ptr::NonNull::dangling(),
            length: 0,
            capacity: 0
        }
    }

    pub fn with_capacity(capacity: usize) -> MyVec<T> {
        let buf = if capacity == 0 {
            std::ptr::NonNull::dangling()
        } else {
            let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
            let raw = unsafe { std::alloc::alloc(layout) } as *mut T;
            std::ptr::NonNull::new(raw).unwrap_or_else(|| std::alloc::handle_alloc_error(layout))
        };

        MyVec {
            buf,
            length: 0,
            capacity
        }
    }

    // push inserts at last element + 1
    pub fn push(&mut self, val: T) -> Result<(), T> {
        if self.length == self.capacity {
            self.grow();
        }

        unsafe {
            std::ptr::write(self.buf.add(self.length).as_mut(), val);
        }

        // self.storage[self.length] = Some(val);
        self.length += 1;

        Ok(())
    }

    // pop removes last element
    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        self.length -= 1;
        // let out = self.storage[self.length].take();

        unsafe {
            let out = std::ptr::read(self.buf.add(self.length).as_mut());
        }

        out
    }

    // // get = return value at index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }

        self.storage[index].as_ref()
    }

    // // set = set value at index
    pub fn set(&mut self, index: usize, val: T) -> Result<(), T> {
        if index >= self.length {
            return Err(val);
        }

        self.storage[index] = Some(val);

        Ok(())
    }

    // grow only full vec's (length == capacity)
    fn grow(&mut self) {
        // calculate new capacity
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };

        // build new storage
        let mut new_storage: Box<[Option<T>]> =
            std::iter::repeat_with(|| None)
                .take(new_capacity)
                .collect::<std::vec::Vec<_>>()
                .into_boxed_slice();

        // move data over
        for i in 0..self.length {
            new_storage[i] = self.storage[i].take();
        }
        
        self.capacity = new_capacity;
        self.storage = new_storage;
    }

    pub fn is_empty(self) -> bool {
        self.length == 0
    }
}