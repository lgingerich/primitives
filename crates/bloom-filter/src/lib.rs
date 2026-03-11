


struct BloomFilter {
    bits: Vec<u8>
}

impl BloomFilter {
    pub fn new() -> BloomFilter { 
        BloomFilter { bits: vec![0; 100] }
    }

    pub fn insert(&mut self, x: u8) {
        self.bits.push(x)
    }

    pub fn contains(&self, y: u8) -> bool {
        self.bits.iter().any(|&bit| bit == y)
    }
}


#[test]
fn membership_works() {
    let mut filter = BloomFilter::new();
    filter.insert(10);
    assert!(filter.contains(10));
    assert!(!filter.contains(9));
}
