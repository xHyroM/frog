use std::{fs, path::Path};

pub mod config;
pub mod eval;

pub struct FrogCore;

impl FrogCore {
    pub fn init(name: String, directory: &String, language: String) -> bool {
        if directory != "." {
            match fs::create_dir_all(directory) {
                Ok(_) => (),
                Err(_) => return false,
            };
        }

        let path = Path::new(directory);

        let code = String::from(
            format!("declare name = \"{}\";
declare language = \"{}\";

task build() {{
    print(\"Building...\");
}}", name, language)
        );

        println!("{}", code);
        
        match fs::write(path.join("uwu.frog"), code) {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
