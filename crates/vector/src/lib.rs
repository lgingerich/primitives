pub struct MyVec<T> {
    buf: std::ptr::NonNull<T>,
    len: usize,
    cap: usize,
}

// Constructor invariants (`new` and `with_capacity`):
// - `len == 0` and `len <= cap`.
// - `buf` is always non-null.
// - If `cap == 0`, `buf` may be dangling and must not be dereferenced.
// - If `cap > 0`, `buf` points to an allocation valid for `cap` values of `T`.
// - No elements are initialized while `len == 0`.
impl<T> MyVec<T> {
    pub fn new() -> MyVec<T> {
        MyVec {
            buf: std::ptr::NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    pub fn with_capacity(cap: usize) -> MyVec<T> {
        let buf = if cap == 0 {
            std::ptr::NonNull::dangling()
        } else {
            let layout = match std::alloc::Layout::array::<T>(cap) {
                Ok(layout) => layout,
                Err(_) => panic!("capacity overflow in MyVec::with_capacity",),
            };
            let raw = unsafe { std::alloc::alloc(layout) } as *mut T;
            std::ptr::NonNull::new(raw).unwrap_or_else(|| std::alloc::handle_alloc_error(layout))
        };

        MyVec {
            buf,
            len: 0,
            cap,
        }
    }

    // push inserts at last element + 1
    pub fn push(&mut self, val: T) -> Result<(), T> {
        if self.len == self.cap {
            self.grow()?;
        }

        unsafe {
            std::ptr::write(self.buf.add(self.len).as_ptr(), val);
        }

        self.len += 1;

        Ok(())
    }

    // pop removes last element
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        let out = unsafe { std::ptr::read(self.buf.add(self.len).as_ptr()) };

        Some(out)
    }

    // get = return value at index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        Some(unsafe { &*self.buf.as_ptr().add(index) })
    }

    // set = set value at index
    pub fn set(&mut self, index: usize, val: T) -> Result<(), T> {
        if index >= self.len {
            return Err(val);
        }

        let _old = unsafe { std::ptr::replace(self.buf.as_ptr().add(index), val) };

        Ok(())
    }

    // grow only full vec's (len == cap)
    fn grow(&mut self) {
        // calculate new capacity
        let new_cap = if self.cap == 0 {
            1
        } else {
            self.cap * 2
        };

        let new_layout = std::alloc::Layout::array::<T>(new_cap).unwrap_or_else(|_| panic!("capacity overflow in MyVec::grow"));

        let new_raw = if self.cap == 0 {
            // use alloc() on first allocation when capacity = 0
            unsafe { std::alloc::alloc(new_layout) }
        } else {
            // for capacity > 0, we want to resize, so use realloc()
            let old_layout = std::alloc::Layout::array::<T>(self.cap).unwrap_or_else(|_| panic!("capacity overflow in MyVec::grow"));

            unsafe {
                std::alloc::realloc(self.buf.as_ptr() as *mut u8, old_layout, new_layout.size())
            }
        } as *mut T;

        let new_buf = std::ptr::NonNull::new(new_raw).unwrap_or_else(|| std::alloc::handle_alloc_error(new_layout));

        self.buf = new_buf;
        self.cap = new_cap;
    }

    pub fn is_empty(self) -> bool {
        self.len == 0
    }
}
