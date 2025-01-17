use std::fmt;

pub enum Strategy {
    // TwoWay,
    Mirror,
    Update,
}

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Default)]
pub enum Actions {
    Create,
    // UPDATE, // or replace
    Delete,
    Skip,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default)]
pub enum PathType {
    Link,
    Dir,
    File,
    #[default]
    Unknown,
}
