mod list_type;

use std::string::String;
use list_type::ListType;
use std::fmt::{Display};
use std::{io, usize};
use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Read, Write};
use std::fs::File;
use log::{debug, info};

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

    pub fn update_text(&mut self, id:u8, text: String) -> bool
    {
        return match self.find_item_mut(id)
        {
            Some(list_item) => {
                list_item.text = text;
                true
            },
            _ => false
        }
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
            internal_string.push_str(&format!("ID:{}\nTask: {}\nStatus: {}\n\n\n", &item.id, &item.text, &item.status));
        }
        return internal_string;
    }

    pub fn pretty_printing_all(&self) -> String
    {
        self.pretty_printing(&self.internal_list.iter().collect())
    }
    pub fn pretty_printing_completed(&self) -> String
    {
        self.pretty_printing(&self.filter_completed())
    }

    pub fn filter_completed(&self) -> Vec<&ListItem>
    {
        self.internal_list.iter().filter(|&x| x.is_done()).collect::<Vec<&ListItem>>()
    }

    pub fn get_all_items(&self) -> &Vec<ListItem> {
        return &self.internal_list
    }
    pub fn filter_by_group(&self, group:String) -> Vec<&ListItem>
    {
        self.internal_list.iter(). filter(|x| x.group == group).collect()
    }
}