use core::fmt;
use std::{error::Error, fmt::Display, fs::File, io::Read};

pub const DEFAULT_RUNTME_STACK_SZE: usize = 512;
pub enum FileError {
    FetchFile,
    CreateFile,
    OpenFile,
    ReadFile,
    WriteFile,
    Metadata,
    Run,
    Unknown,
}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::FetchFile => write!(f, "unable to fetch file extenssion.."),
            FileError::CreateFile => write!(f, "unable to create file name.."),
            FileError::OpenFile => write!(f, "unable to open file.."),
            FileError::ReadFile => write!(f, "unable to read file.."),
            FileError::WriteFile => write!(f, "unable to write file.."),
            FileError::Metadata => write!(f, "unable to read file metadata.."),
            FileError::Run => write!(f, "unable to run program.."),
            FileError::Unknown => write!(f, "invalid command! Use build or run command.."),
        }
    }
}

impl fmt::Debug for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl Error for FileError {}

pub type FileResult<T> = Result<T, FileError>;

pub enum Operation {
    Buid,
    Run,
    Unknown,
}
pub fn get_command(command: &str) -> Operation {
    match command {
        "build" => Operation::Buid,
        "run" => Operation::Run,
        _ => Operation::Unknown,
    }
}

pub fn check_file_ext(file_name: &str, extenssion: &str) -> FileResult<bool> {
    match file_name.split('.').collect::<Vec<&str>>().get(1) {
        Some(filename) => Ok(filename.eq(&extenssion)),
        None => Err(FileError::FetchFile),
    }
}

pub fn create_file(file_name: &str) -> FileResult<String> {
    match file_name.split('.').collect::<Vec<&str>>().get(0) {
        Some(filenmame) => Ok(format!("{}.alaybeyvm", filenmame)),
        None => Err(FileError::CreateFile),
    }
}

pub fn load_text_from_file(file_name: &str, language_text: &mut String) -> FileResult<usize> {
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => return Err(FileError::OpenFile),
    };

    read_from_file_to_str(file, language_text)
}

fn read_from_file_to_str(mut file: File, mut language_text: &mut String) -> FileResult<usize> {
    let size = match file.read_to_string(&mut language_text) {
        Ok(size) => size,
        Err(_) => return Err(FileError::ReadFile),
    };
    Ok(size)
}
