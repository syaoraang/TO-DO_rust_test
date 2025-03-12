use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::listing::ListItem;

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
pub enum ListType
{
    #[default]
    Todo,
    Done
}

impl PartialEq for ListType {
    fn eq(&self, other: &Self) -> bool {
        self == &ListType::Done && other == &ListType::Done ||
            self == &ListType::Todo && self == &ListType::Todo
    }
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