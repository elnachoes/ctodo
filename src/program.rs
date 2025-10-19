
use clap::Parser;

use std::{
    fs::{read_dir, DirBuilder, File}, 
    io::Write, 
    io::stdout,
    process::Command
};

use chrono::{prelude::*, TimeDelta};

use crate::{
    get_base_dir,
    Config,
    TEMPLATE_FILE_NAME,
    get_template_file
};

fn init_folder() {
    let base_dir = get_base_dir();
    DirBuilder::new().recursive(true).create(base_dir).unwrap();
    Config::reset();
}

fn get_target_dir_for_date(date : DateTime<Local>) -> String {
    format!("{}/{}/{}/{}", get_base_dir(), date.year(), date.month(), date.day())
}

fn get_full_path_for_date(date : DateTime<Local>) -> String {
    format!("{}/{}", get_target_dir_for_date(date), TEMPLATE_FILE_NAME)
}

fn init_todo_for_date(date : DateTime<Local>) {
    DirBuilder::new().recursive(true).create(get_target_dir_for_date(date)).unwrap();
    let mut f = File::options().write(true).create(true).open(get_full_path_for_date(date)).unwrap();
    f.write_all(get_template_file(date).as_bytes()).unwrap();
}

fn check_todo_for_date(date : DateTime<Local>) {
    if File::options().read(true).open(get_full_path_for_date(date)).is_err() {
        init_todo_for_date(date)
    }
}

fn check_dir_setup() {
    if read_dir(get_base_dir()).is_err() {
        init_folder()
    }
}

fn open_date_in_editor(date : DateTime<Local>, editor : &str) {
    check_todo_for_date(date);
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C".to_string(), format!("{} {}", editor, get_full_path_for_date(date))])
            .stdout(stdout())
            .output()
            .expect("failed to open in editor");
    } else if cfg!(target_os = "linux") {
        Command::new(editor)
            .args([format!("{}", get_full_path_for_date(date))])
            .stdout(stdout())
            .output()
            .expect("failed to open in editor");
    } else {
        unimplemented!("ctodo is not implemented for this os.")        
    }
}

fn parse_int(input : &str) -> Result<(&str, u64), ()> {
    let number_string : String = input.chars().take_while(|c| c.is_numeric()).into_iter().collect::<String>();
    let number_result = number_string.parse().map_err(|_| ())?;
    Ok((&input[number_string.len()..], number_result))
}

fn parse_slash<'a>(input : &'a str) -> Result<&'a str, ()> {
    if input.chars().next().ok_or(())? == '/' { Ok(&input[1..]) } else { Err(()) }
}

fn date_parser(input : &str) -> Result<DateTime<Local>, ()> {
    let (remaining, day) = parse_int(input)?;
    let remaining = parse_slash(remaining)?;
    let (remaining, month) = parse_int(remaining)?;
    let remaining = parse_slash(remaining)?;
    let (_remaining, year) = parse_int(remaining)?;

    Ok(DateTime::<Local>::from(Local.with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0).unwrap()))
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RawArgs {
    #[arg(short, long, default_value_t = false)]
    yesterday: bool,

    #[arg(short, long, default_value_t = false)]
    tommorrow: bool,

    #[arg(short, long, default_value_t = String::default())]
    date: String,

    #[arg(short, long, default_value_t = String::default())]
    editor: String,
}
#[derive(Default)]
struct Args {
    date: DateTime<Local>,
    editor: String,
}
impl Args {
    pub fn parse_with_config(config : &Config) -> Self {
        let mut args = Args::default();
        let raw_args = RawArgs::parse();

        args.editor = if raw_args.editor.is_empty() {
             config.editor.to_string()
        } else {
            raw_args.editor
        };

        args.date = if raw_args.yesterday {
            Local::now() - TimeDelta::days(1)
        } else if raw_args.tommorrow {
            Local::now() + TimeDelta::days(1)
        } else if raw_args.date.is_empty() {
            Local::now()
        } else {
            date_parser(&raw_args.date).unwrap()
        };
        
        args
    }
}

pub fn program() {
    check_dir_setup();
    let config = Config::load();
    let args = Args::parse_with_config(&config);
    open_date_in_editor(args.date, &args.editor)
}
