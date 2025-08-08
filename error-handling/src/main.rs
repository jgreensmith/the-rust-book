use errorhandling;

fn main() {

    // let ahoy = errorhandling::open_file()
    //     .unwrap_or_else(|error| {
    //         panic!("Bruh where your file? {error:?}");
    //     });
    errorhandling::open_or_create();

    // Expect causes panic with a custom message
    // errorhandling::open_file()
    //     .expect("There should probably be a file here");

    let username = errorhandling::proper_file().unwrap();
    println!("{}", username);
}
