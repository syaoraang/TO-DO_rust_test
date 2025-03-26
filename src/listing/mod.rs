mod list_type;

use std::string::String;
use list_type::ListType;
use std::fmt::{format, Display};
use std::{io, usize};
use std::cmp::PartialEq;
use std::collections::{HashMap, LinkedList};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Read, Write};
use std::fs::File;
use log::{debug, info};
use std::collections::VecDeque;


#[derive(Clone)]
#[derive(Serialize, Deserialize)]
struct Group(String);

impl Default for Group
{
    fn default() -> Self {
        Group(String::from("Default"))
    }
}

/*impl Clone for Group
{
    fn clone(&self) -> Group {
        Group(self.0.clone())
    }
}*/

/*#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Tag
{
    tag_map: Hash<String>
}

impl Tag
{
    pub fn add_tag()
}*/

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ListItem
{
    text: String,
    id: u8,
    status: ListType,
    group: String,
    tags: VecDeque<String>
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
    fn mark_as_done(&mut self)
    {
        self.status = ListType::Done
    }

    fn unmark_as_done(&mut self)
    {
        self.status = ListType::Todo
    }

    fn update_text(&mut self, new_text: String)
    {
        self.text = new_text.clone();
    }

    fn change_group(&mut self, new_group: String)
    {
        self.group = new_group.clone();
    }

    fn is_on_group_x(&self, group_X: &String) -> bool
    {
        self.group.eq(group_X)
    }

    fn add_tag(&mut self, tag: String) -> bool
    {
        match self.tags.binary_search(&tag)
        {
            Ok(pos) => false,
            _ => { self.tags.push_back(tag); true }
        }
    }

    fn print_tags(&self) -> String
    {
        self.tags.iter().map(| tag | format!("{},", tag )).collect()
    }

    fn print_tags_enumerated(&self) -> String
    {
        self.tags.iter().enumerate().map(|(index, tag)| format!("{}. {}", index+1, tag)).collect()
    }

    fn remove_tag(&mut self, tag: String) -> bool
    {
        match self.tags.binary_search(&tag)
        {
            Ok(pos) => self.tags.remove(pos).is_some(),
            _ => false
        }
    }

    fn has_tag(&mut self, tag: String) -> bool
    {
        self.tags.binary_search(&tag).is_ok()
    }
}


#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct Listing
{
    internal_list: Vec<ListItem>,
    last_id: u8,
    tags: HashMap<u16, String>,
    groups: HashMap<u16, String>,
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
        let new_item:ListItem = ListItem {text, id:self.last_id as u8 +1, status:ListType::Todo, group: String::from("Default"), tags: VecDeque::new()};
        self.internal_list.push(new_item);
        self.last_id+=1;
    }

    pub fn emplace_group(&mut self, text:String, group: String)
    {
        let new_item:ListItem = ListItem {text, id:self.last_id as u8 +1, status:ListType::Todo, group, tags: VecDeque::new() };
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
                _ => true
            },
            None => false
        }
    }

    pub fn update_text(&mut self, id:u8, text: String) -> bool
    {
        return match self.find_item_mut(id)
        {
            Some(list_item) => {
                list_item.update_text(text);
                true
            },
            _ => false
        }
    }

    pub fn change_group(&mut self, id:u8, text: String) -> bool
    {
        return match self.find_item_mut(id)
        {
            Some(list_item) => {
                list_item.change_group(text);
                true
            },
            _ => false
        }
    }

    pub fn mark_as_done(&mut self, id:u8) -> bool
    {
        return match self.find_item_mut(id)
        {
            Some(list_item) => {
                list_item.mark_as_done();
                true
            },
            _ => false
        }
    }
    pub fn unmark_as_done(&mut self, id:u8) -> bool
    {
        return match self.find_item_mut(id)
        {
            Some(list_item) => {
                list_item.unmark_as_done();
                true
            },
            _ => false
        }
    }

    pub fn is_item_done(&mut self, id:u8) -> bool
    {
        return match self.find_item_mut(id)
        {
            Some(list_item) => {
                list_item.is_done()
            },
            _ => false
        }
    }

    pub fn add_tag_item(&mut self, item_id: u8, tag: String) -> bool
     {
         match self.find_item_mut(item_id)
         {
             Some(list_item) =>
             {
                 list_item.add_tag(tag);
                 true
             },
            _ => false
         }
    }

    pub fn remove_tag_item(&mut self, item_id: u8, tag: String) -> bool
    {
        match self.find_item_mut(item_id)
        {
            Some(list_item) =>
                {
                    list_item.remove_tag(tag)
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
    pub fn pretty_printing_minimum(&self, list:&Vec<&ListItem>) -> String
    {
        if list.is_empty()
        {
            return String::from("none");
        }
        let mut internal_string: String = String::new();
        for (index, item) in list.iter().enumerate()
        {
            let mut ending = "\n";
            if index == list.len() - 1
            {
                ending = "";
            }
            internal_string.push_str(&format!("{} - [{}] {} [#{} @{}]{}", &item.id, &item.status, &item.text, &item.group, &item.print_tags(), ending));
        }
        return internal_string;
    }

    pub fn pretty_printing_all(&self) -> String
    {
        self.pretty_printing(&self.internal_list.iter().collect())
    }
    pub fn pretty_printing_all_minimum(&self) -> String
    {
        self.pretty_printing_minimum(&self.internal_list.iter().collect())
    }
    pub fn pretty_printing_completed(&self) -> String
    {
        self.pretty_printing(&self.filter_completed())
    }

    pub fn pretty_printing_tags(&self, id: u8) -> String
    {
        match self.find_item(id)
        {
            Some(item) => item.print_tags_enumerated(),
            None => String::from(""),
        }
    }

    pub fn pretty_printing_todo(&self) -> String
    {
        self.pretty_printing(&self.filter_todo())
    }

    pub fn filter_completed(&self) -> Vec<&ListItem>
    {
        self.internal_list.iter().filter(|&x| x.is_done()).collect::<Vec<&ListItem>>()
    }

    pub fn filter_todo(&self) -> Vec<&ListItem>
    {
        self.internal_list.iter().filter(|&x| !x.is_done()).collect::<Vec<&ListItem>>()
    }

    pub fn get_all_items(&self) -> &Vec<ListItem> {
        return &self.internal_list
    }
    pub fn filter_by_group(&self, group:String) -> Vec<&ListItem>
    {
        self.internal_list.iter(). filter(|x| x.is_on_group_x(&group)).collect()
    }
}