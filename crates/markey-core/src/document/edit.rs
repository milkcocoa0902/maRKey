use crate::document::TextRange;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextEdit {
    range: TextRange,
    replacement: String,
}
impl TextEdit {
    pub fn new(range: TextRange, replacement: String) -> Self {
        TextEdit { range, replacement }
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}
