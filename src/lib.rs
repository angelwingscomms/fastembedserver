use std::collections::HashMap;

use anyhow::Error;
use fastembed::{Embedding, EmbeddingModel, InitOptions, TextEmbedding};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
struct JsonData {
    resultset: ResultSet,
}

#[derive(Deserialize)]
struct ResultSet {
    row: Vec<Row>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum FieldValue {
    I32(i32),
    Str(String),
}

#[derive(Deserialize)]
struct Row {
    field: Vec<FieldValue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Embed {
    i: i32,
    v: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verse {
    pub b: i32,
    pub c: i32,
    pub v: i32,
    pub t: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerseVerbose {
    pub chapter: i32,
    pub verse: i32,
    pub book: String,
    pub text: String,
}

// type Books = Vec<String>;

// pub fn cv_count(file_path: &str, books_path: &str) -> Result<Vec<Verse>, Error> {
//     #[derive(Deserialize)]
//     // struct Verse {
//     //     b: i32,
//     //     c: i32,
//     //     v: i32
//     // }

//     #[derive(Default)]
//     struct Count {
//         c: i32,
//         v: HashMap<i32, i32>
//     }

//     let mut count: HashMap<i32, Count> = HashMap::new();

//     let rows: Vec<Verse> = serde_json::from_str(&std::fs::read_to_string(file_path)?)?;

//     let current_book: i32 = 1;
//     let current_chapter: i32 = 1;
//     let current_count: Count = Default::default();

//     for row in rows {
//         if row.b == current_book {
//             if row.c == current_chapter {
//                 // current_count.v +=1
//             } else {

//             }
//         }
//     }

//     for row in rows.resultset.row {
//         if row.field.len() >= 4 {
//             let mut r = Verse {
//                 b: if let FieldValue::I32(l) = row.field[1] {
//                     l
//                 } else {
//                     panic!("b not i32")
//                 },
//                 c: if let FieldValue::I32(l) = row.field[2] {
//                     l
//                 } else {
//                     panic!("c not i32")
//                 },
//                 v: if let FieldValue::I32(l) = row.field[3] {
//                     l
//                 } else {
//                     panic!("v not i32")
//                 },
//                 t: if let FieldValue::Str(l) = &row.field[4] {
//                     l.to_string()
//                 } else {
//                     panic!("t not string")
//                 },
//             };
//             // println!("{:#?}", r);
//             rows.push(r);
//         }
//     }
//     Ok(rows)
// }

pub fn process_verses(file_path: &str, books_path: &str) -> Result<Vec<Verse>, Error> {
    let books: Vec<String> = serde_json::from_str(&std::fs::read_to_string(books_path)?)?;
    let json_data: JsonData = serde_json::from_str(&std::fs::read_to_string(file_path)?)?;
    // let books: Books = serde_json::from_str(&std::fs::read_to_string("books.json")?)?;

    let mut rows: Vec<Verse> = Vec::new();

    for row in json_data.resultset.row {
        if row.field.len() >= 4 {
            let mut r = Verse {
                b: if let FieldValue::I32(l) = row.field[1] {
                    l
                } else {
                    panic!("b not i32")
                },
                c: if let FieldValue::I32(l) = row.field[2] {
                    l
                } else {
                    panic!("c not i32")
                },
                v: if let FieldValue::I32(l) = row.field[3] {
                    l
                } else {
                    panic!("v not i32")
                },
                t: if let FieldValue::Str(l) = &row.field[4] {
                    l.to_string()
                } else {
                    panic!("t not string")
                },
            };
            // println!("{:#?}", r);
            rows.push(r);
        }
    }
    Ok(rows)
}

pub fn count() -> anyhow::Result<()> {
    let vah_string = &std::fs::read_to_string("ylt.json")?;
    let map: HashMap<i32, Book> = serde_json::from_str(vah_string)?;
    type Book = HashMap<i32, Chapter>;
    type Chapter = HashMap<i32, String>;
    let mut new_map: HashMap<i32, BookC> = HashMap::new();
    type ChapterC = HashMap<i32, usize>;
    
    #[derive(Serialize)]
    struct BookC {
        count: usize,
        chapters: ChapterC
    }
    
    for (book_number, book) in map.iter() {
        let mut bookc = BookC {
            count: book.len(),
            chapters: HashMap::new()
        };
        for (chapter_number, chapter) in book {
            bookc.chapters.insert(*chapter_number, chapter.len());
        };
        new_map.insert(*book_number, bookc);
    }
    std::fs::write("ylt-count.json", serde_json::to_string_pretty(&new_map).unwrap()).unwrap();
    Ok(())
}

pub fn verses_as_hashes() -> anyhow::Result<()> {
    #[derive(Deserialize)]
    struct R {
        b: i32,
        c: i32,
        v: i32,
        t: String,
    }
    let vah_string = &std::fs::read_to_string("ylt.json")?;
    let verses: Vec<R> = serde_json::from_str(vah_string)?;
    println!("vah");
    let mut map: HashMap<i32, Book> = HashMap::new();
    type Book = HashMap<i32, Chapter>;
    type Chapter = HashMap<i32, String>;
    for verse in verses {
        if !map.contains_key(&verse.b) {
            map.insert(verse.b, HashMap::new());
        }
        let book = map
            .get_mut(&verse.b)
            .ok_or(anyhow::anyhow!("map.get_mut for book"))?;
        if !book.contains_key(&verse.c) {
            book.insert(verse.c, HashMap::new());
        }
        let chapter = book
            .get_mut(&verse.c)
            .ok_or(anyhow::anyhow!("map.get_mut for chapter"))?;
        if !chapter.contains_key(&verse.v) {
            chapter.insert(verse.v, verse.t);
        }
    }
    std::fs::write("ylt.json", serde_json::to_string_pretty(&map).unwrap()).unwrap();
    Ok(())
}

pub fn process_verses_verbose(
    file_path: &str,
    books_path: &str,
) -> Result<Vec<VerseVerbose>, Error> {
    let books: Vec<String> = serde_json::from_str(&std::fs::read_to_string(books_path)?)?;
    let json_data: JsonData = serde_json::from_str(&std::fs::read_to_string(file_path)?)?;
    let mut rows: Vec<VerseVerbose> = Vec::new();
    let mut book_number: i32;

    for row in json_data.resultset.row {
        book_number = if let FieldValue::I32(l) = row.field[1] {
            l
        } else {
            panic!("book number is not i32")
        };
        if row.field.len() >= 4 {
            let mut r = VerseVerbose {
                book: "".into(),
                chapter: if let FieldValue::I32(l) = row.field[2] {
                    l
                } else {
                    panic!("c not i32")
                },
                verse: if let FieldValue::I32(l) = row.field[3] {
                    l
                } else {
                    panic!("v not i32")
                },
                text: if let FieldValue::Str(l) = &row.field[4] {
                    l.to_string()
                } else {
                    panic!("t not string")
                },
            };
            r.book = books[(book_number - 1) as usize].clone();
            rows.push(r);
        }
    }
    Ok(rows)
}

pub async fn embed_verses() {
    // let mut rows_embed: Vec<Embed> = vec![];
    for row in process_verses("t_ylt.json", "books.json").unwrap() {
        // rows_embed.push(Embed {
        //     i: row.i,
        //     v: embed(&row.t).unwrap(),
        // });
        // reqwest::Client::new().put(format!("{}/collections/i/points", std::env::var("QDRANT_URL").unwrap())).bearer_auth(std::env::var("QDRANT_KEY").unwrap()).body(format!(r#"{{points: [{{"id":"{}", "payload": {{"b": {}, "c": {}, "v": {}}}, "vector": {}, }}]}}"#, row.i, row.b, row.c, row.v, serde_json::to_string(&embed(&row.t).unwrap()).unwrap())).send().await.unwrap();
        // println!("{}", row.i)
    }
    // std::fs::write(
    //     "verses_embed.json",
    //     serde_json::to_string_pretty(&process_json_file("t_ylt.json").unwrap()).unwrap(),
    // )
    // .unwrap();
}

pub fn embed(text: &str) -> Result<Embedding, Error> {
    // With default InitOptions
    // let model = TextEmbedding::try_new(Default::default())?;

    // With custom InitOptions
    let model = TextEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::MxbaiEmbedLargeV1,
        show_download_progress: true,
        ..Default::default()
    })?;

    let documents = vec![text];

    // Generate embeddings with the default batch size, 256
    let embeddings = model.embed(documents, None)?;
    Ok(embeddings[0].clone())
}

//  println!("Embeddings length: {}", embeddings.len()); // -> Embeddings length: 4
//  println!("Embedding dimension: {}", embeddings[0].len()); // -> Embedding dimension: 384
