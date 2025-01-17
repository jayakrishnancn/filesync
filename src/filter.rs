use crate::{
    compare::State,
    enums::{Actions, PathType},
};

pub fn filter_ds_store(item: &State) -> bool {
    !item.path.ends_with(".DS_Store")
}

pub fn filter_skip_and_unknown_action(item: &State) -> bool {
    match item.action {
        Actions::Unknown | Actions::Skip => false,
        _ => true,
    }
}

// pub fn skip_directory(item: &State) -> bool {
//     match item.path_type {
//         PathType::FILE => true,
//         _ => false,
//     }
// }

pub fn filter(items: Vec<State>) -> Vec<State> {
    items
        .into_iter()
        .filter(filter_ds_store)
        // .filter(skip_directory)
        .filter(filter_skip_and_unknown_action)
        .collect()
}
