use dialoguer::{theme::ColorfulTheme, Input, Select};


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
        let input: String = Input::with_theme(&self.theme)
            .with_prompt(prompt_str)
            .interact_text()
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