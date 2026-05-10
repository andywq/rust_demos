use std::fs;
use std::env;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct StatEntry {
    key: String,
    count: u32,
}

/// Counts keys using the rules:
/// - Scan left to right; skip whitespace (not counted, not part of keys).
/// - If the current char is ASCII alphabetic (English letter), consume until the next
///   Unicode whitespace; that substring is one key.
/// - Otherwise, one non-whitespace char is one key.
fn analyze(input: &str) -> Vec<StatEntry> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        if c.is_ascii_alphabetic() {
            let start = i;
            i += 1;
            while i < chars.len() && !chars[i].is_whitespace() {
                i += 1;
            }
            let key: String = chars[start..i].iter().collect();
            *counts.entry(key).or_insert(0) += 1;
        } else {
            let key = c.to_string();
            *counts.entry(key).or_insert(0) += 1;
            i += 1;
        }
    }

    let mut out: Vec<StatEntry> = Vec::new();
    for (key, count) in counts {
        out.push(StatEntry { key, count });
    }

    out.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.key.cmp(&b.key)));
    out
}

/// Returns JSON array: `[{"key":"...","count":n}, ...]` sorted by count desc, key asc.
fn word_count(input: &str) -> String {
    let stats = analyze(input);
    serde_json::to_string(&stats).unwrap_or_else(|_| "[]".to_string())
}

fn main() -> std::io::Result<()> {
    let path = env::args()
        .nth(1)
        .expect("usage: program <path-to-file>");

    let contents: String = fs::read_to_string(path)?;

    println!("{}", word_count(contents.as_str()));

    Ok(())
}
