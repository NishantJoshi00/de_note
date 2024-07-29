#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AddError {
    #[error("Note already exists")]
    NoteAlreadyExists,
    #[error("Note does not exist")]
    NoteDoesNotExist,

    #[error("Link already exists")]
    LinkAlreadyExists,
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeError {}

#[derive(thiserror::Error, Debug)]
pub enum DeleteError {}

#[derive(thiserror::Error, Debug)]
pub enum ReadError {}
