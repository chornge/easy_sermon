use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
struct Verse {
    verse: u32,
    text: String,
}

#[derive(Debug, Deserialize)]
struct Chapter {
    chapter: u32,
    verses: Vec<Verse>,
}

#[derive(Debug, Deserialize)]
struct Book {
    name: String,
    chapters: Vec<Chapter>,
}

#[derive(Debug, Deserialize)]
struct BibleJson {
    books: Vec<Book>,
}

/// Bible data structure: Book -> Chapter -> Verse -> Text
type BibleMap = HashMap<String, HashMap<String, HashMap<String, String>>>;

fn load_bible(path: &str) -> Result<BibleMap> {
    let data = fs::read_to_string(path)?;
    let raw: BibleJson = serde_json::from_str(&data)?;

    let mut bible = HashMap::new();

    for book in raw.books {
        let mut chapters_map = HashMap::new();
        for chapter in book.chapters {
            let mut verses_map = HashMap::new();
            for verse in chapter.verses {
                verses_map.insert(verse.verse.to_string(), verse.text.trim().to_string());
            }
            chapters_map.insert(chapter.chapter.to_string(), verses_map);
        }
        bible.insert(book.name, chapters_map);
    }

    Ok(bible)
}

fn bible_offline(reference: &str, bible: &BibleMap) -> String {
    let parts: Vec<&str> = reference.rsplitn(2, ' ').collect();
    if parts.len() < 2 {
        return format!("Verse not found: {reference}");
    }

    let chapter_verse = parts[0];
    let book = parts[1];

    if let Some(chapter_split) = chapter_verse.split_once(':') {
        let chapter = chapter_split.0;
        let verse_part = chapter_split.1;

        if let Some((start_str, end_str)) = verse_part.split_once('-') {
            // Handle verse range
            if let (Ok(start), Ok(end)) = (start_str.parse::<u32>(), end_str.parse::<u32>()) {
                let mut lines = Vec::new();
                for v in start..=end {
                    if let Some(text) = bible
                        .get(book)
                        .and_then(|c| c.get(chapter))
                        .and_then(|vmap| vmap.get(&v.to_string()))
                    {
                        let line = format!("{book} {chapter}:{v} — {text}");
                        println!("{}", line);
                        lines.push(line);
                    }
                }
                return lines.join("\n");
            }
        } else {
            // Single verse
            if let Some(text) = bible
                .get(book)
                .and_then(|c| c.get(chapter))
                .and_then(|vmap| vmap.get(verse_part))
            {
                let line = format!("{reference} — {text}");
                println!("{}", line);
                return line;
            }
        }
    }

    format!("Verse not found: {reference}")
}

#[allow(dead_code)]
pub async fn stage_display(verse: &str) -> Result<()> {
    let request_obj = serde_json::json!({
        "url": "v1/stage/message",
        "method": "PUT",
        "body": verse,
        "chunked": false
    });

    let request_str = format!("{}\r\n", request_obj);

    let addr = format!("{}:{}", "localhost", "54346");

    let full_verse = bible_offline(verse, &load_bible("translations/akjv/akjv.json")?);

    match TcpStream::connect(addr).await {
        Ok(mut stream) => {
            stream.write_all(request_str.as_bytes()).await?;
            stream.flush().await?;
            println!("✅ {full_verse} is on Stage Display");
        }
        Err(e) => {
            eprintln!("❌ Error sending to Stage Display: {e}");
        }
    }

    Ok(())
}
