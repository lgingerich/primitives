pub struct Vec<T> {
    storage: Box<[Option<T>]>, // uses to avoid manual memory management for now
    length: usize,
    capacity: usize
}

impl<T> Vec<T> {
    pub fn new() -> Vec<T> {
        Vec {
            storage: Box::new([]),
            length: 0,
            capacity: 0
        }
    }

    // push inserts at last element + 1
    pub fn push() -> {

    }

    // pop removes last element
    pub fn pop() -> {

    }

    // get = return value at index
    pub fn get() -> {

    }

    // set = set value at index
    pub fn set() -> {

    }

    // grow only full vec's (length == capacity)
    fn grow() -> {

    }

    pub fn len(self) -> usize {

    }

    pub fn is_empty(self) -> bool {

    }

}