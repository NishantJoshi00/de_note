use nanoid::nanoid;

///
/// [`NoteId`] is a unique identifier for a note.
/// It is a string generated using nanoid.
///
/// ```rust
/// use branch_core::types::NoteId;
///
/// let id1 = NoteId::new();
/// let id2 = NoteId::new();
///
/// assert_ne!(id1, id2);
/// ```
///
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct NoteId(String);

///
/// [`NoteId`] is a unique identifier for a note.
/// It is a string generated using nanoid.
///
/// ```rust
///
/// use branch_core::types::BranchId;
///
/// let id1 = BranchId::new();
/// let id2 = BranchId::new();
///
/// assert_ne!(id1, id2);
/// ```
///
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct BranchId(String);

///
/// [`Link`] represents a link between two notes.
/// It has a destination note id and an optional reason.
///
#[cfg_attr(test, derive(Clone))]
pub struct Link {
    pub id: NoteId,
    pub reason: String,
}

///
/// [`Branch`] represents a branch in a note. It has a condition and a list of branches.
/// Each branch can be a link or another branch. This allows for a tree-like structure in a note.
///
#[cfg_attr(test, derive(Clone))]
pub struct Branch {
    id: BranchId,
    pub condition: String,
    pub branches: Vec<Link>,
}

///
/// [`FLink`] represents a forward link in a note.
/// It can be a link or a branch. This allows for a tree-like structure in a note.
///
///
#[cfg_attr(test, derive(Clone))]
pub enum FLink {
    Link(Link),
    Branch(Branch),
}

///
/// [`Note`] represents a note in the note-taking app.
/// It has a unique id, a title, a subtitle, a body, a list of backlinks, and a list of forward links. backlinks are automatically generated when a note links to another note. Forward links are manually added by the user.
///
#[cfg_attr(test, derive(Clone))]
pub struct Note {
    id: NoteId,
    pub marked: bool,
    pub title: String,
    pub subtitle: Option<String>,
    pub body: String,
    backlinks: Vec<NoteId>,
    pub forwardlinks: Vec<FLink>,
    pub timestamp: time::PrimitiveDateTime,
}

impl NoteId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        NoteId(nanoid!())
    }

    #[cfg(test)]
    pub fn new_test(id: String) -> Self {
        NoteId(id)
    }
}

impl BranchId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        BranchId(nanoid!())
    }

    #[cfg(test)]
    pub fn new_test(id: String) -> Self {
        BranchId(id)
    }
}

impl Note {
    pub fn new(title: String, subtitle: Option<String>, body: String) -> Self {
        let now_odt = time::OffsetDateTime::now_utc();
        Note {
            id: NoteId::new(),
            title,
            subtitle,
            body,
            backlinks: Vec::new(),
            forwardlinks: Vec::new(),
            timestamp: time::PrimitiveDateTime::new(now_odt.date(), now_odt.time()),
            marked: false,
        }
    }
    pub fn get_id(&self) -> NoteId {
        self.id.clone()
    }

    #[cfg(test)]
    pub fn set_id(self, id: String) -> Self {
        Note {
            id: NoteId::new_test(id),
            ..self
        }
    }
}
