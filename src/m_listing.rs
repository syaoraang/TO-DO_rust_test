use std::fmt::{Display, Error, Formatter};
use std::{io, usize};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::fs::File;

#[derive(Default)]
#[derive(Serialize, Deserialize)]
enum ListType
{
    #[default]
    Todo,
    Done
}
impl Display for ListType
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let m_string:String = match self {
            ListType::Todo => String::from("TODO"),
            ListType::Done => String::from("DONE")
        };
        write!(f, "{}", m_string)
    }
}

#[derive(Default)]
#[derive(Serialize, Deserialize)]
struct ListItem
{
    text: String,
    id: u8,
    status: ListType
}

#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct Listing
{
    internal_list: Vec<ListItem>,
    last_id: u8
}

impl Listing {

    fn new() -> Listing
    {
        return Listing::default();
    }
    fn add(&mut self, item:ListItem)
    {
        self.internal_list.push(item);
    }

    pub fn emplace(&mut self, text:String)
    {
        let new_item:ListItem = ListItem {text, id:self.last_id as u8 +1, status:ListType::Todo};
        self.internal_list.push(new_item);
        self.last_id+=1;
    }
    fn find_item(&self, id:u8) -> Option<&ListItem>
    {
        self.internal_list.iter().find(| &x| x.id == id)
    }

    fn find_item_mut(&mut self, id:u8) -> Option<&mut ListItem>
    {
        self.internal_list.iter_mut().find(| x| x.id == id)
    }

    fn find_position(&self, id:u8) -> Option<usize>
    {
        self.internal_list.iter().position(|x| x.id==id)
    }
    pub fn remove(&mut self, id:u8)
    {
        let tmp_pos = self.find_position(id);
        if ( tmp_pos.is_some())
        {
            self.internal_list.remove(tmp_pos.unwrap());
        }
        else {
            println!("Can't find the id: {}", id);
        }
    }

    pub fn update_text(&mut self, id:u8, text: String)
    {
        let mut item:&mut ListItem = self.find_item_mut(id).unwrap();
        //item.text.clear();
        item.text = text;
    }

    pub fn pretty_printing(&self) -> String
    {
        let mut internal_string: String = String::new();
        for item in &self.internal_list
        {
            internal_string.push_str(&format!("ID:{}\nTask: {}\nStatus: {}\n\n\n", item.id, item.text, item.status));
        }
        return internal_string;
    }

    pub fn to_json(&self) -> String
    {
        return serde_json::to_string(self).expect("FATAL");
    }

    pub fn from_json(&self, ser_string:String) -> Result<ListItem, std::error>
    {
        return serde_json::from_str(ser_string.unwrap().as_str())?
    }
    fn load_json(&mut self, file_path:&str) -> Result<bool, io::Error>
    {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let m_res = match line{
                Err(_) => continue,
                Ok(t ) => self.from_json(t.unwrap())
            };
            let m_other = match m_res{
                Err(_) => continue,
                Ok(t ) =>  self.add(t.unwrap())
            };
        }
    }

    fn write_json(&mut self, file_path:&str, list:Vec<String>) -> Result<bool, io::Error>
    {
        let mut file = File::open(file_path).unwrap_or(File::create(file_path)?);
        for item in list.iter() {
            write!(file, "{}", item)?;
        }
        Ok(true)
    }
}