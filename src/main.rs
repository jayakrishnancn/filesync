// features
// ------------
// 1. compare
// 2. sync
// 3. delete strategy (to trash, permanat delete)
// 4. save progress
// 5. error handling
// 6. UI
// 7. validate using hash
// 8. batch process
// 9. schedule

mod compare;
mod enums;
mod filter;
mod sync;
mod sync_state;
mod validate;
use std::{env, sync::Arc};

use enums::Strategy;
use sync_state::SyncState;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (source, destination, db_file_name) = validate::validate(args);
    let strategy = Strategy::Update;
    let items = compare::compare(source.as_path(), destination.as_path(), strategy);
    let items = filter::filter(items);
    let state = Arc::new(SyncState::new(&db_file_name));
    state.mark_multiple(&items, "compared");
    sync::sync_files(source.as_path(), destination.as_path(), items, &state);
    if state.check_if_all_are_comleted() {
        println!("Successfully synced ♥");
    } else {
        println!("Some files are missing in destination");
    }
}
