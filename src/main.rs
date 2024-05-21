/*  Tener dos listas, una de to-do y otras done
    En el menú elegir primero qué lista manejar, y luego ver, editar o borrar item
    También agregar una opción para agregar directamente, y se irá directo a to-do
    Guardar listas en archivos y cargarlas al iniciar
*/

mod m_listing;
mod m_logger;

//use log::{error, info, warn, debug};
use log4rs;
use std::borrow::Cow;

struct Items<'a, X> where [X]: ToOwned<Owned = Vec<X>> {
    values: Cow<'a, [X]>,
}

impl<'a, X: Clone + 'a> Items<'a, X> where [X]: ToOwned<Owned = Vec<X>> {
    fn new(v: Cow<'a, [X]>) -> Self {
        Items { values: v }
    }
}
fn main() {
    // Using configuration files
    // log4rs::init_file("src/conf_log4rs.yaml", Default::default()).unwrap();
    // Using mini-framework
    // m_logger::init_logs("file.log");
    // Example of usage
    /*debug!("Debug log");
    error!("{}", "Error log");
    info!("{:?}", "Info log");
    warn!("{:#?}", "Warn log");*/

    let mut my_listing:m_listing::Listing = m_listing::Listing::default();..
    my_listing.emplace("Dar amor a mi amorcito".to_string());
    println!("{}", &my_listing.pretty_printing());
    my_listing.emplace("Jugar con Benji".to_string());
    println!("{}", &my_listing.pretty_printing());
    my_listing.emplace("Aprender más Rust".to_string());
    println!("{}", &my_listing.pretty_printing());
    my_listing.update_text(3, "Aprender más lado obscuro".to_string());
    println!("{}", &my_listing.pretty_printing());
    &my_listing.remove(2);
    println!("{}", &my_listing.pretty_printing());
    println!("Serialized: {}", &my_listing.to_json());
    my_listing.emplace("Molestar al gato".to_string());
    println!("{}", &my_listing.pretty_printing());
    println!("Serialized: {}", &my_listing.to_json());
}
