use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

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
static BIBLE_MAP: Lazy<HashMap<&'static str, Vec<usize>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // Old Testament
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
    map.insert("Job", vec![22,13,26,21,27,30,21,22,35,22,20,25,28,22,35,22,16,21,29,29,34,30,17,25,6,14,23,28,25,31,40,22,33,37,16,33,24,41,30,24,34,17]);
    map.insert("Psalm", vec![6,12,8,8,12,10,17,9,20,18,7,8,6,7,5,11,15,50,14,9,13,31,6,10,22,12,14,9,11,12,24,11,22,22,28,12,40,22,13,17,13,11,5,26,17,11,9,14,20,23,19,9,6,7,23,13,11,11,17,12,8,12,11,10,13,20,7,35,36,5,24,20,28,23,10,12,20,72,13,19,16,8,18,12,13,17,7,18,52,17,16,15,5,23,11,13,12,9,9,5,8,29,22,35,45,48,43,13,31,7,10,10,9,8,18,19,2,29,176,7,8,9,4,8,5,6,5,6,8,8,3,18,3,3,21,26,9,8,24,13,10,7,12,15,21,10,20,14,9,6]);
    map.insert("Proverbs", vec![33,22,35,27,23,35,27,36,18,32,31,28,25,35,33,33,28,24,29,30,31,29,35,34,28,28,27,28,27,33,31]);
    map.insert("Ecclesiastes", vec![18,26,22,16,20,12,29,17,18,20,10,14]);
    map.insert("Song Of Solomon", vec![17,17,11,16,16,12,14,14]);
    map.insert("Isaiah", vec![31,22,25,6,30,13,22,22,21,34,16,6,22,32,9,14,14,7,25,6,17,25,18,23,12,21,13,29,24,33,9,20,24,17,10,22,38,22,8,31,29,25,28,28,25,13,15,22,26,11,23,15,12,17,13,12,21,14,21,22,11,12,19,12,25,24,23,23,57,30,34,34,28,34,31,22,44]);
    map.insert("Jeremiah", vec![19,37,25,31,31,30,34,22,26,25,23,17,27,22,21,21,27,23,15,18,14,30,40,10,38,24,22,17,32,24,40,44,26,22,19,32,21,28,18,16,18,22,31,7,9,28,23,27,22,17,27,21,23,15,18,14,30,40,10,38,24,22,17,32,24,40,44,26,22,19,32,21,28,18,16,18,22,31,7,9,28,23,27,22]);
    map.insert("Lamentations", vec![22,22,66,22,22]);
    map.insert("Ezekiel", vec![28,10,27,17,17,14,27,18,11,22,25,28,23,23,9,27,17,22,26,20,27,31,25,24,23,35,40,23,35,27,23,34,16,33,24,23,38,23,29,49,26,20,27,31,25,24,23,35,40,23,35,27,23,34,16,33,24,23,38,23,29]);
    map.insert("Daniel", vec![21,49,30,37,31,28,28,27,27,21,45,13]);
    map.insert("Hosea", vec![11,23,5,19,15,11,16,14,17,15,12,15,11,17]);
    map.insert("Joel", vec![20,32,21]);
    map.insert("Amos", vec![15,16,15,13,27,14,17,14,15]);
    map.insert("Obadiah", vec![21]);
    map.insert("Jonah", vec![17,10,10,11]);
    map.insert("Micah", vec![16,13,12,13,15,16,20]);
    map.insert("Nahum", vec![15,13,19]);
    map.insert("Habakkuk", vec![17,20,19]);
    map.insert("Zephaniah", vec![18,15,20]);
    map.insert("Haggai", vec![15,23]);
    map.insert("Zechariah", vec![17,17,10,14,9,11,16,6,14,10,8,12,14,18]);
    map.insert("Malachi", vec![14,17,18,6]);
    // New Testament
    map.insert("Matthew", vec![25,23,17,25,48,34,29,34,38,42,30,50,58,36,39,28,27,35,30,34,46,46,39,51,46,75,66,20]);
    map.insert("Mark", vec![45,28,35,41,43,56,37,38,50,52,33,44,37,72,47,20]);
    map.insert("Luke", vec![80,52,38,44,39,49,50,56,62,42,54,59,35,35,32,31,37,43,48,47,38,71,56,53,59,37]);
    map.insert("John", vec![51,25,36,54,47,71,53,59,41,42,57,50,38,31,27,33,26,40,42,31,25]);
    map.insert("Acts", vec![26,47,26,37,42,15,60,40,43,48,30,25,52,28,41,40,34,28,41,38,40,30,35,27,27,32,44,31]);
    map.insert("Romans", vec![32,29,31,25,21,23,25,39,33,21,36,21,14,23,33,27]);
    map.insert("1 Corinthians", vec![31,16,23,21,13,20,40,13,27,33,34,31,13,40,58,24]);
    map.insert("2 Corinthians", vec![24,17,18,18,21,18,16,24,15,18,33,21,13]);
    map.insert("Galatians", vec![24,21,29,31,26,18]);
    map.insert("Ephesians", vec![23,22,21,32,33,24]);
    map.insert("Philippians", vec![30,30,21,23]);
    map.insert("Colossians", vec![29,23,25,18]);
    map.insert("1 Thessalonians", vec![10,20,13,18,28]);
    map.insert("2 Thessalonians", vec![12,17,18]);
    map.insert("1 Timothy", vec![20,15,16,16,25,21]);
    map.insert("2 Timothy", vec![18,26,17,22]);
    map.insert("Titus", vec![16,15,15]);
    map.insert("Philemon", vec![25]);
    map.insert("Hebrews", vec![14,18,19,16,14,20,28,13,28,39,40,29,25]);
    map.insert("James", vec![27,26,18,17,20]);
    map.insert("1 Peter", vec![25,25,22,19,14]);
    map.insert("2 Peter", vec![21,22,18]);
    map.insert("1 John", vec![10,29,24,21,21]);
    map.insert("2 John", vec![13]);
    map.insert("3 John", vec![14]);
    map.insert("Jude", vec![25]);
    map.insert("Revelation", vec![20,29,22,11,14,17,17,13,21,11,19,17,18,20,8,21,18,24,21,15,21,21]);

    map
});

static REF_RE: Lazy<Regex> = Lazy::new(|| {
    let books = BIBLE_MAP
        .keys()
        .map(|b| regex::escape(&b.to_lowercase()))
        .collect::<Vec<_>>()
        .join("|");
    let pat = format!(
        r"(?i)\b(?:(\d+)\s+)?({books})\s+(?:chapter\s+)?([\w\s-]+)\s+verses?\s+([\w\s-]+?)(?:\s*(?:-|–|—|to|through|and)\s+([\w\s-]+))?\b"
    );
    Regex::new(&pat).unwrap()
});

/// - Normalize ordinals like "first" → "1", "second" → "2", etc.
/// - Also handles "1st", "2nd", "3rd", etc.
fn normalize_ordinals(text: &str) -> String {
    let mut s = text.to_string();
    for (word, digit) in ORDINALS.iter() {
        let re = Regex::new(&format!(r"(?i)\b{}\b", regex::escape(word))).unwrap();
        s = re.replace_all(&s, *digit).into_owned();
    }
    s
}

///   - vs. becomes verses, v. becomes verse
///   - revelations becomes revelation, etc.
fn normalize_text(input: &str) -> String {
    let mut text = normalize_ordinals(input);
    text = Regex::new(r"(?i)\bvs\.?\b")
        .unwrap()
        .replace_all(&text, "verses")
        .into_owned();
    text = Regex::new(r"(?i)\bv\.?\b")
        .unwrap()
        .replace_all(&text, "verse")
        .into_owned();

    text.replace("psalms", "psalm")
        .replace("revelations", "revelation")
        .replace("songs of solomon", "song of solomon")
}

#[allow(dead_code)]
/// Extract a Bible verse from an input text
pub fn bible_verse(input: &str) -> Vec<String> {
    let text = normalize_text(input);

    let mut results = Vec::new();
    for cap in REF_RE.captures_iter(&text) {
        let ord_raw = cap.get(1).map(|m| m.as_str());
        let book_raw = cap.get(2).unwrap().as_str();
        let ord_num = ord_raw.and_then(|o| o.parse::<usize>().ok());
        let fuzzy = match fuzzy_book_match(book_raw) {
            Some(f) => f,
            None => continue,
        };

        let book_key = if fuzzy == "john" {
            if let Some(n) = ord_num {
                format!("{n} john")
            } else {
                "john".into()
            }
        } else if let Some(&max_ord) = ORDINAL_RULES.get(fuzzy.as_str()) {
            let n = ord_num.unwrap_or(0);
            if n == 0 || n > max_ord {
                continue;
            }
            format!("{n} {fuzzy}")
        } else {
            fuzzy.clone()
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

        // chapter & verse parsing
        let chap_raw = cap.get(3).unwrap().as_str().trim();
        let verse_start_raw = cap.get(4).map(|m| m.as_str()).unwrap_or("").trim();
        let verse_end_raw = cap.get(5).map(|m| m.as_str()).unwrap_or("").trim();

        // parse chapter number or skip
        let chap_n: usize = match word_to_number(chap_raw).and_then(|s| s.parse::<usize>().ok()) {
            Some(n) => n,
            None => continue,
        };

        // parse start‐verse (default 1 if empty) or skip
        let start_n: usize = if verse_start_raw.is_empty() {
            1
        } else {
            match word_to_number(verse_start_raw).and_then(|s| s.parse::<usize>().ok()) {
                Some(n) => n,
                None => continue,
            }
        };

        // parse end‐verse if given
        let end_n: Option<usize> = if verse_end_raw.is_empty() {
            None
        } else {
            match word_to_number(verse_end_raw).and_then(|s| s.parse::<usize>().ok()) {
                Some(n) => Some(n),
                None => continue,
            }
        };

        // validate against the Bible
        let verses = match BIBLE_MAP.get(book.as_str()) {
            Some(v) => v,
            None => continue,
        };
        if chap_n == 0 || chap_n > verses.len() {
            continue;
        }
        if start_n == 0 || start_n > verses[chap_n - 1] {
            continue;
        }

        let mut reference = format!("{book} {chap_n}:{start_n}");
        if let Some(e) = end_n {
            if e >= start_n && e <= verses[chap_n - 1] {
                reference = format!("{reference}-{e}");
            }
        }
        results.push(reference);
    }

    results
}

/// Map for converting words to numbers, e.g. "twenty" → "20"
pub mod word_to_num {
    use std::collections::HashMap;

    pub fn parse(tok: &str) -> Result<u32, &'static str> {
        let nums: HashMap<&str, u32> = [
            ("o", 0),
            ("oh", 0),
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("ten", 10),
            ("eleven", 11),
            ("twelve", 12),
            ("thirteen", 13),
            ("fourteen", 14),
            ("fifteen", 15),
            ("sixteen", 16),
            ("seventeen", 17),
            ("eighteen", 18),
            ("nineteen", 19),
            ("twenty", 20),
            ("thirty", 30),
            ("forty", 40),
            ("fifty", 50),
            ("sixty", 60),
            ("seventy", 70),
            ("eighty", 80),
            ("ninety", 90),
            ("hundred", 100),
        ]
        .iter()
        .cloned()
        .collect();
        nums.get(tok).cloned().ok_or("Invalid number word")
    }
}

fn word_to_number(token: &str) -> Option<String> {
    // 1) Normalize casing & hyphens, and bind to keep alive for the borrows below
    let normalized = token.to_lowercase().replace('-', " ");
    let parts: Vec<&str> = normalized
        .split_whitespace()
        .map(str::trim)
        .filter(|w| !w.is_empty())
        .collect();
    if parts.is_empty() {
        return None;
    }

    // 2) If it's a raw digit, short-circuit
    if parts.len() == 1 {
        if let Ok(n) = parts[0].parse::<usize>() {
            return Some(n.to_string());
        }
    }

    // 3) Otherwise do a scale-aware parse:
    //    total = the running grand total
    //    current = the subtotal before hitting a scale (e.g. hundred)
    //
    let mut total = 0usize;
    let mut current = 0usize;
    for &w in &parts {
        match word_to_num::parse(w) {
            Ok(100) => {
                // “hundred” multiplies the current (or 1 if none)
                if current == 0 {
                    current = 1;
                }
                current *= 100;
            }
            Ok(val) => {
                // units/tens just add
                current += val as usize;
            }
            Err(_) => {
                // if any token fails, bail
                return None;
            }
        }
    }
    total += current;

    Some(total.to_string())
}

/// Fuzzy-match a candidate book against BIBLE_MAP
fn fuzzy_book_match(candidate: &str) -> Option<String> {
    use fuzzy_matcher::FuzzyMatcher;
    use fuzzy_matcher::skim::SkimMatcherV2;

    let matcher = SkimMatcherV2::default();
    let mut best: Option<(&str, i64)> = None;
    for &book in BIBLE_MAP.keys() {
        if let Some(score) = matcher.fuzzy_match(&book.to_lowercase(), &candidate.to_lowercase()) {
            if best.is_none_or(|(_, s)| score > s) {
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
