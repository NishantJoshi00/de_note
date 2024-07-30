#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AddError {
    #[error("Note already exists")]
    NoteAlreadyExists,
    #[error("Note does not exist")]
    NoteDoesNotExist,

    #[error("Link already exists")]
    LinkAlreadyExists,

    #[error("Branch already exists")]
    BranchAlreadyExists,

    #[error("Branch does not exist")]
    BranchDoesNotExist,
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeError {
    #[error("Note does not exist")]
    NoteDoesNotExist,

    #[error("Link does not exist")]
    LinkDoesNotExist,

    #[error("Branch does not exist")]
    BranchDoesNotExist,
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteError {
    #[error("Note does not exist")]
    NoteDoesNotExist,
    #[error("Link does not exist")]
    LinkDoesNotExist,

    #[error("Branch does not exist")]
    BranchDoesNotExist,
    #[error("Branch is not empty")]
    BranchNotEmpty,
}

#[derive(thiserror::Error, Debug)]
pub enum ReadError {}
