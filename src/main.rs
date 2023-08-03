use clap::Parser;
use cli::{Cli, Commands};
use db::{add_note, delete_note, get_note, list_notes};
use notes::Note;
use rocksdb::DB;

mod cli;
mod db;
mod notes;

fn main() {
    // path of the db
    let path = "./database";
    // open db with default options
    let db = DB::open_default(path).unwrap();

    // parse the Cli from the command line
    let cli = Cli::parse();

    match cli.command {
        Commands::AddNote { id, content } => add_note_to_db(&db, id, content).unwrap(),
        Commands::DeleteNote { id } => delete_note_from_db(&db, id).unwrap(),
        Commands::GetNote { id } => get_note_from_db(&db, id).unwrap(),
        Commands::ListNotes {} => list_notes_from_db(&db).unwrap(),
    };

    /*
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
    */
}

fn add_note_to_db(db: &DB, id: u64, content: String) -> Result<(), rocksdb::Error> {
    let note = Note::new(id, content);
    add_note(db, &note)
}

fn delete_note_from_db(db: &DB, id: u64) -> Result<(), rocksdb::Error> {
    delete_note(db, id)
}

fn get_note_from_db(db: &DB, id: u64) -> Result<(), rocksdb::Error> {
    match get_note(db, id)? {
        Some(content) => {
            println!("{}", String::from_utf8(content).unwrap());
            Ok(())
        }
        None => {
            println!("note {} does not exist in the db", id);
            Ok(())
        }
    }
}

fn list_notes_from_db(db: &DB) -> Result<(), rocksdb::Error> {
    match list_notes(db) {
        Ok(notes) => {
            println!("printing all values in the db...");
            notes.iter().for_each(|value| {
                println!("{}", String::from_utf8(value.to_owned()).unwrap());
            });
            Ok(())
        }
        Err(e) => {
            println!("operational problem encountered: {}", e);
            Err(e)
        }
    }
}
