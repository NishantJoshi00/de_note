use crate::errors;
use crate::manager_impl::{DeleteBranch, DeleteLink, DeleteNote};

impl DeleteNote for super::NotesManager {
    fn delete_note(
        &mut self,
        note: crate::types::NoteId,
    ) -> error_stack::Result<(), errors::DeleteError> {
        let note_ = self
            .notes
            .get(&note)
            .ok_or(errors::DeleteError::NoteDoesNotExist)?;

        if note_.backlinks.is_empty() {
            for flink in note_.forwardlinks.clone() {
                match flink {
                    crate::types::FLink::Link(link) => {
                        self.delete_link(note.clone(), link.id.clone())?;
                    }
                    crate::types::FLink::Branch(branch) => {
                        self.delete_branch(note.clone(), branch.get_id())?;
                    }
                }
            }
            self.notes.remove(&note);
        } else {
            for backlink in note_.backlinks.clone() {
                self.delete_link(backlink.clone(), note.clone())?;
            }
        }

        Ok(())
    }
}

impl DeleteLink for super::NotesManager {
    fn delete_link(
        &mut self,
        from_note: crate::types::NoteId,
        to_note: crate::types::NoteId,
    ) -> error_stack::Result<(), errors::DeleteError> {
        let from_note_ = self
            .notes
            .get_mut(&from_note)
            .ok_or(errors::DeleteError::NoteDoesNotExist)?;

        from_note_.forwardlinks.retain(|flink| match flink {
            crate::types::FLink::Link(link) => link.id != to_note,
            crate::types::FLink::Branch(_) => true,
        });

        let links = self
            .notes
            .get_mut(&to_note)
            .ok_or(errors::DeleteError::NoteDoesNotExist)?
            .delete_backlink(&from_note);

        if links == 0 {
            self.delete_note(to_note)?;
        }

        Ok(())
    }
}

impl DeleteBranch for super::NotesManager {
    fn delete_branch(
        &mut self,
        note: crate::types::NoteId,
        branch: crate::types::BranchId,
    ) -> error_stack::Result<(), errors::DeleteError> {
        let note_ = self
            .notes
            .get_mut(&note)
            .ok_or(errors::DeleteError::NoteDoesNotExist)?;

        let branch_ = note_
            .forwardlinks
            .iter()
            .find_map(|flink| match flink {
                crate::types::FLink::Branch(branch_) => {
                    if branch_.get_id() == branch {
                        Some(branch_)
                    } else {
                        None
                    }
                }
                crate::types::FLink::Link(_) => None,
            })
            .ok_or(errors::DeleteError::BranchDoesNotExist)?;

        if branch_.branches.is_empty() {
            note_.forwardlinks.retain(|flink| match flink {
                crate::types::FLink::Branch(branch_) => branch_.get_id() != branch,
                crate::types::FLink::Link(_) => true,
            });
        } else {
            for link in branch_.branches.clone() {
                self.delete_branch_link(note.clone(), branch.clone(), link.id.clone())?;
            }
        }

        Ok(())
    }

    fn delete_branch_link(
        &mut self,
        note: crate::types::NoteId,
        branch: crate::types::BranchId,
        link_note: crate::types::NoteId,
    ) -> error_stack::Result<(), errors::DeleteError> {
        let note_ = self
            .notes
            .get_mut(&note)
            .ok_or(errors::DeleteError::NoteDoesNotExist)?;

        let branch_ = note_
            .forwardlinks
            .iter_mut()
            .find_map(|flink| match flink {
                crate::types::FLink::Branch(branch_) => {
                    if branch_.get_id() == branch {
                        Some(branch_)
                    } else {
                        None
                    }
                }
                crate::types::FLink::Link(_) => None,
            })
            .ok_or(errors::DeleteError::BranchDoesNotExist)?;

        branch_.branches.retain(|link| link.id != link_note);

        let link_note_ = self
            .notes
            .get_mut(&link_note)
            .ok_or(errors::DeleteError::NoteDoesNotExist)?;

        link_note_.delete_backlink(&note);

        Ok(())
    }
}
