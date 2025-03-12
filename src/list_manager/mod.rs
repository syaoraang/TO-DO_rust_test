use std::{default, env, fs, io};
use std::fs::File;
use std::io::Write;
use dialoguer::{Select, theme::ColorfulTheme, Input};
use log::{debug, info};
use crate::{listing, DEFAULT_LIST_FILE};
use crate::listing::{ListItem, Listing};
use crate::dialogue_manager::Dialogue_manager;

pub const PATH_LISTS: &'static str = "./files/";

pub struct ListManager
{
    default_list_path: String,
    current_list: Listing,
    current_path: String,
    list_files: Vec<String>,
    dialogue_manager: Dialogue_manager
}

impl ListManager {
    pub fn get_current_list(&self) -> & Listing {
        &self.current_list
    }
}

impl Default for ListManager
{
    fn default() -> Self {
        let default_folder = std::path::Path::new(PATH_LISTS);
        let default_list_path: String = default_folder.join(DEFAULT_LIST_FILE).to_str().unwrap().to_string();
        if!default_folder.exists() {
            fs::create_dir(&default_folder).expect("Could not create directory");
            File::create(&default_list_path).unwrap();
        }
        let default_listing = ListManager::load_json(&default_list_path);
        let mut list_files = Vec::new();
        list_files.push(default_list_path.to_string());
        ListManager { default_list_path: default_list_path.clone(), current_path: default_list_path, current_list: default_listing, list_files: list_files, dialogue_manager: Dialogue_manager::default() }
    }
}
impl ListManager {

    pub fn new(dialogue_manager: &Dialogue_manager) -> Self {
        let mut default_list_manager = ListManager::default();
        default_list_manager.dialogue_manager = dialogue_manager.clone();
        if default_list_manager.current_list.num_items() == 0 {
            default_list_manager.populate_listing();
        }
        default_list_manager
    }
    fn load_listing(&mut self, index: usize)
    {
        self.write_json().unwrap();
        self.current_list = ListManager::load_json(self.list_files[index].as_str());
        self.current_path = self.list_files[index].clone();
    }
    fn retrieve_files(&mut self) {
        let mut list: Vec<String> = Vec::new();
        if !fs::exists(PATH_LISTS).unwrap()
        {
            return;
        }
        for entry in fs::read_dir(PATH_LISTS).unwrap() {
            let entry = entry.unwrap().path().canonicalize().unwrap().to_str().unwrap().to_string();
            list.push(entry);
        }
        self.list_files = list;
    }


    pub fn change_list(&mut self)
    {
        let selections_lists: &'static [&str] = &[
            "List lists",
            "Select ID",
            "Close"
        ];
        let selections_empty: &'static [&str] = &[
            "Close"
        ];
        self.retrieve_files();
        let mut exit: bool = false;
        let mut selection: usize;
        if self.list_files.is_empty()
        {
            selection = self.dialogue_manager.get_selection("Nothing to list", selections_empty.to_vec());

            match selection {
                0 => {println!("Returning");},
                _ => {println!("Bad choice, try again");}
            }
        }
        else
        {
            self.change_list_inner(&selections_lists);
        }
    }

    pub fn change_list_inner(&mut self, selections_lists: &[&str]) {
        let mut exit: bool = false;
        while !exit {
            let selection = self.dialogue_manager.get_selection_default(selections_lists.to_vec());
            match selection {
                0 => {
                    println!("{}", self.pretty_printing_files())
                },
                1 => println!("{}", match self.dialogue_manager.get_input_i32("Get index") {
                    x if x < 0 => "Bad index",
                    x if (x >= 0 && x <= self.list_files.len() as i32) => {
                        self.load_listing(x as usize);
                        "Modified"
                    },
                    _ => "Bad choice"
                }),
                2 => exit = true,
                _ => println!("Bad choice, try again")
            }
        }
    }

    pub fn add_item(&mut self)
    {
        let input = self.dialogue_manager.get_input_string("Enter text");
        &self.current_list.emplace(input);
    }

    pub fn modify_item(&mut self)
    {
        let selections = &[
            "List Items",
            "Select ID",
            "Close"
        ];
        let mut listing: &mut Listing = &mut self.current_list;
        let dialogue_manager = &self.dialogue_manager;
        let mut exit: bool = false;
        while !exit
        {
            let selection = dialogue_manager.get_selection_default(selections.to_vec());
            match selection {
                0 => println!("{}", listing.pretty_printing_all()),
                1 => println!("{}", match dialogue_manager.get_input_i32("Select_index") {
                    x if x < 0 => "Bad index",
                    x if (x > 0 && x <= listing.num_items() as i32) => {
                        let input_text = dialogue_manager.get_input_string("Input");
                        match listing.update_text(x as u8, input_text)
                        {
                            true => "Item modified",
                            false => "Can't access the item",
                        }
                    },
                    _ => "Bad choice"
                }),
                2 => exit = true,
                _ => println!("Bad choice, try again")
            }
        }
    }

    pub fn remove_item(&mut self)
    {
        let mut listing: &mut Listing = &mut self.current_list;
        let dialogue_manager = &self.dialogue_manager;
        let selections = &[
            "List Items",
            "Select ID",
            "Close"
        ];
        let mut exit: bool = false;
        while !exit
        {
            let selection = dialogue_manager.get_selection_default(selections.to_vec());
            match selection {
                0 => println!("{}", listing.pretty_printing_all()),
                1 => println!("{}", match dialogue_manager.get_input_i32("Get index") {
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

    pub fn pretty_printing_files(&self) -> String
    {
        let list = &self.list_files;
        let mut internal_string = String::new();
        let mut i=0;
        if list.is_empty()
        {
            return String::from("");
        }
        for value in list
        {
            internal_string.push_str(&format!("{}. - {}", i, value));
            i += 1;
        }
        return internal_string;
    }

    pub fn load_json(file_path:&str) -> Listing
    {
        let message = match fs::read_to_string(file_path)
        {
            Ok(t) => {debug!("Read: {}", t);t},
            Err(_) => return Listing::default(),
        };
        match serde_json::from_str(message.as_str())
        {
            Ok(t) => return t,
            Err(e) => {println!("Error parsing the data"); return Listing::default()}
        }
    }

    pub fn from_json(ser_string:String) -> Result<Listing, bool>
    {
        serde_json::from_str(ser_string.as_str()).unwrap_or_else(|t| {
            println!("{}", t);
            Err(false)
        })
    }

    pub fn write_json(&self) -> Result<bool, io::Error>
    {
        let file_path = &self.current_path;
        self.write_json_to(file_path)
    }
    pub fn write_json_to(&self, file_path:&str) -> Result<bool, io::Error>
    {
        let mut file = match File::create(file_path) {
            Ok(t) => t,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other,format!("Could not create the file: {}\n{}", file_path, e))),
        };
        let my_json = serde_json::to_string(&self.current_list).expect("FATAL");
        file.write_all(&my_json.as_bytes())?;
        Ok(true)
    }

    pub fn pretty_printing_all(&self) -> String
    {
        self.current_list.pretty_printing_all()
    }
    pub fn pretty_printing_completed(&self) -> String
    {
        self.current_list.pretty_printing_completed()
    }

    fn populate_listing(&mut self)
    {
        let mut my_listing = &mut self.current_list;
        let items_list = &[
            "Gato",
            "Perro",
            "Hamster",
            "Loro",
            "Tortuga",
            "Canguro",
            "Oso",
        ];
        for &item in items_list
        {
            my_listing.emplace(item.to_string());
        }
    }
}