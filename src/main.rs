use clap::Parser;
use cli::{Cli, Commands};
use db::{add_note, add_notes, delete_note, get_note, list_notes};
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
        Commands::AddNotes { note } => add_notes_to_db(&db, note).unwrap(),
        Commands::DeleteNote { id } => delete_note_from_db(&db, id).unwrap(),
        Commands::GetNote { id } => get_note_from_db(&db, id).unwrap(),
        Commands::ListNotes {} => list_notes_from_db(&db).unwrap(),
    };
}

fn add_note_to_db(db: &DB, id: u64, content: String) -> Result<(), rocksdb::Error> {
    let note = Note::new(id, content);
    add_note(db, &note)
}

fn add_notes_to_db(db: &DB, notes: Vec<(u64, String)>) -> Result<(), rocksdb::Error> {
    let notes: Vec<Note> = notes
        .into_iter()
        .map(|(id, content)| Note::new(id, content))
        .collect();
    add_notes(db, &notes)
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

#[cfg(test)]
mod test {
    use clap::Parser;

    use crate::cli::{Cli, Commands};

    #[test]
    fn parse_add_note() {
        let cli = Cli::parse_from([
            "rocksdb-example",
            "add-note",
            "--id",
            "0",
            "--content",
            "Ale nota 1",
        ]);
        assert!(
            matches!(cli.command, Commands::AddNote { id: 0, content: ref c } if c == "Ale nota 1")
        );
    }

    #[test]
    fn parse_add_notes() {
        let cli = Cli::parse_from([
            "rocksdb-example",
            "add-notes",
            "--note",
            "0 nota0",
            "--note",
            "1 nota1",
        ]);
        assert!(
            matches!(cli.command, Commands::AddNotes { note } if note == vec![(0, "nota0".to_string()), (1, "nota1".to_string())])
        );
    }

    #[test]
    fn parse_get_note() {
        let cli = Cli::parse_from(["rocksdb-example", "get-note", "--id", "0"]);
        assert!(matches!(cli.command, Commands::GetNote { id: 0 }));
    }

    #[test]
    fn parse_delete_note() {
        let cli = Cli::parse_from(["rocksdb-example", "delete-note", "--id", "0"]);
        assert!(matches!(cli.command, Commands::DeleteNote { id: 0 }));
    }

    #[test]
    fn parse_list_notes() {
        let cli = Cli::parse_from(["rocksdb-example", "list-notes"]);
        assert!(matches!(cli.command, Commands::ListNotes {}));
    }
}
