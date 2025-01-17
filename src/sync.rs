use std::sync::Arc;

use crate::{
    compare::State,
    enums::{Actions, Strategy},
    sync_state::SyncState,
};

pub fn sync_files(items: Vec<State>, state: &Arc<SyncState>) {
    let mut done_items: Vec<State> = vec![];
    for item in items {
        match item.action {
            Actions::CREATE => {
                //TODO: copy directory/file
                done_items.push(item.clone());
            }
            _ => {}
        }
    }
    state.mark_multiple(&done_items, "completed");
}
