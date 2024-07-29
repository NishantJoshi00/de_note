use crate::errors::{AddError, ChangeError, ReadError};
use crate::types::{BranchId, FLink, Link, Note, NoteId};
use error_stack::Result;

///
/// [`AddNote`] is a trait that defines the method to add a note.
///
pub trait Add: AddNote + AddLink + AddBranch {}

///
/// [`Change`] is a trait that combines the [`ChangeNote`], [`ChangeLink`], and [`ChangeBranch`] traits.
///
pub trait Change: ChangeNote + ChangeLink + ChangeBranch {}

///
/// [`Delete`] is a trait that combines the [`DeleteNote`], [`DeleteLink`], and [`DeleteBranch`] traits.
///
///
pub trait Delete: DeleteNote + DeleteLink + DeleteBranch {}

pub trait Read: ReadNote + ReadLink + ReadBranch {}

impl<T> Add for T where T: AddNote + AddLink + AddBranch {}
impl<T> Change for T where T: ChangeNote + ChangeLink + ChangeBranch {}
impl<T> Delete for T where T: DeleteNote + DeleteLink + DeleteBranch {}

pub trait AddNote {
    fn add_note(&mut self, note: Note) -> Result<NoteId, AddError>;
}

pub trait AddLink {
    fn add_link(
        &mut self,
        from_note: NoteId,
        to_note: NoteId,
        reason: String,
    ) -> Result<(), AddError>;
}

pub trait AddBranch {
    fn create_branching(&mut self, note: NoteId, condition: String) -> Result<BranchId, AddError>;
    fn add_branch(
        &mut self,
        note: NoteId,
        on_branch: BranchId,
        link_note: NoteId,
        reason: String,
    ) -> Result<(), AddError>;
}

pub trait ChangeNote {
    fn change_note_title(&mut self, note: NoteId, title: String) -> Result<(), ChangeError>;
    fn change_note_subtitle(&mut self, note: NoteId, subtitle: String) -> Result<(), ChangeError>;
    fn change_note_body(&mut self, note: NoteId, body: String) -> Result<(), ChangeError>;
    fn mark_note(&mut self, note: NoteId) -> Result<(), ChangeError>;
    fn unmark_note(&mut self, note: NoteId) -> Result<(), ChangeError>;

    ///
    /// [`reconsile_nodes`] is a method that reconsiles the nodes in the note-taking app. This will
    /// resolve links and branches that are not valid, mark notes whose forward links are all marked, And perform vacuuming.
    ///
    fn reconsile_nodes(&mut self) -> Result<(), ChangeError>;
}

pub trait ChangeLink {
    fn change_link_reason(
        &mut self,
        from_note: NoteId,
        to_note: NoteId,
        reason: String,
    ) -> Result<(), ChangeError>;
}

pub trait ChangeBranch {
    fn change_branch_condition(
        &mut self,
        note: NoteId,
        branch: BranchId,
        condition: String,
    ) -> Result<(), ChangeError>;
    fn change_branch_reason(
        &mut self,
        note: NoteId,
        branch: BranchId,
        link_note: NoteId,
        reason: String,
    ) -> Result<(), ChangeError>;

    fn collapse_branch(&mut self, branch: BranchId, link_note: NoteId) -> Result<(), ChangeError>;
}

pub trait DeleteNote {
    fn delete_note(&mut self, note: NoteId) -> Result<(), AddError>;
}

pub trait DeleteLink {
    fn delete_link(&mut self, from_note: NoteId, to_note: NoteId) -> Result<(), AddError>;
}

pub trait DeleteBranch {
    fn delete_branch(&mut self, note: NoteId, branch: BranchId) -> Result<(), AddError>;
    fn delete_branch_link(&mut self, branch: BranchId, link_note: NoteId) -> Result<(), AddError>;
}

pub trait ReadNote {
    fn read_note(&self, note: NoteId) -> Result<&Note, ReadError>;
    fn list_root_notes(&self) -> Result<Vec<&NoteId>, ReadError>;
}

pub trait ReadLink {
    fn list_forwardlinks(&self, note: NoteId) -> Result<Vec<&FLink>, ReadError>;
    fn list_unmarked_forwardlinks(&self, note: NoteId) -> Result<Vec<&FLink>, ReadError>;
    fn list_pure_links(&self, note: NoteId) -> Result<Vec<&Link>, ReadError>;
    fn list_backlinks(&self, note: NoteId) -> Result<Vec<&NoteId>, ReadError>;
}

pub trait ReadBranch {
    fn list_branches(&self, note: NoteId) -> Result<Vec<&BranchId>, ReadError>;
    fn list_branch_links(&self, branch: BranchId) -> Result<Vec<&Link>, ReadError>;
}
