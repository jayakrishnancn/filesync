use std::fmt;

pub enum Strategy {
    TwoWay,
    MIRROR,
    UPDATE,
}

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Default)]
pub enum Actions {
    CREATE,
    UPDATE, // or replace
    DELETE,
    SKIP,
    #[default]
    UNKNOWN,
}

#[derive(Clone, Debug, Default)]
pub enum PathType {
    LINK,
    DIR,
    FILE,
    #[default]
    UNKNOWN,
}
