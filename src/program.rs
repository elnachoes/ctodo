use std::{
    env::args_os, 
    fs::{read_dir, DirBuilder, File}, 
    io::Write, 
    process::Command
};

use chrono::prelude::*;

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
        Command::new("cmd").args(["/C".to_string(), format!("{} {}", editor, get_full_path_for_date(date))]).spawn().expect("failed to open in editor");
    } else {
        Command::new("sh").args(["-c".to_string(), format!("{} {}", editor, get_full_path_for_date(date))]).spawn().expect("failed to open in editor");
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

pub fn program() {
    check_dir_setup();
    let config = Config::load();
    let args : Vec<String> = args_os().map(|arg| arg.into_string().unwrap()).collect();

    let editor = args.iter().skip_while(|arg| *arg != "-e").skip(1).next().or(Some(&config.editor)).unwrap();
    let date = args.iter().skip_while(|arg| *arg != "-d").skip(1).next().map_or(Local::now(),|arg| date_parser(&arg).unwrap());

    open_date_in_editor(date, editor)
}