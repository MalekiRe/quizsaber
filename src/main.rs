use std::path::Path;
use rusqlite::Connection;
use anyhow::Result;
use serde_json::{from_str, to_string_pretty, Value};

fn main() -> Result<()>{
    //let conn = Connection::open(Path::new("/home/malek/Downloads/anki-example/collection.anki2"))?;
    quizsaber::my_func().unwrap();
    //quizsaber::anki_parser::parse().unwrap();

    // let mut statement = conn.prepare("SELECT models FROM col")?;
    // let models = statement.query_map([], |row| {
    //     let str: String = row.get(0)?;
    //     let json_model: Value = from_str(str.as_str()).unwrap();
    //     Ok(json_model)
    // })?;
    //
    // let mut i = 0;
    // models.for_each(|model| {
    //     if i == 1 {
    //         return;
    //     }
    //     i += 1;
    //    let mut model = model.unwrap();
    //     let mut i2 = 0;
    //     model.as_object().unwrap().iter().for_each(|my_map| {
    //         if i2 == 1 {
    //             return;
    //         }
    //         i2 += 1;
    //         let val = to_string_pretty(my_map.1).unwrap();
    //         println!("{}", val);
    //     });
    //     //println!("{}", model);
    // });


    // let mut stmt = conn.prepare("SELECT nid FROM cards")?;
    // let card_ids = stmt.query_map([], |row| {
    //     let card_id: u64 = row.get(0)?;
    //    Ok(card_id)
    // })?;
    // let model_ids = conn.prepare("SELECT ")

    // let mut val = 0;
    // card_ids.for_each(|card_id| {
    //     if val == 20 {
    //         return;
    //     }
    //     val += 1;
    //     let card_id = card_id.unwrap();
    //     let mut stmt = conn.prepare(format!("SELECT flds FROM notes WHERE id={}", card_id).as_str()).unwrap();
    //     let field = stmt.query_map([], |row| {
    //        let val: String = row.get(0).unwrap();
    //         Ok(val)
    //     }).unwrap();
    //     field.for_each(|f| {
    //        let mut f = f.unwrap();
    //         f = f.replace(0x1f as char, "$$$$");
    //         println!("field: {}", f);
    //     });
    //     println!("card id: {}", card_id);
    // });
    // panic!("");
    // let mut stmt = conn.prepare("SELECT flds FROM notes")?;
    // let res = stmt.query_map([], |row| {
    //     let val: String = row.get(0).unwrap();
    //     println!("{}", val);
    //     Ok(val)
    // })?;
    // res.for_each(|s| {
    //    let s = s.unwrap();
    //     println!("{}", s);
    // });
    // let mut stmt = conn.prepare("SELECT models FROM col")?;
    // let x = stmt.query_map([], |row| {
    //     let str: String = row.get_unwrap(0);
    //     //println!("{}", str);
    //     Ok(str)
    // })?;
    // let mut stmt = conn.prepare("SELECT mid FROM notes")?;
    // let notes = stmt.query_map([], |row| {
    //     let str: String = row.get_unwrap(0);
    //     //panic!("{}", str);
    //     Ok(str)
    // })?;
    // notes.for_each(|a| {
    //    let my_str = a.unwrap();
    //     println!("{}", my_str);
    // });
    // x.for_each(|a| {
    //     //println!("{:?}", a);
    //     let my_value: Value = serde_json::from_str(a.unwrap().as_str()).unwrap();
    //     //let v = my_value.(0).unwrap();
    //     //println!("{:?}", v);
    // });
    Ok(())
}