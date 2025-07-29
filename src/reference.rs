use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

/* External crates needed in Cargo.toml:
fuzzy-matcher = "0.3"
once_cell = "1.18"
regex = "1.8"
word-to-num = "0.1"
*/
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
    map.insert("Exodus", vec![22,25,22,31,23,30,29,28,35,29,10,51,22,31,27,36,16,27,25,26,37,30,33,18,40,37,21,43,46,38,18,35,23,35,35,38,29,31,43,38]);
    map.insert("Leviticus", vec![17,16,17,35,26,23,38,36,24,20,47,8,59,57,33,34,16,30,33,24,23,55,46,34,34,28,34]);
    map.insert("Numbers", vec![54,34,51,49,31,27,89,26,23,36,35,16,33,45,41,35,28,32,22,29,35,41,30,25,19,36,37,27,31,33,33,35,23,34,46,34]);
    map.insert("Deuteronomy", vec![46,37,29,49,33,25,26,20,29,22,32,31,19,29,23,22,20,22,21,20,23,29,26,22,19,19,26,68,29,20,30,52,29,12]);
    map.insert("Joshua", vec![18,24,17,24,15,27,26,35,27,43,23,24,33,15,63,10,18,28,51,9,45,34,16,33]);
    map.insert("Judges", vec![36,23,31,24,31,40,25,35,57,18,40,15,25,20,20,31,13,31,30,48,25]);
    map.insert("Ruth", vec![22,23,18,22]);
    map.insert("1 Samuel", vec![28,36,21,22,12,21,17,22,24,31,15,25,23,52,35,23,58,30,24,42,15,23,29,22,44,25,12,25,11,31,13]);
    map.insert("2 Samuel", vec![27,32,39,12,25,23,29,18,13,19,27,31,39,33,37,23,29,33,43,26,22,51,39,25]);
    map.insert("1 Kings", vec![53,46,28,34,18,38,51,66,28,29,43,33,34,31,34,34,24,46,21,43,29,53]);
    map.insert("2 Kings", vec![18,25,27,44,27,33,20,29,37,36,21,21,25,29,38,20,41,37,37,21,26,20,37,20,30]);
    map.insert("1 Chronicles", vec![54,55,24,43,26,81,40,40,44,14,47,41,14,17,29,43,27,17,19,8,30,19,32,31,31,32,34,21,30]);
    map.insert("2 Chronicles", vec![17,18,17,22,14,42,22,18,31,19,23,16,23,14,19,14,19,34,11,37,20,12,21,27,28,23,9,27,36,27,21,33,25,33,27,23]);
    map.insert("Ezra", vec![11,70,13,24,17,22,28,36,15,44]);
    map.insert("Nehemiah", vec![11,20,32,23,19,19,73,18,38,39,36,47,31]);
    map.insert("Esther", vec![22,23,15,17,14,14,10,17,32,3]);
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
