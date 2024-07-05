use chrono::prelude::*;

pub const TEMPLATE_FILE_NAME : &'static str = "todo.md";

pub fn get_template_file(date : DateTime<Local>) -> String {
    format!("
# ctodo : {}/{}/{}

- todo

- notes", date.day(), date.month(), date.year())
}