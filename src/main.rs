mod cli;
mod db;
mod notes;

fn main() {
    use crate::notes::Note;
    use db::{add_note, delete_node, get_node, list_notes};
    use rocksdb::DB;

    // path of the db
    let path = "./database";

    // open db with default options
    let db = DB::open_default(path).unwrap();

    // create a note
    let note_1 = Note::new(0, "Questa è la storia numero 0".to_string());

    // save the note in the db, use id as key
    add_note(&db, &note_1).unwrap();

    // retrieve the value
    match get_node(&db, 0) {
        Ok(Some(value)) => println!("retrieved value: {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    // create a note
    let note_2 = Note::new(1, "Questa è la nota numero 1".to_string());

    // save the note in the db, use id as key
    add_note(&db, &note_2).unwrap();

    // retrieve the value
    match get_node(&db, 1) {
        Ok(Some(value)) => println!("retrieved value: {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    // retrieve all values
    println!("printing all values in the database...");
    match list_notes(&db) {
        Ok(notes) => {
            notes.iter().for_each(|value| {
                println!(
                    "retrieved value: {}",
                    String::from_utf8(value.to_owned()).unwrap()
                );
            });
        }
        Err(e) => println!("operational problem encountered: {}", e),
    }

    // delete a note
    match delete_node(&db, 0) {
        Ok(()) => println!("note deleted"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    // retrieve all values
    println!("printing all values in the database...");
    match list_notes(&db) {
        Ok(notes) => {
            notes.iter().for_each(|value| {
                println!(
                    "retrieved value: {}",
                    String::from_utf8(value.to_owned()).unwrap()
                );
            });
        }
        Err(e) => println!("operational problem encountered: {}", e),
    }
}
