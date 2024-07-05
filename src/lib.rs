mod config;
pub use config::Config;

mod program;
pub use program::program;

mod template;
pub use template::{
    TEMPLATE_FILE_NAME,
    get_template_file
};

pub fn get_base_dir() -> &'static str {
    if cfg!(target_os = "windows") {
        "C:/ctodo"
    } else {
        "/ctodo"
    }
}