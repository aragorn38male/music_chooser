use csv::ReaderBuilder;
use rand::Rng;
use std::error::Error;
use std::fs;
use std::fs::read_dir;
use std::fs::File;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
    if !fs::metadata("music.lst").is_ok() {
        println!("\nYou're WELCOME! Indexing your MUSIC...");
        let dirs = list_dir();
        let _ = write_csv(dirs);
    }
    let list = read_csv();
    let list_length = list.as_ref().expect("PROBLEM!").len() as usize;
    let rand_item = rand::thread_rng().gen_range(0..list_length);
    println!(
        "\nYour daily MUSIC album to listen to: {:?}",
        &list.unwrap()[rand_item]
    );
    let _ = cut_list(rand_item as u32);
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn list_dir() -> Vec<String> {
    let mut directories_list: Vec<String> = vec![];
    for entry_res in read_dir(".").unwrap() {
        let entry = entry_res.unwrap();
        let file_name_buf = entry.file_name();
        let file_name = file_name_buf.to_str().unwrap();
        if !file_name.starts_with(".") && entry.file_type().unwrap().is_dir() {
            directories_list.push(file_name.to_string());
        }
    }
    return directories_list;
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn write_csv(list: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file = File::create("music.lst")?;
    let mut writer = csv::Writer::from_writer(file);
    for n in 0..list.len() {
        let _ = writer.write_record(&[list[n].clone()]);
    }
    let _ = writer.flush()?;
    return Ok(());
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn read_csv() -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open("music.lst")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut list: Vec<String> = Vec::new();
    for element in reader.records() {
        let record = element?;
        list.push(record[0].to_string());
    }
    return Ok(list);
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn cut_list(item: u32) -> Result<(), Box<dyn Error>> {
    let file = File::open("music.lst")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut list: Vec<String> = Vec::new();
    let mut iterator = 0;
    for element in reader.records() {
        let record = element?;
        if iterator != item {
            list.push(record[0].to_string());
        }
        iterator += 1;
    }

    if iterator > 1 {
        let _ = write_csv(list);
    } else {
        println!("\nGREAT JOB, you've listening to all of your MUSIC playlist !!!");
        let dirs = list_dir();
        let _ = write_csv(dirs);
    }
    return Ok(());
}
