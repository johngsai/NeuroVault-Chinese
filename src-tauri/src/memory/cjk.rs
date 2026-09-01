//! Shared CJK language infrastructure for the memory layer.
//!
//! The embedding model, BM25 keyword branch and entity extractor all
//! serve Chinese content now (`bge-small-zh-v1.5` embedder; see
//! `embedder.rs`). This module holds the pieces those consumers share:
//! a single jieba instance (the embedded dictionary is built once per
//! process, not once per consumer) and the CJK detection predicate.
//!
//! `jieba-rs`'s default dictionary is compiled into the binary
//! (`default-dict` feature) — zero runtime downloads, fully local, no
//! Python.

use once_cell::sync::Lazy;

use jieba_rs::Jieba;

/// Shared jieba instance. `Jieba::cut`/`tag` take `&self`, so one
/// shared instance is safe under our serialised usage (BM25 rebuilds
/// and entity extraction don't run concurrently on the same text, and
/// jieba's own state is read-only after construction). The embedded
/// dictionary is decompressed + parsed on first access, so keeping it
/// in one `Lazy` matters — every consumer would otherwise pay that cost.
pub static JIEBA: Lazy<Jieba> = Lazy::new(Jieba::new);

/// True when `s` contains any CJK Unified Ideograph. The ASCII
/// `[a-z0-9]` token regexes elsewhere in the codebase match zero CJK
/// characters, so CJK-bearing text must route through jieba instead.
pub fn contains_cjk(s: &str) -> bool {
    s.chars().any(|c| {
        matches!(c,
            '\u{3400}'..='\u{4DBF}' // CJK Ext A
            | '\u{4E00}'..='\u{9FFF}' // CJK Unified Ideographs
            | '\u{F900}'..='\u{FAFF}' // CJK Compatibility Ideographs
            | '\u{20000}'..='\u{2A6DF}' // CJK Ext B
            | '\u{2A700}'..='\u{2B81F}' // CJK Ext C/D
            | '\u{2F800}'..='\u{2FA1F}' // CJK Compatibility Supp
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_cjk_and_ascii() {
        assert!(contains_cjk("北京烤鸭"));
        assert!(contains_cjk("用 fastembed 嵌入中文"));
        assert!(!contains_cjk("the quick brown fox"));
        assert!(!contains_cjk("2023-07-01"));
    }
}
