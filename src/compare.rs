use rayon::prelude::*;
use std::{collections::HashMap, path::Path};

use walkdir::WalkDir;

use crate::enums::{Actions, PathType, Strategy};

#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    pub path: String,
    pub path_type: PathType,
    pub action: Actions,
    pub hash: String,
}

// compare and return the diffs entries
pub fn compare(source: &Path, destination: &Path, strategy: Strategy) -> Vec<State> {
    let source_entries = WalkDir::new(source);
    let destination_entries = WalkDir::new(destination);

    let sources: HashMap<String, State> = get_states_map(source_entries);
    let destinations: HashMap<String, State> = get_states_map(destination_entries);

    let mut result: Vec<State> = sources
        .values()
        .cloned()
        .collect::<Vec<State>>()
        .par_iter()
        .map(|source_entry| {
            // TODO: find dest_entry from destinations
            // TODO: should we use hash instad of path to compare?
            if let Some(dest_entry) = destinations.get(&source_entry.path) {
                // TODO: can we remove use of clone for optimization?
                let mut result = dest_entry.clone();
                result.action = Actions::Skip;
                return result;
            } else {
                let mut result = source_entry.clone();
                result.action = Actions::Create;
                return result;
            }
        })
        .collect();

    match strategy {
        Strategy::Mirror => {
            destinations
                .values()
                .cloned()
                // TODO: parallel processing?
                // .collect::<Vec<State>>()
                // .par_iter()
                .for_each(|destination_entry| {
                    if sources.get(&destination_entry.path).is_none() {
                        let mut item = destination_entry.clone();
                        item.action = Actions::Delete;
                        result.push(item);
                    }
                });
        }
        _ => {}
    }

    result
}

fn get_states_map(dir_entries: WalkDir) -> HashMap<String, State> {
    let mut map: HashMap<String, State> = HashMap::new();
    dir_entries.into_iter().for_each(|entry| {
        let path = entry.expect("Error Reading entries").path().to_owned();
        let mut path_type: PathType = PathType::Unknown;
        if path.is_dir() {
            path_type = PathType::Dir;
        } else if path.is_file() {
            path_type = PathType::File;
        } else if path.is_symlink() {
            path_type = PathType::Link;
        }
        let path_string = path.to_string_lossy().to_string();
        map.insert(
            path_string.clone(),
            State {
                hash: get_hash(path.as_path()),
                path_type,
                path: path_string,
                action: Actions::Skip,
            },
        );
    });
    map
}

//TODO: error handling, remove all expect and unwrap
fn get_hash(path: &Path) -> String {
    let metadata = path.metadata().expect("metadata missing");
    let time = metadata
        .created()
        .expect("created field missing")
        .elapsed()
        .expect("no elapsed")
        .as_millis();
    return format!("{}-{}-{}", path.to_string_lossy(), time, metadata.len());
}
