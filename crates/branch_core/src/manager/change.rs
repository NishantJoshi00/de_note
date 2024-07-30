use error_stack::ResultExt;

use crate::errors;
use crate::manager_impl::{ChangeBranch, ChangeLink, ChangeNote, DeleteBranch};

impl ChangeNote for super::NotesManager {
    fn change_note_title(
        &mut self,
        note: crate::types::NoteId,
        title: String,
    ) -> error_stack::Result<(), crate::errors::ChangeError> {
        let note = self
            .notes
            .get_mut(&note)
            .ok_or(crate::errors::ChangeError::NoteDoesNotExist)?;

        note.title = title;

        Ok(())
    }

    fn change_note_subtitle(
        &mut self,
        note: crate::types::NoteId,
        subtitle: String,
    ) -> error_stack::Result<(), crate::errors::ChangeError> {
        let note = self
            .notes
            .get_mut(&note)
            .ok_or(crate::errors::ChangeError::NoteDoesNotExist)?;

        note.subtitle = Some(subtitle);

        Ok(())
    }

    fn change_note_body(
        &mut self,
        note: crate::types::NoteId,
        body: String,
    ) -> error_stack::Result<(), crate::errors::ChangeError> {
        let note = self
            .notes
            .get_mut(&note)
            .ok_or(crate::errors::ChangeError::NoteDoesNotExist)?;

        note.body = body;

        Ok(())
    }

    fn mark_note(
        &mut self,
        note: crate::types::NoteId,
    ) -> error_stack::Result<(), crate::errors::ChangeError> {
        let note = self
            .notes
            .get_mut(&note)
            .ok_or(crate::errors::ChangeError::NoteDoesNotExist)?;

        note.marked = true;

        Ok(())
    }

    fn unmark_note(
        &mut self,
        note: crate::types::NoteId,
    ) -> error_stack::Result<(), crate::errors::ChangeError> {
        let note = self
            .notes
            .get_mut(&note)
            .ok_or(crate::errors::ChangeError::NoteDoesNotExist)?;

        note.marked = false;

        Ok(())
    }

    fn reconsile_nodes(&mut self) -> error_stack::Result<(), crate::errors::ChangeError> {
        todo!()
    }
}

impl ChangeLink for super::NotesManager {
    fn change_link_reason(
        &mut self,
        from_note: crate::types::NoteId,
        to_note: crate::types::NoteId,
        reason: String,
    ) -> error_stack::Result<(), crate::errors::ChangeError> {
        let from_note_ = self
            .notes
            .get_mut(&from_note)
            .ok_or(crate::errors::ChangeError::NoteDoesNotExist)?;

        let link = from_note_
            .forwardlinks
            .iter_mut()
            .find(|flink| match flink {
                crate::types::FLink::Link(link) => link.id == to_note,
                crate::types::FLink::Branch(_) => false,
            })
            .ok_or(crate::errors::ChangeError::LinkDoesNotExist)?;

        if let crate::types::FLink::Link(link) = link {
            link.reason = reason;
        }

        Ok(())
    }
}

impl ChangeBranch for super::NotesManager {
    fn change_branch_condition(
        &mut self,
        note: crate::types::NoteId,
        branch: crate::types::BranchId,
        condition: String,
    ) -> error_stack::Result<(), crate::errors::ChangeError> {
        let note = self
            .notes
            .get_mut(&note)
            .ok_or(crate::errors::ChangeError::NoteDoesNotExist)?;

        let branch = note
            .forwardlinks
            .iter_mut()
            .find(|flink| match flink {
                crate::types::FLink::Branch(branch_) => branch_.get_id() == branch,
                crate::types::FLink::Link(_) => false,
            })
            .ok_or(crate::errors::ChangeError::BranchDoesNotExist)?;

        if let crate::types::FLink::Branch(branch_) = branch {
            branch_.condition = condition;
        }

        Ok(())
    }

    fn change_branch_reason(
        &mut self,
        note: crate::types::NoteId,
        branch: crate::types::BranchId,
        link_note: crate::types::NoteId,
        reason: String,
    ) -> error_stack::Result<(), crate::errors::ChangeError> {
        let note = self
            .notes
            .get_mut(&note)
            .ok_or(crate::errors::ChangeError::NoteDoesNotExist)?;

        let branch = note
            .forwardlinks
            .iter_mut()
            .find(|flink| match flink {
                crate::types::FLink::Branch(branch_) => branch_.get_id() == branch,
                crate::types::FLink::Link(_) => false,
            })
            .ok_or(crate::errors::ChangeError::BranchDoesNotExist)?;

        if let crate::types::FLink::Branch(branch_) = branch {
            let link = branch_
                .branches
                .iter_mut()
                .find(|link| link.id == link_note)
                .ok_or(crate::errors::ChangeError::LinkDoesNotExist)?;

            link.reason = reason;
        }

        Ok(())
    }

    fn collapse_branch(
        &mut self,
        note: crate::types::NoteId,
        branch: crate::types::BranchId,
        link_note: crate::types::NoteId,
    ) -> error_stack::Result<(), crate::errors::ChangeError> {
        let note_ = self
            .notes
            .get_mut(&note)
            .ok_or(crate::errors::ChangeError::NoteDoesNotExist)?;

        let branch_ = note_
            .forwardlinks
            .iter()
            .find(|flink| match flink {
                crate::types::FLink::Branch(branch_) => branch_.get_id() == branch,
                crate::types::FLink::Link(_) => false,
            })
            .cloned()
            .ok_or(crate::errors::ChangeError::BranchDoesNotExist)?;

        note_.forwardlinks.retain(|flink| match flink {
            crate::types::FLink::Branch(branch_) => branch_.get_id() != branch,
            crate::types::FLink::Link(_) => true,
        });

        if let crate::types::FLink::Branch(branch_) = branch_ {
            for b in branch_.branches.clone() {
                if b.id == link_note {
                    note_.forwardlinks.push(crate::types::FLink::Link(b));
                    break;
                }
            }

            for b in branch_.branches.clone() {
                if b.id != link_note {
                    self.delete_branch_link(note.clone(), branch_.get_id(), b.id.clone())
                        .change_context(errors::ChangeError::BranchDoesNotExist)?;
                }
            }
        }

        Ok(())
    }
}
