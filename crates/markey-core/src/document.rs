mod edit;
mod error;
mod position;

pub use edit::TextEdit;
pub use error::DocumentError;
pub use position::{TextOffset, TextRange};

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
    fn increment(&mut self) {
        self.0 += 1
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

    pub fn apply_edit(&mut self, edit: TextEdit) -> Result<(), DocumentError> {
        let s = edit.range().start().value();
        let e = edit.range().end().value();

        self.source.replace_range(s..e, edit.replacement());
        self.revision.increment();
        Ok(())
    }
}
