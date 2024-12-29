pub mod keyword_in_context;
pub mod proc_args;
pub mod read_phrase_files;
//pub mod read_stop_word_files;
pub mod proc_phrases;
pub mod circ_rot;
pub mod sort;

use std::env;
fn main() {
    println!("Running code.\n");
    let args: Vec<String> = env::args().collect();
    match keyword_in_context::execute(args) {
        Ok(true) => println!("When ready, run the code as 'cargo run' with 'args' followed by arguments, if necessary.\n"),
        Ok(false) => println!("Program finished sucessfully\n
+ To run with arguments, type 'cargo run args' followed by arguments.
+ For further help, type 'cargo run args -h' or 'cargo run args --help'.\n"),
        Err(s) => println!("Program terminated by error: {}\n
+ To run with arguments, type 'cargo run args' followed by arguments.
+ For further help, type 'cargo run args -h' or 'cargo run args --help'.\n",s),
    };
}
