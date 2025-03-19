
mod listing;
mod m_logger;
mod list_manager;
mod dialogue_manager;
use crate::dialogue_manager::Dialogue_manager;
use crate::list_manager::ListManager;
use log4rs;

// TODO: Use multiple selection on modifications of items
fn main() {
    const SELECTIONS: &'static [&str] = &[
        "List TODO items",
        "List completed items",
        "Add item",
        "Modify Item",
        "Remove Item",
        "Change list",
        "Close"
    ];

    m_logger::init_logs_default();
    let dialogue_manager = Dialogue_manager::default();
    let mut list_manager = ListManager::new(&dialogue_manager);
    let mut exit = false;
    while !exit {
        let selection = dialogue_manager.get_selection_default(SELECTIONS.to_vec());
        match selection {
            0 => println!("{}", list_manager.pretty_printing_todo()),
            1 => println!("{}", list_manager.pretty_printing_completed()),
            2 => list_manager.add_item(),
            3 => list_manager.modify_item(),
            4 => list_manager.remove_item(),
            5 => list_manager.change_list(),
            6 => exit = true,
            _ => println!("Unknown choice")
        }
    }
    match list_manager.write_json() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
