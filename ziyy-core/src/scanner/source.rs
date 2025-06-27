pub trait Source<T: PartialEq> {
    fn null(&self) -> T;
    fn nl(&self) -> T;
    fn len(&self) -> usize;
    fn at(&self, i: usize) -> T;
}

impl Source<char> for Vec<char> {
    fn null(&self) -> char {
        '\0'
    }

    fn nl(&self) -> char {
        '\n'
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn at(&self, i: usize) -> char {
        self[i]
    }
}

impl Source<u8> for Vec<u8> {
    fn null(&self) -> u8 {
        b'\0'
    }

    fn nl(&self) -> u8 {
        b'\n'
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn at(&self, i: usize) -> u8 {
        self[i]
    }
}
