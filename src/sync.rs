use std::{fs, path::Path, sync::Arc};

use crate::{compare::State, enums::Actions, sync_state::SyncState};

pub fn sync_files(source: &Path, destination: &Path, items: Vec<State>, state: &Arc<SyncState>) {
    let mut done_items: Vec<State> = vec![];
    for item in items {
        let path = Path::new(&item.path)
            .canonicalize()
            .expect("Failed to get absolute path");
        let relative_path = path.strip_prefix(source).unwrap();
        let to = destination.join(relative_path);
        if path.is_dir() {
            fs::create_dir_all(&to).expect(&format!(
                "Failed to create directory {:?}",
                to.to_string_lossy()
            ));
            continue;
        }
        match item.action {
            Actions::CREATE => {
                println!("copying {:?}", path);
                fs::copy(path, to).expect("Can not copy the file/dir");
                done_items.push(item.clone());
            }
            _ => {}
        }
    }
    state.mark_multiple(&done_items, "completed");
}
