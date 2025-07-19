use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

// External crates needed in Cargo.toml:
// fuzzy-matcher = "0.3"
// once_cell = "1.18"
// regex = "1.8"
// word-to-num = "0.1"

static ORDINALS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("first", "1");
    m.insert("second", "2");
    m.insert("third", "3");
    m
});

static ORDINAL_RULES: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
    [
        ("samuel", 2),
        ("kings", 2),
        ("chronicles", 2),
        ("corinthians", 2),
        ("thessalonians", 2),
        ("timothy", 2),
        ("peter", 2),
    ]
    .iter()
    .cloned()
    .collect()
});

#[rustfmt::skip]
static BIBLE_STRUCTURE: Lazy<HashMap<&'static str, Vec<usize>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Genesis", vec![31,25,24,26,32,22,24,22,29,32,32,20,18,24,21,16,27,33,38,18,34,24,20,67,34,35,46,22,35,43,55,32,20,31,29,43,36,30,23,23,57,38,34,34,28,34,31,22,33,26]);
    // add remaining books
    map.insert("Revelation", vec![20,29,22,11,14,17,17,13,21,11,19,17,18,20,8,21,18,24,21,15,21,21]);

    map
});

static BOOKS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    BIBLE_STRUCTURE
        .keys()
        .map(|&b| b.to_lowercase().as_str())
        .collect()
});

static REF_RE: Lazy<Regex> = Lazy::new(|| {
    let books = BIBLE_STRUCTURE
        .keys()
        .map(|b| regex::escape(&b.to_lowercase()))
        .collect::<Vec<_>>()
        .join("|");
    let pattern = format!(
        r"(?xi)
        \b
        (?:(\d+)\s+)?
        ({books})
        \s+(?:chapter\s+)?([\w\s\-]+?)(?=\s+verse\b|$)
        (?:\s+verse(?:s)?\s+([\w\s\-]+))?
        (?:\s*(?:[-–—]|to|through|and)\s+([\w\s\-]+))?
        \b
    "
    );
    Regex::new(&pattern).unwrap()
});

/// Normalize ordinals like "first" → "1"
fn normalize_ordinals(text: &str) -> String {
    let mut s = text.to_string();
    for (word, digit) in ORDINALS.iter() {
        let re = Regex::new(&format!(r"(?i)\\b{}\\b", word)).unwrap();
        s = re.replace_all(&s, *digit).into_owned();
    }

    s
}

/// Convert word tokens to numbers, e.g. "twenty one" → "21"
fn word_to_number(token: &str) -> Option<String> {
    let tok = token.to_lowercase().replace('-', " ").trim().to_string();
    if tok.is_empty() {
        return None;
    }
    if let Ok(n) = tok.parse::<usize>() {
        return Some(n.to_string());
    }
    // Use word_to_num crate
    match word_to_num::parse(&tok) {
        Ok(n) => Some(n.to_string()),
        Err(_) => None,
    }
}

/// Fuzzy-match a candidate book against BIBLE_STRUCTURE
fn fuzzy_book_match(candidate: &str) -> Option<String> {
    use fuzzy_matcher::skim::SkimMatcherV2;
    use fuzzy_matcher::FuzzyMatcher;

    let matcher = SkimMatcherV2::default();
    let mut best: Option<(&str, i64)> = None;
    for &book in BIBLE_STRUCTURE.keys() {
        if let Some(score) = matcher.fuzzy_match(&book.to_lowercase(), &candidate.to_lowercase()) {
            if best.map_or(true, |(_, s)| score > s) {
                best = Some((book, score));
            }
        }
    }
    if let Some((name, score)) = best {
        if score >= 80 {
            return Some(name.to_string().to_lowercase());
        }
    }
    None
}

/// Extract Bible references from an input text
pub fn extract_bible_reference(input: &str) -> Vec<String> {
    let mut text = normalize_ordinals(input);
    text = text.replace("psalms", "psalm");
    text = text.replace("revelations", "revelation");
    text = text.replace("songs of solomon", "song of solomon");

    let mut results = Vec::new();
    for cap in REF_RE.captures_iter(&text) {
        let ord_raw = cap.get(1).map(|m| m.as_str());
        let book_raw = cap.get(2).unwrap().as_str();
        let chap_raw = cap.get(3).unwrap().as_str();
        let verse_start_raw = cap.get(4).map(|m| m.as_str()).unwrap_or("");
        let verse_end_raw = cap.get(5).map(|m| m.as_str()).unwrap_or("");

        let ord_num = ord_raw.and_then(|o| o.parse::<usize>().ok());
        let fuzzy = fuzzy_book_match(book_raw);
        if fuzzy.is_none() {
            continue;
        }

        let fb = fuzzy.unwrap();

        // Determine book key
        let book_key = if fb == "john" {
            if let Some(n) = ord_num {
                format!("{} john", n)
            } else {
                "john".into()
            }
        } else if let Some(&max_ord) = ORDINAL_RULES.get(fb.as_str()) {
            if ord_num.unwrap_or(0) == 0 || ord_num.unwrap() > max_ord {
                continue;
            }
            format!("{} {}", ord_num.unwrap(), fb)
        } else {
            fb.clone()
        };
        let book = book_key
            .split(' ')
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        // Chapter & verse conversion
        let chap = match word_to_number(chap_raw) {
            Some(c) => c,
            None => continue,
        };
        let start = word_to_number(verse_start_raw).unwrap_or_else(|| "1".into());
        let end = word_to_number(verse_end_raw);

        let chap_n: usize = chap.parse().unwrap();
        let start_n: usize = start.parse().unwrap();
        let verses = match BIBLE_STRUCTURE.get(book.as_str()) {
            Some(v) => v,
            None => continue,
        };
        if chap_n == 0 || chap_n > verses.len() {
            continue;
        }
        if start_n == 0 || start_n > verses[chap_n - 1] {
            continue;
        }

        let mut reference = format!("{} {}:{}", book, chap_n, start_n);
        if let Some(e) = end {
            let end_n: usize = e.parse().unwrap();
            if end_n >= start_n && end_n <= verses[chap_n - 1] {
                reference = format!("{}-{}", reference, end_n);
            }
        }
        results.push(reference);
    }

    results
}
