use error_stack::ensure;

use crate::errors::AddError;
use crate::manager_impl::{AddBranch, AddLink, AddNote};
use crate::types;

impl AddNote for super::NotesManager {
    fn add_note(
        &mut self,
        note: crate::types::Note,
    ) -> error_stack::Result<crate::types::NoteId, AddError> {
        let note_id = note.get_id();

        ensure!(
            !self.notes.contains_key(&note_id),
            AddError::NoteAlreadyExists
        );

        self.notes.insert(note_id.clone(), note);

        Ok(note_id)
    }
}

impl AddLink for super::NotesManager {
    fn add_link(
        &mut self,
        from_note: crate::types::NoteId,
        to_note: crate::types::NoteId,
        reason: String,
    ) -> error_stack::Result<(), AddError> {
        ensure!(
            self.notes.contains_key(&to_note),
            AddError::NoteDoesNotExist
        );

        let from_note_ = self
            .notes
            .get_mut(&from_note)
            .ok_or(AddError::NoteDoesNotExist)?;

        let duplicate_clause = from_note_.forwardlinks.iter().any(|flink| match flink {
            crate::types::FLink::Link(link) => link.id == to_note,
            crate::types::FLink::Branch(_) => false,
        });

        ensure!(!duplicate_clause, AddError::LinkAlreadyExists);

        from_note_
            .forwardlinks
            .push(types::FLink::Link(types::Link {
                id: to_note.clone(),
                reason,
            }));

        let to_note = self
            .notes
            .get_mut(&to_note)
            .ok_or(AddError::NoteDoesNotExist)?;

        to_note.add_backlink(from_note);

        Ok(())
    }
}

impl AddBranch for super::NotesManager {
    fn create_branching(
        &mut self,
        note: types::NoteId,
        condition: String,
    ) -> error_stack::Result<types::BranchId, AddError> {
        let note = self
            .notes
            .get_mut(&note)
            .ok_or(AddError::NoteDoesNotExist)?;

        let branch = types::Branch::new(condition);

        let branch_id = branch.get_id();

        note.forwardlinks.push(types::FLink::Branch(branch));

        Ok(branch_id)
    }

    fn add_branch(
        &mut self,
        note: types::NoteId,
        on_branch: types::BranchId,
        link_note: types::NoteId,
        reason: String,
    ) -> error_stack::Result<(), AddError> {
        ensure!(
            self.notes.contains_key(&link_note),
            AddError::NoteDoesNotExist
        );

        let note_ = self
            .notes
            .get_mut(&note)
            .ok_or(AddError::NoteDoesNotExist)?;

        let branch = note_
            .forwardlinks
            .iter_mut()
            .find_map(|flink| match flink {
                types::FLink::Branch(branch) if branch.get_id() == on_branch => Some(branch),
                _ => None,
            })
            .ok_or(AddError::BranchDoesNotExist)?;

        let duplicate_clause = branch.branches.iter().any(|link| link.id == link_note);

        ensure!(!duplicate_clause, AddError::BranchAlreadyExists);

        branch.branches.push(types::Link {
            id: link_note.clone(),
            reason,
        });

        self.notes
            .get_mut(&link_note)
            .ok_or(AddError::NoteDoesNotExist)?
            .add_backlink(note);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_verify() {
        let mut manager = super::super::NotesManager::default();
        let note = crate::types::Note::new(
            "title".to_string(),
            Some("subtitle".to_string()),
            "body".to_string(),
        );

        let note = note.set_id("00000".to_string());

        let note_id = manager.add_note(note).unwrap();

        assert_eq!(manager.notes.len(), 1);
        assert_eq!(note_id, crate::types::NoteId::new_test("00000".to_string()));
    }

    #[test]
    fn test_duplicate_note() {
        let mut manager = super::super::NotesManager::default();
        let note = crate::types::Note::new(
            "title".to_string(),
            Some("subtitle".to_string()),
            "body".to_string(),
        );

        let note = note.set_id("00000".to_string());

        let note_id = manager.add_note(note.clone());

        assert!(note_id.is_ok());

        let note_id = manager.add_note(note);

        assert!(note_id.is_err());
        assert_eq!(manager.notes.len(), 1);
        assert_eq!(
            note_id.unwrap_err().current_context(),
            &AddError::NoteAlreadyExists
        );
    }
}
