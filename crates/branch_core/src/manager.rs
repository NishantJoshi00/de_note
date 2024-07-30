use std::collections::HashMap;

use crate::types::{Note, NoteId};

#[derive(Default)]
pub struct NotesManager {
    notes: HashMap<NoteId, Note>,
}

mod add;
mod delete;
mod change;
