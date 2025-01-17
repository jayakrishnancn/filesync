pub enum Strategy {
    TwoWay,
    MIRROR,
    UPDATE,
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
