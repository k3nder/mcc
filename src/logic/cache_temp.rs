use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct Cache {
    file: String,
    dir: String,
}
impl Cache {
    pub fn new(file: String, dir: String) -> Cache {
        Cache { file, dir }
    }
    pub fn put(self, value: String) {
        // 1. If file or dir not exist, create it
        let filename: String = self.file;
        let dir: String = self.dir;
        let full_filepath: String = format!("{}/{}.cache", dir, filename);
        let _pf = Path::new(&full_filepath);
        let _pp = Path::new(&dir);
        if !_pp.exists() {
            fs::create_dir(_pp).expect("Cannot create the dir")
        }
        if !_pf.exists() {
            File::create(&full_filepath).expect("Cannot create the file");
        }
        // 2. Write in file the value
        fs::write(full_filepath.clone(), value)
            .expect(format!("Cannot write in the file: {}", full_filepath).as_str());
    }
    pub fn get(self) -> Option<String> {
        // 1. Return (None) if file no exist
        let filename: String = self.file;
        let dir: String = self.dir;
        let full_filepath: String = format!("{}/{}.cache", dir, filename);
        let _p = Path::new(&full_filepath);
        if !_p.exists() {
            return None;
        }
        // 2. Open the file
        let mut file: File = File::open(full_filepath).expect("Cannot open the file");
        // 3. Read the file
        let mut content: String = "".to_string();
        file.read_to_string(&mut content)
            .expect("Cannot read the file");
        // 4. Return the value in Some(String)
        Some(content)
    }
}
