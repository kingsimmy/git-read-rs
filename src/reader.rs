use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use libflate::zlib::Decoder;

pub struct Reader {
    pub repository_path: String
}

impl Reader {
    pub fn read_loose_file(&self, hash: &str) -> String {
        let folder_name = &hash[..2];
        let file_name = &hash[2..];

        let mut path = PathBuf::from(&self.repository_path);
        path.push(".git".to_string());
        path.push("objects".to_string());
        path.push(folder_name);
        path.push(file_name);

        println!("Reading {:?}", path);
        let mut file = File::open(path).unwrap();

        let mut buffer = Vec::new();
        let file_read_res = file.read_to_end(&mut buffer);
        if let Ok(bytes_read) = file_read_res {
            println!("Read {} bytes", bytes_read);
        };

        let mut decoder = Decoder::new(&buffer[..]).unwrap();
        let mut decoded_bytes = Vec::new();
        decoder.read_to_end(&mut decoded_bytes).unwrap();

        let decoded_string = String::from_utf8(decoded_bytes).expect("Found invalid UTF-8");        
        
        let chars: Vec<char> = decoded_string.chars().collect();
        let mut i = 0;

        let mut git_file_type = String::new();
        while chars[i] != ' ' { //Single space comes after file type
            git_file_type.push(chars[i]);
            i = i + 1;
        }

        let mut length_string = String::new();
        while chars[i] != '\u{0}' { //Null char comes after length
            length_string.push(chars[i]);
            i = i + 1;
        }
        i = i + 1; //Want to skip null char
        decoded_string.chars().skip(i).collect()
    }
}