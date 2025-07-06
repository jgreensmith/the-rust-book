mod actions;
use actions::read::download::download_dir;
// use crate::actions::write::update;
use crate::actions::write::create::File;

use crate::actions::write::delete;
fn main() {
    download_dir();
    let df = actions::read::download::download_file;
    df();
    // update::update_dir();
    delete::delete_file(); // this is the idiomatic approach
    let file: File = File {
        name: String::from("file_name123"),
        size: 23
    };
    
    println!("{:#?}", file);
}
