/*  Tener dos listas, una de to-do y otras done
    En el menú elegir primero qué lista manejar, y luego ver, editar o borrar item
    También agregar una opción para agregar directamente, y se irá directo a to-do
    Guardar listas en archivos y cargarlas al iniciar
*/
mod listing;
mod m_logger;
mod list_manager;
mod dialogue_manager;
use crate::dialogue_manager::Dialogue_manager;
use crate::list_manager::{ListManager, PATH_LISTS};
use log::{error, info, warn, debug};
use log4rs;

const DEFAULT_LIST_FILE: &'static str = "File_list_default.txt";

fn main() {
    let selections = &[
        "List All items",
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
        let selection = dialogue_manager.get_selection_default(selections.to_vec());
        match selection {
            0 => println!("{}", list_manager.pretty_printing_all()),
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
