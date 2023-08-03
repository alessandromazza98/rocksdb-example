use crate::notes::Note;
use rocksdb::DB;

/// Add a note to the database.
pub fn add_note(db: &DB, note: &Note) -> Result<(), rocksdb::Error> {
    db.put(note.get_id().to_le_bytes(), note.get_content().as_bytes())
}

/// Get a note from the database.
pub fn get_node(db: &DB, id: u64) -> Result<Option<Vec<u8>>, rocksdb::Error> {
    db.get(id.to_le_bytes())
}

/// Delete a note from the database.
pub fn delete_node(db: &DB, id: u64) -> Result<(), rocksdb::Error> {
    db.delete(id.to_le_bytes())
}

/// List all notes from the database.
pub fn list_notes(db: &DB) -> Result<Vec<Vec<u8>>, rocksdb::Error> {
    let mut notes: Vec<Vec<u8>> = vec![];

    let iter = db.iterator(rocksdb::IteratorMode::Start);
    for item in iter {
        match item {
            Ok((_, value)) => notes.push(value.into()),
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(notes)
}
