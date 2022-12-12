use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(crate) fn read_file(path_to_file: &str) -> String {
    let path = Path::new(path_to_file);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Err(why) => panic!("couldnt read {}: {}", display, why),
        Ok(_) => return text,
    }
}
