use std::{default, env, fs, io};
use std::fs::File;
use std::io::Write;
use std::thread::current;
use dialoguer::{Select, theme::ColorfulTheme, Input};
use log::{debug, info};
use crate::listing::{ListItem, Listing};
use crate::dialogue_manager::Dialogue_manager;

const DEFAULT_LIST_FILE: &'static str = "File_list_default.txt";
pub const PATH_LISTS: &'static str = "./files/";

pub struct ListManager
{
    default_list_path: String,
    current_list: Listing,
    current_path: String,
    list_files: Vec<String>,
    dialogue_manager: Dialogue_manager
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
        let mut new_list_manager = ListManager { default_list_path: default_list_path.clone(), current_path: default_list_path, current_list: default_listing, list_files: list_files, dialogue_manager: Dialogue_manager::default() };

        new_list_manager.retrieve_files(&PATH_LISTS.to_owned());
        return new_list_manager
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

    pub fn get_current_list(&self) -> & Listing {
        &self.current_list
    }
    fn load_listing(&mut self, index: usize)
    {
        self.write_json().unwrap();
        self.current_list = ListManager::load_json(self.list_files[index].as_str());
        self.current_path = self.list_files[index].clone();
    }
    fn load_listing_by_path(&mut self, path: &String)
    {
        self.current_list = ListManager::load_json(path.as_str());
        self.current_path = path.clone();
    }

    fn load_default_listing(&mut self)
    {
        self.current_list = ListManager::load_json(self.default_list_path.as_str());
        self.current_path = self.default_list_path.clone();
    }

    fn retrieve_files(&mut self, path: &String) {
        let mut list: Vec<String> = Vec::new();
        if !fs::exists(path).unwrap()
        {
            return;
        }
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path().canonicalize().unwrap().to_str().unwrap().to_string();
            list.push(entry);
        }
        self.list_files = list;
    }

    pub fn manage_items(&mut self)
    {
        let selections_lists: &'static [&str] = &[
            "Show items",
            "Add item",
            "Modify Item",
            "Remove Item",
            "Close"
        ];
        let mut exit = false;
        while !exit {
            let mut selection = self.dialogue_manager.get_selection("Select an option", selections_lists.to_vec());
            match selection {
                0 => self.show_items(),
                1 => self.add_item(),
                2 => self.modify_item(),
                3 => self.remove_item(),
                _ => exit = true
            }
        }
    }

    pub fn show_items(&mut self)
    {
        let selections_lists: &'static [&str] = &[
            "List TODO items",
            "List completed items",
            "Close"
        ];
        let mut exit = false;
        while !exit {
            let mut selection = self.dialogue_manager.get_selection("Select an option", selections_lists.to_vec());
            match selection {
                0 => println!("{}", self.pretty_printing_todo()),
                1 => println!("{}", self.pretty_printing_completed()),
                _ => exit = true
            }
        }
    }

    pub fn manage_lists(&mut self){
        let selections_lists: &'static [&str] = &[
            "List lists",
            "Create new list",
            "Delete list",
            "Change list",
            "Close"
        ];
        let mut exit = false;
        while !exit {
            let mut selection = self.dialogue_manager.get_selection("Select an option", selections_lists.to_vec());
            match selection {
                0 => { println!("{}", &self.pretty_printing_files());},
                1 => { &self.create_new_list(); },
                2 => { &self.remove_lists(); },
                3 => { &self.change_list(); },
                _ => exit = true
            }
        }
    }

    pub fn get_list_file(&mut self, index:usize) -> String
    {
        self.list_files[index].clone()
    }
    pub fn remove_lists(&mut self)
    {
        //let mut listing: &mut Listing = &mut self.current_list;
        let dialogue_manager = &self.dialogue_manager.clone();
        let current_values = "0 - Exit\n".to_owned() + &self.pretty_printing_files();
        let selections_items: Vec<&str> = current_values.split('\n').collect();
        let selections_gotten = dialogue_manager.get_multiple_input_string("Select at least one item".to_owned(), &selections_items);
        if selections_gotten == [0] || (selections_gotten.len() == 1 && selections_gotten[0] > selections_items.len()) {
            return
        }
        for selection in selections_gotten {
            let choosen = selection - 1;
            let file_to_remove = self.get_list_file(choosen);
            let final_choice = dialogue_manager.get_selection(&format!("Do you really want to delete file: {}", &file_to_remove), vec!("No", "Yes"));
            match final_choice {
                0 => {println!("Skipping"); },
                1 => {
                    println!("Removing file: {}", &file_to_remove);
                    if file_to_remove == self.current_path
                    {
                        if file_to_remove == self.default_list_path
                        {
                            self.current_list = Listing::default();
                        }
                        else
                        {
                            self.load_default_listing();
                        }
                        self.write_json().unwrap();
                    }
                    fs::remove_file(file_to_remove).unwrap()
                },
                _ => {println!("Bad choice"); }
            }
        }
        self.retrieve_files(&PATH_LISTS.to_owned());
    }

    pub fn create_new_list(&mut self)
    {
        let dialogue_manager =  &self.dialogue_manager;
        let list_name = dialogue_manager.get_input_string("Enter new list name (empty to cancel): ");
        if list_name == "" {
            println!("No name given, returning");
            return;
        }
        let default_folder = std::path::Path::new(PATH_LISTS);
        let new_list_path: String = default_folder.join(list_name).to_str().unwrap().to_string();
        File::create(&new_list_path).unwrap();
        self.retrieve_files(&PATH_LISTS.to_owned());
    }

    pub fn change_list(&mut self)
    {

        //let mut listing: &mut Listing = &mut self.current_list;
        let dialogue_manager = &self.dialogue_manager.clone();
        let current_values = "0 - Exit\n".to_owned() + &self.pretty_printing_files();
        let selections_items: Vec<&str> = current_values.split('\n').collect();
        let selections_gotten = dialogue_manager.get_selection("Select one item", selections_items);
        if selections_gotten == 0 {
            println!("No list selected, returning");
            return;
        }
        self.load_listing(selections_gotten - 1);
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
        let selections_lists: &'static [&str] = &[
            "Change status",
            "Add tag",
            "Remove tag",
            "Modify group",
            "Close"
        ];
        let mut listing: &mut Listing = &mut self.current_list;
        let dialogue_manager = &self.dialogue_manager;
        let current_values = "0 - Exit\n".to_owned() + &listing.pretty_printing_all_minimum();
        let selections_items: Vec<&str> = current_values.split('\n').collect();
        let selections_gotten = dialogue_manager.get_multiple_input_string("Select at least one item".to_owned(), &selections_items);
        if selections_gotten == [0] {
            return
        }
        for selection in selections_gotten {
            let item_id = selection as u8;
            let mut exit = false;
            while !exit {
                let mut selection = self.dialogue_manager.get_selection("Select an option", selections_lists.to_vec());
                match selection {
                    0 => {
                        if listing.is_item_done(item_id)
                        {
                            match listing.unmark_as_done(item_id)
                            {
                                true => { println!("Unmarked as TODO"); }
                                _ => { println!("Can't unmark as TODO"); }
                            }
                        }
                        else
                        {
                            match listing.mark_as_done(item_id)
                            {
                                true => { println!("Marked as TODO"); }
                                _ => { println!("Can't Mark as TODO"); }
                            }
                        }
                    },
                    1 => {
                        let tag = dialogue_manager.get_input_string_allow_empty("Type new tag");
                        if (tag.is_empty())
                        {
                            println!("Returning; no tag");
                        }
                        listing.add_tag_item(item_id, tag);
                    },
                    2 => {
                        let current_values = "0 - Exit\n".to_owned() + &listing.pretty_printing_tags(item_id);
                        let selections_items: Vec<&str> = current_values.split('\n').collect();
                        let selections_gotten = dialogue_manager.get_multiple_input_string("Select at least one item".to_owned(), &selections_items);
                        if selections_gotten == [0] {
                            return
                        }
                        for selection in selections_gotten {
                            let tag_str = selections_items[selection];
                            let final_choice = dialogue_manager.get_selection(&format!("Do you really want to delete tag: {}", &tag_str), vec!("No", "Yes"));
                            match final_choice {
                                0 => {println!("Skipping"); },
                                _ => {listing.remove_tag_item(item_id, tag_str.to_string());}
                            }
                        }
                    },
                    3 => {
                        let group_str = dialogue_manager.get_input_string_allow_empty("Type new group");
                        if (group_str.is_empty())
                        {
                            println!("Returning; no group");
                        }
                        listing.change_group(item_id, group_str);
                    }
                    _ => exit = true
                }
            }
            let input_text = dialogue_manager.get_input_string_allow_empty(
                &format!("Item: {}\n Enter new text or blank to cancel", &selections_items[selection])
            );
            if input_text.is_empty() {
                continue
            }
            match listing.update_text(selection as u8, input_text)
            {
                true => println!("Item modified"),
                false => println!("Can't access the item"),
            }
        }
    }

    pub fn remove_item(&mut self)
    {
        let mut listing: &mut Listing = &mut self.current_list;
        let dialogue_manager = &self.dialogue_manager;
        let current_values = "0 - Exit\n".to_owned() + &listing.pretty_printing_all_minimum();
        let selections_items: Vec<&str> = current_values.split('\n').collect();
        let selections_gotten = dialogue_manager.get_multiple_input_string("Select at least one item".to_owned(), &selections_items);
        if selections_gotten == [0] {
            return
        }
        for selection in selections_gotten {
            listing.remove(selection as u8);
        }
    }

    pub fn pretty_printing_files(&self) -> String
    {
        let list = &self.list_files;
        let mut internal_string = String::new();
        let mut i=1;
        if list.is_empty()
        {
            return String::from("");
        }
        for (index, value) in list.iter().enumerate()
        {
            let mut ending = "\n";
            if index == list.len() - 1
            {
                ending = "";
            }
            internal_string.push_str(&format!("{}. - {}{}", i, value, ending));
            i += 1;
        }
        return internal_string;
    }

    pub fn load_json(file_path:&str) -> Listing
    {
        let message = match fs::read_to_string(file_path)
        {
            Ok(t) => t,
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

    pub fn pretty_printing_todo(&self) -> String
    {
        self.current_list.pretty_printing_todo()
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