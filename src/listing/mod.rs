mod list_type;

use std::string::String;
use list_type::ListType;
use std::fmt::{Display, Error, Formatter};
use std::{fs, io, usize};
use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Read, Write};
use std::fs::File;
//use serde_json::Value::String;

struct Group(String);

impl Default for Group
{
    fn default() -> Self {
        Group(String::from("Default"))
    }
}

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ListItem
{
    text: String,
    id: u8,
    status: ListType,
    group: String
}


impl PartialEq for ListType {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl ListItem {
    fn is_done(&self) -> bool
    {
        return match self.status
        {
            ListType::Done => true,
            _ => false
        }
    }
}
#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct Listing
{
    internal_list: Vec<ListItem>,
    last_id: u8
}

impl Listing {

    pub fn num_items(&self) -> usize
    {
        return self.internal_list.len()
    }
    fn new() -> Listing
    {
        return Listing::default();
    }

    pub fn add(&mut self, item:ListItem)
    {
        self.internal_list.push(item);
    }

    pub fn emplace(&mut self, text:String)
    {
        let new_item:ListItem = ListItem {text, id:self.last_id as u8 +1, status:ListType::Todo, group: String::from("Default")};
        self.internal_list.push(new_item);
        self.last_id+=1;
    }

    pub fn emplace_group(&mut self, text:String, group: String)
    {
        let new_item:ListItem = ListItem {text, id:self.last_id as u8 +1, status:ListType::Todo, group };
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

    pub fn remove(&mut self, id:u8) -> bool
    {
        match self.find_position(id){
            Some(t) => match self.internal_list.remove(t) {
                t => true
            },
            None => false
        }
    }

    pub fn update_text(&mut self, id:u8, text: String)
    {
        let mut item:&mut ListItem = self.find_item_mut(id).unwrap();
        //item.text.clear();
        item.text = text;
    }

    pub fn pretty_printing_(&self) -> String
    {
        let list = &self.internal_list.iter().collect();
        self.pretty_printing(list)
    }
    pub fn pretty_printing(&self, list:&Vec<&ListItem>) -> String
    {
        if list.is_empty()
        {
            return String::from("None");
        }
        let mut internal_string: String = String::new();
        for item in list
        {
            internal_string.push_str(&format!("ID:{}\nTask: {}\nStatus: {}\n\n\n", item.id, item.text, item.status));
        }
        return internal_string;
    }

    pub fn to_json(&self) -> String
    {
        return serde_json::to_string(self).expect("FATAL");
    }

    pub fn from_json(&self, ser_string:String) -> Result<ListItem, bool>
    {
        serde_json::from_str(ser_string.as_str()).unwrap_or_else(|e| Err(false))
    }

    pub fn load_json(&mut self, file_path:&str) -> Result<bool, io::Error>
    {
        /*let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let m_res = match line{
                Err(_) => continue,
                Ok(t ) => self.from_json(t)
            };
            match m_res{
                Err(_) => continue,
                Ok(t ) =>  self.add(t)
            };
        }*/
        let message = match fs::read_to_string(file_path)
        {
            Ok(t) => t,
            Err(_) => { fs::File::create(file_path)?; return Ok(true)}
        };
        self.from_json(message);
        Ok(true)
    }

    pub fn write_json(&mut self, file_path:&str) -> Result<bool, io::Error>
    {
        let mut file = File::open(file_path)?;//.unwrap_or(File::create(file_path)?);
        /*for item in list.iter() {
            write!(file, "{}", item)?;
        }*/
        let my_json = self.to_json();
        //write!(file, "{}", my_json)?;
        file.write_all(my_json.as_bytes());
        Ok(true)
    }

    pub fn filter_completed(&self) -> Vec<&ListItem>
    {
        self.internal_list.iter().filter(|x| x.is_done()).collect()
    }

    pub fn filter_by_group(&self, group:String) -> Vec<&ListItem>
    {
        self.internal_list.iter(). filter(|x| x.group == group).collect()
    }
}