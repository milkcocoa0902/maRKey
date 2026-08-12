use markey_core::document;

#[test]
fn set_document_id_test() {
    let raw_id = 10;
    let document_id = document::DocumentId::new(raw_id);
    assert_eq!(raw_id, document_id.value());
}

#[test]
fn revision_default() {
    let initial_revision = 0;
    let revision = document::Revision::default();
    assert_eq!(initial_revision, revision.value())
}

#[test]
fn document_returns_document_id_test() {
    let raw_id = 10;
    let document_id = document::DocumentId::new(raw_id);
    let doc = document::Document::new(document_id, "".to_string());

    assert_eq!(raw_id, doc.id().value());
}

#[test]
fn always_new_document_has_default_revision() {
    let initial_revision = 0;
    let document_id = 1;
    let source = "";
    let doc = document::Document::new(document::DocumentId::new(document_id), source.to_string());

    assert_eq!(initial_revision, doc.revision().value());
}

#[test]
fn source_must_be_kept() {
    let source = "english document";

    let document_id = 1;
    let doc = document::Document::new(document::DocumentId::new(document_id), source.to_string());
    assert_eq!(source, doc.source());
}

#[test]
fn english_document_len_bytes() {
    let source = "english document";
    // String#len()は文字数ではなくバイト数を返す
    let source_len = source.len();

    let document_id = 1;
    let doc = document::Document::new(document::DocumentId::new(document_id), source.to_string());
    assert_eq!(source_len, doc.len_bytes());
}

#[test]
fn japanese_document_len_bytes() {
    let source = "日本語文章";
    // String#len()は文字数ではなくバイト数を返す
    let source_len = source.len();

    let document_id = 1;
    let doc = document::Document::new(document::DocumentId::new(document_id), source.to_string());
    assert_eq!(source_len, doc.len_bytes());
}

#[test]
fn empty_document() {
    let source = "";
    let document_id = 1;
    let doc = document::Document::new(document::DocumentId::new(document_id), source.to_string());
    assert!(doc.is_empty());
}

#[test]
fn non_empty_document() {
    let source = "non empty document";
    let document_id = 1;
    let doc = document::Document::new(document::DocumentId::new(document_id), source.to_string());
    assert!(!doc.is_empty());
}

// Offset test
#[test]
fn text_offset_keeps_positive_value() {
    let raw_value = 10;
    let text_offset = document::TextOffset::new(raw_value);

    assert_eq!(raw_value, text_offset.value())
}

#[test]
fn text_offset_must_be_comparable() {
    let offset_1 = document::TextOffset::new(1);
    let offset_2 = document::TextOffset::new(1);

    assert_eq!(offset_1, offset_2)
}

#[test]
fn text_range_preserved_boundaries() {
    let start = document::TextOffset::new(1);
    let end = document::TextOffset::new(5);

    let range = document::TextRange::new(start, end);

    assert_eq!(start, range.start());
    assert_eq!(end, range.end());
}

#[test]
fn text_edit_preserved_range_and_replacement() {
    let start = document::TextOffset::new(1);
    let end = document::TextOffset::new(4);

    let range = document::TextRange::new(start, end);

    let replacement = "replace with me";

    let edit = document::TextEdit::new(range, replacement.to_string());

    assert_eq!(range, edit.range());
    assert_eq!(replacement, edit.replacement());
}

#[test]
fn text_edit_minimul_insert() {
    let replacement = "X";
    let original = "ab";

    let mut document = document::Document::new(document::DocumentId::new(1), original.to_string());
    let current_revision = document.revision().value();

    let range =
        document::TextRange::new(document::TextOffset::new(1), document::TextOffset::new(1));
    let edit = document::TextEdit::new(range, replacement.to_string());

    let result = document.apply_edit(edit);
    assert!(result.is_ok());
    assert_eq!(document.revision().value(), current_revision + 1);
    assert_eq!(document.source(), "aXb");
}
