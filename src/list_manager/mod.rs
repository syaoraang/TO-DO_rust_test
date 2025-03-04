use std::fs;
use dialoguer::{Select, theme::ColorfulTheme, Input};
use crate::listing;
use crate::listing::{Listing};
use crate::dialogue_manager::Dialogue_manager;

const PATH_LOGS: &'static str = "./logs/";
const PATH_FILES: &'static str = "./files/";

pub struct ListManager
{
    files_path: String,
    current_list: Listing,
    list_files: Vec<String>,
    num_file: usize,
    dialogue_manager: Dialogue_manager
}

impl Default for ListManager
{
    fn default() -> Self {
        let mut list = ListManager { files_path: PATH_FILES.to_string(), current_list: Listing::default(), list_files: Vec::new(), num_file: 0, dialogue_manager: Dialogue_manager::default() };
        list
    }
}
impl ListManager {

    pub fn new(dialogue_manager: &Dialogue_manager) -> Self {
        ListManager { files_path: PATH_FILES.to_string(), current_list: Listing::default(), list_files: Vec::new(), num_file: 0, dialogue_manager: dialogue_manager.clone() }
    }
    fn load_listing(&mut self, index: usize)
    {
        self.current_list = listing::load_json(self.list_files[index].as_str())
    }
    fn retrieve_files(&mut self) {
        let mut list: Vec<String> = Vec::new();
        if !fs::exists(PATH_FILES).unwrap()
        {
            return;
        }
        for entry in fs::read_dir(PATH_FILES).unwrap() {
            let entry = entry.unwrap().path().canonicalize().unwrap().to_str().unwrap().to_string();
            list.push(entry);
        }
        self.list_files = list;
    }


    pub fn change_list(&mut self)
    {
        let mut empty_lists = false;
        //let mut mylisting: Listing = Listing::default();
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
        if self.list_files.is_empty()
        {
            let selection = self.dialogue_manager.get_selection("Nothing to list", selections_empty.to_vec());

            match selection {
                0 => {println!("Returning");},
                _ => {println!("Bad choice, try again");}
            }
        }
        else
        {
            while !exit
            {
                self.change_list_inner( &selections_lists);
            }
        }
    }

    pub fn change_list_inner(&mut self, selections_lists: &[&str]) {
        let exit: bool;

        let selection = self.dialogue_manager.get_selection_default(selections_lists.to_vec());
        match selection {
            0 => { println!("{}", self.pretty_printing_files()) },
            1 => println!("{}", match self.dialogue_manager.get_input_i32("Get index") {
                x if x < 0 => "Bad index",
                x if (x > 0 && x <= self.list_files.len() as i32) => {
                    self.load_listing(x as usize);
                    "Modified"
                },
                _ => "Bad choice"
            }),
            2 => exit = true,
            _ => println!("Bad choice, try again")
        }
    }

    pub fn add_item(&mut self, listing: &mut Listing)
    {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter text")
            .interact_text()
            .unwrap();
        let input = self.dialogue_manager.get_input_string("Enter text");
        listing.emplace(input)
    }

    pub fn modify_item(&mut self, listing: &mut Listing)
    {
        let selections = &[
            "List Items",
            "Select ID",
            "Close"
        ];
        let mut exit: bool = false;
        while !exit
        {
            let selection = self.dialogue_manager.get_selection_default(selections.to_vec());
            match selection {
                0 => println!("{}", listing.pretty_printing_()),
                1 => println!("{}", match self.dialogue_manager.get_input_i32("Select_index") {
                    x if x < 0 => "Bad index",
                    x if (x > 0 && x <= listing.num_items() as i32) => {
                        match listing.update_text(x as u8, self.dialogue_manager.get_input_string("Input"))
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

    pub fn remove_item(&mut self, listing: &mut Listing)
    {
        let selections = &[
            "List Items",
            "Select ID",
            "Close"
        ];
        let mut exit: bool = false;
        while !exit
        {
            let selection = self.dialogue_manager.get_selection_default(selections.to_vec());
            match selection {
                0 => println!("{}", listing.pretty_printing_()),
                1 => println!("{}", match self.dialogue_manager.get_input_i32("Get index") {
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
        let mut list = &self.list_files;
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
}



