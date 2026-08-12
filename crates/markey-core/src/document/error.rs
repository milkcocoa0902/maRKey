#[derive(Debug)]
pub enum DocumentError {
    InvertedRange,
    OffsetOverRun,
    InvalidOffsetBoundary,
    CorruptRevision,
}
