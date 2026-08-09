pub struct DocumentId(u64);

impl DocumentId {
    pub fn new(id: u64) -> Self {
        DocumentId(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Default)]
pub struct Revision(u64);
impl Revision {
    pub fn value(&self) -> u64 {
        self.0
    }
}

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

pub struct Document {
    id: DocumentId,
    source: String,
    revision: Revision,
}

impl Document {
    pub fn new(id: DocumentId, source: String) -> Self {
        Document {
            id,
            source,
            revision: Revision::default(),
        }
    }

    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn len_bytes(&self) -> usize {
        // 逐次取得するか、フィールドにキャッシュするか
        // そもそも初期段階では、sourceが文書へのパスではなくて内容そのものであると仮定する
        self.source.len()
    }

    pub fn revision(&self) -> &Revision {
        &self.revision
    }

    pub fn is_empty(&self) -> bool {
        self.len_bytes() == 0
    }
}
