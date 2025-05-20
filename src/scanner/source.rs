pub trait Source<T> {
    fn null(&self) -> T;
    fn len(&self) -> usize;
    fn at(&self, i: usize) -> T;
}

impl Source<char> for Vec<char> {
    fn null(&self) -> char {
        '\0'
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn at(&self, i: usize) -> char {
        self[i]
    }
}
