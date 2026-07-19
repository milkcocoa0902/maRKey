# Mermaid Preview Fixture

この文書は、Mermaid code fenceの検出、描画、theme変更、構文error表示を確認するための正常系fixtureです。

## Flowchart

```mermaid
flowchart LR
    A[Markdownを編集] --> B{Previewを開く}
    B -->|通常Markdown| C[HTMLを表示]
    B -->|Mermaid| D[Diagramを描画]
    B -->|Marp| E[Slidesを表示]
    D --> F[revisionを確認]
    E --> F
```

## Sequence diagram

```mermaid
sequenceDiagram
    participant Editor
    participant Preview
    Editor->>Preview: RenderRequest(source, revision)
    Preview-->>Editor: RenderedDocument(revision)
```

通常のfenced codeはMermaidとして処理しません。

```text
flowchart LR
    A --> B
```
