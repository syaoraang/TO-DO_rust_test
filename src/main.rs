/*  Tener dos listas, una de to-do y otras done
    En el menú elegir primero qué lista manejar, y luego ver, editar o borrar item
    También agregar una opción para agregar directamente, y se irá directo a to-do
    Guardar listas en archivos y cargarlas al iniciar
*/

mod listing;
mod m_logger;

//use log::{error, info, warn, debug};
use log4rs;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use crate::listing::{Listing, ListItem};

const FILE_OUTPUT:&'static str = "File.log";

fn add_item(listing: &mut Listing)
{
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter text")
        .interact_text()
        .unwrap();
    listing.emplace(input)
}

fn get_index(prompt_str: &str) -> i32
{
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_str)
        .interact_text()
        .unwrap();
    return input.parse::<i32>().unwrap_or(-1)
}

fn get_new_text(prompt_str: &str) -> String
{
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_str)
        .interact_text()
        .unwrap();
    return input
}

fn modify_item(listing: &mut Listing)
{
    let selections = &[
        "List Items",
        "Select ID",
        "Close"
    ];
    let mut exit:bool = false;
    while !exit
    {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select option")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
        match selection {
            0 => println!("{}", listing.pretty_printing_()),
            1 => println!("{}", match get_index("Select_index") {
                x if x < 0 => "Bad index",
                x if (x > 0 && x <= listing.num_items() as i32) => {
                    listing.update_text(x as u8, get_new_text("Input"));
                    "Modified"
                },
                _ => "Bad choice"
            }),
            2 => exit = true,
            _ => println!("Bad choice, try again")
        }
    }
}

fn remove_item(listing: &mut Listing)
{
    let selections = &[
        "List Items",
        "Select ID",
        "Close"
    ];
    let mut exit:bool = false;
    while !exit
    {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select option")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
        match selection {
            0 => println!("{}", listing.pretty_printing_()),
            1 => println!("{}", match get_index("Get index") {
                x if x < 0 => "Bad index",
                x if (x > 0 && x <= listing.num_items() as i32) => {
                    listing.remove(x as u8);
                    "Modified"
                },
                _ => "Bad choice"
            }),
            2 => exit = true,
            _ => println!("Bad choice, try again")
        }
    }
}

fn main() {
    let selections = &[
        "List All items",
        "List TO-DO items",
        "Add item",
        "Modify Item",
        "Remove Item",
        "Close"
    ];
    // Using configuration files
    // log4rs::init_file("src/conf_log4rs.yaml", Default::default()).unwrap();
    // Using mini-framework
    // m_logger::init_logs("file.log");
    // Example of usage
    /*debug!("Debug log");
    error!("{}", "Error log");
    info!("{:?}", "Info log");
    warn!("{:#?}", "Warn log");*/
    let mut my_listing: listing::Listing = listing::Listing::default();
    my_listing.load_json(FILE_OUTPUT).unwrap();
    /*let mut exit = false;
    while !exit {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your option")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
        match selection {
            0 => println!("{}", my_listing.pretty_printing_()),
            1 => println!("{}", my_listing.pretty_printing(&my_listing.filter_completed())),
            2 => add_item(&mut my_listing),
            3 => modify_item(&mut my_listing),
            4 => remove_item(&mut my_listing),
            5 => exit=true,
            _ => println!("Unknown choice")
        }
    }*/
    my_listing.emplace("Gato".to_string());
    my_listing.emplace("Perro".to_string());
    my_listing.write_json(FILE_OUTPUT).unwrap();
}
