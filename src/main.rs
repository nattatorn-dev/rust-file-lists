use std::fs;
use std::path::Path;

fn list_files(path: &Path) -> () {
    match fs::read_dir(path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }
}

fn main() {
    let mut path_url_input = String::new();
    println!("Enter Folder Path (default current path):");

    std::io::stdin().read_line(&mut path_url_input).unwrap();

    let mut path_url = path_url_input.trim().to_string();

    let path_is_empty = path_url.is_empty();
    if path_is_empty == true {
        path_url = ".".to_string();
    }

    let path = &Path::new(&path_url);
    list_files(path);
}
