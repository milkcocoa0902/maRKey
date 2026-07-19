# Fixtures

maRKeyの自動testと手動runtime確認で共有する、決まった入力データです。すべてUTF-8で管理します。

## Fixture一覧

| Path | Lines | Bytes | Purpose |
|---|---:|---:|---|
| `small-ja.md` | 40 | 905 | 日本語IME、選択置換、改行、copy/paste |
| `mixed-content.md` | 70 | 1,585 | Markdown要素、Unicode、長い行、code fence、相対asset |
| `mermaid.md` | 32 | 721 | Mermaid flowchart、sequence diagram、通常code fenceとの区別 |
| `marp.md` | 36 | 442 | `marp: true`、複数slide、日本語、code block |
| `assets/preview-grid.svg` | 13 | 878 | workspace内の相対画像読込、scale、SVG表示 |
| `generated/large.md` | 100,000 | 13,699,752 | editorのload、edit、scroll、save性能。generatorから作成 |

line数とbyte数はLF改行での値です。fixtureを意図的に変更した場合は、この表も更新します。

## 使い方

read-only表示やpreview確認ではfixtureを直接開けます。入力、保存、外部変更競合などfileを書き換える検証では、baselineを壊さないようtemporary workspaceへコピーして使います。

```text
fixtures/small-ja.md
  -> temporary workspaceへcopy
  -> maRKeyでopen/edit/save
  -> 期待内容を確認
  -> temporary workspaceを破棄
```

用途ごとの基本fixtureは次の通りです。

- Editor/IME: `small-ja.md`
- Unicode、layout、outline: `mixed-content.md`
- Mermaid preview: `mermaid.md`
- Marp preview: `marp.md`
- Relative asset security: `mixed-content.md`から`assets/preview-grid.svg`を参照

## Baseline確認

```text
wc -l -c fixtures/*.md fixtures/assets/preview-grid.svg
sha256sum fixtures/*.md fixtures/assets/preview-grid.svg
```

## Large Fixture

大規模文書は手作業で保守せず、次のcommandで生成します。

```text
cargo run -p xtask -- generate-large-fixture
```

期待値:

```text
path: fixtures/generated/large.md
lines: 100000
bytes: 13699752
fnv1a64: 8170f2a8fad18c07
```

同じgeneratorを2回実行しても、この3つの値は変わりません。`fixtures/generated/`はGit管理対象外です。

Runtime確認の記録先:

- [Ubuntu GNOME Wayland](../docs/validation/ubuntu-wayland.md)
- [Windows](../docs/validation/windows.md)
- [Performance](../docs/validation/performance.md)
