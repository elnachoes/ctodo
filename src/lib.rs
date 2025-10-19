mod config;
pub use config::Config;

mod program;
pub use program::program;

mod template;
pub use template::{
    TEMPLATE_FILE_NAME,
    get_template_file
};

extern crate dirs;

pub fn get_base_dir() -> String {
    if cfg!(target_os = "windows") {
        "C:/ctodo".to_string()
    } else {
        format!("{}/ctodo", dirs::home_dir().unwrap().display())
    }
}
