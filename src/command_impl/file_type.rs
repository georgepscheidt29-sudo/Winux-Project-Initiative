use std::fs::Metadata;

pub(crate) enum FileType {
    File,
    Directory,
    SymLink,
    Other
}

pub(crate) fn match_file_type(file_metadata: &Metadata) -> FileType {
    if file_metadata.is_file() {
        FileType::File
    } else if file_metadata.is_dir() {
        FileType::Directory
    } else if file_metadata.is_symlink() {
        FileType::SymLink
    } else {
        FileType::Other
    }
}

impl FileType {
    pub(crate) fn to_string(&self) -> String {
        match self {
            FileType::File => "File".to_string(),
            FileType::Directory => "Dir".to_string(),
            FileType::SymLink => "SymLink".to_string(),
            FileType::Other => "Other".to_string()
            
        }
    }
}