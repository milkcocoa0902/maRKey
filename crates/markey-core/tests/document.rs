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
