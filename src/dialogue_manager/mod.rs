use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use crate::listing::ListItem;

pub struct Dialogue_manager
{
    theme: ColorfulTheme,
    empty_string: String
}

impl Default for Dialogue_manager
{
    fn default() -> Self {
        let theme = ColorfulTheme::default();
        return Self {
            theme,
            empty_string: "Close".to_owned(),
        };
    }
}

impl Clone for Dialogue_manager {
    fn clone(&self) -> Self {
        Dialogue_manager {theme: ColorfulTheme::default(), empty_string: self.empty_string.clone()}
    }
}

impl Dialogue_manager
{
    pub fn get_input_string(&self, prompt_str: &str) -> String
    {
        self.get_input_string_base(prompt_str, false)
    }
    pub fn get_input_string_allow_empty(&self, prompt_str: &str) -> String
    {
        self.get_input_string_base(prompt_str, true)
    }
    pub fn get_input_string_base(&self, prompt_str: &str, allow_empty: bool) -> String
    {
        let input: String = Input::with_theme(&self.theme)
            .with_prompt(prompt_str)
            .allow_empty(allow_empty)
            .interact_text()
            .unwrap();
        return input;
    }

    pub fn get_multiple_input_string(&self, prompt_str: String, items: &Vec<&str>) -> Vec<usize>
    {
        let input = MultiSelect::with_theme(&self.theme)
            .with_prompt(&prompt_str)
            .items(items)
            .interact()
            .unwrap();
        return input;
    }

    pub fn get_input_i32(&self, prompt_str: &str) -> i32
    {
        self.get_input_string(prompt_str).parse::<i32>().unwrap_or(-1)
    }

    pub fn get_selection(&self, prompt_str: &str, options: Vec<&str>) -> usize
    {
        let selection = Select::with_theme(&self.theme)
            .with_prompt(prompt_str)
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();
        selection
    }
    pub fn get_selection_default(&self, options: Vec<&str>) -> usize
    {
        self.get_selection("Select an option", options)
    }

}