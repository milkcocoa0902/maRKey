#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextOffset(usize);
impl TextOffset {
    pub fn new(offset: usize) -> Self {
        TextOffset(offset)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextRange {
    start: TextOffset,
    end: TextOffset,
}
impl TextRange {
    pub fn new(start: TextOffset, end: TextOffset) -> Self {
        TextRange { start, end }
    }

    pub fn start(&self) -> TextOffset {
        self.start
    }

    pub fn end(&self) -> TextOffset {
        self.end
    }
}
