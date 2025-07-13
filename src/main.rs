use chrono::Local;
use colored::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    body: String,
    time: String,
    pwd: String,
}

fn main() {
    let mut args = env::args().skip(1);

    match args.next() {
        Some(arg) if arg == "-s" || arg == "--show" => show_notes(),
        Some(note) => add_note(note),
        None => eprintln!("No note provided. Please provide a note wrapped in double quotes."),
    }
}

fn add_note(note: String) {
    let path = match get_notes_file_path() {
        Some(p) => p,
        None => {
            eprintln!("Failed to get notes file path.");
            return; // todo -> handle error appropriately
        }
    };

    let mut notes = load_notes(&path);

    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from(""));

    let pwd = env::current_dir()
        .unwrap_or_else(|_| PathBuf::from(""))
        .strip_prefix(&home)
        .unwrap_or_else(|_| Path::new(""))
        .to_string_lossy()
        .to_string();

    let new_note = Note {
        body: note,
        time: Local::now().format("%Y-%m-%d %H:%M").to_string(),
        pwd,
    };

    notes.push(new_note);
    save_notes(&path, &notes);
    println!("Note saved ✅");
}

fn show_notes() {
    let path = match get_notes_file_path() {
        Some(p) => p,
        None => {
            eprintln!("Failed to get notes file path.");
            return; // todo -> handle error appropriately
        }
    };

    let notes = load_notes(&path);

    if notes.is_empty() {
        println!("No notes found!");
        return;
    }

    println!("\n------------------------- \n");
    for note in notes {
        println!("- {} {}", " PWD:".green().bold(), note.pwd);
        println!("- {} {}", "TIME:".blue().bold(), note.time);
        println!("- {} {}", "NOTE:".yellow().bold(), note.body);
        println!("\n------------------------- \n")
    }
}

fn get_notes_file_path() -> Option<PathBuf> {
    let data_dir = dirs::data_dir()?;
    let notes_dir = data_dir.join("nt");
    std::fs::create_dir_all(&notes_dir).ok()?;
    Some(notes_dir.join("notes.json"))
}

fn load_notes(path: &PathBuf) -> Vec<Note> {
    if !path.exists() {
        return vec![];
    }

    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

fn save_notes(path: &PathBuf, notes: &Vec<Note>) {
    let json = serde_json::to_string_pretty(notes).expect("Failed to serialize notes");
    fs::write(path, json).expect("Failed to write notes file");
}
