/*
##########   Keyword In Context (Grupo 01)   ########## 
#         Estilo: Passive Aggressive -- Rust          #
#  -------------------------------------------------  #
#        Eduardo Pereira de Sousa -- 231018937        # 
#       João Henrique Jacomo Lemes -- 231018893       #
#        José Neto Souza de Lima -- 231018955         #
#         Luca Heringer Megiorin -- 231003390         #
#      Pedro Henrique Silva de Sousa -- 222001411     #
#                                                     #
#  -------------------------------------------------  #
#      Para rodar o código com argumentos digite      #
#       `cargo run args` seguido dos argumentos       #
#   (use -h ou --help para ver lista dos argumentos)  #
#                                                     #
#######################################################
*/

use keyword_in_context::keyword_in_context::execute;

use std::env;
fn main() {
    println!("Running code.\n");
    let args: Vec<String> = env::args().collect();
    match execute(args) {
        Ok(true) => println!("When ready, run the code as 'cargo run' with 'args' followed by arguments, if necessary.\n"),
        Ok(false) => println!("Program finished sucessfully\n
+ To run with arguments, type 'cargo run args' followed by arguments.
+ For further help, type 'cargo run args -h' or 'cargo run args --help'.\n"),
        Err(s) => println!("Program terminated by error: {}\n
+ To run with arguments, type 'cargo run args' followed by arguments.
+ For further help, type 'cargo run args -h' or 'cargo run args --help'.\n",s),
    };
}
