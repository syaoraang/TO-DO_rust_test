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
use crate::list_manager::{ListManager};
//use crate::m_logger::*;
use dialoguer::{theme::ColorfulTheme, Input};
use std::string::String;
use std::fs;
use log::{error, info, warn, debug};
use log4rs;
use crate::listing::{Listing, ListItem};

const FILE_OUTPUT: &'static str = "File.log";
const LOG_OUTPUT: &'static str = "traces.log";

fn main() {
    let selections = &[
        "List All items",
        "List TO-DO items",
        "Add item",
        "Modify Item",
        "Remove Item",
        "Change list",
        "Close"
    ];

    m_logger::init_logs(LOG_OUTPUT);
    let dialogue_manager = Dialogue_manager::default();
    let mut list_manager = ListManager::new(&dialogue_manager);
    let mut my_listing: Listing = listing::load_json(FILE_OUTPUT);
    let mut exit = false;
    while !exit {
        let selection = dialogue_manager.get_selection_default(selections.to_vec());
        match selection {
            0 => println!("{}", my_listing.pretty_printing_()),
            1 => println!("{}", my_listing.pretty_printing(&my_listing.filter_completed())),
            2 => list_manager.add_item(&mut my_listing),
            3 => list_manager.modify_item(&mut my_listing),
            4 => list_manager.remove_item(&mut my_listing),
            5 => list_manager.change_list(),
            6 => exit = true,
            _ => println!("Unknown choice")
        }
    }
    /*println!("{}", my_listing.pretty_printing_());
    my_listing.emplace("Gato".to_string());
    my_listing.emplace("Perro".to_string());*/
    my_listing.write_json(FILE_OUTPUT).unwrap();
}
