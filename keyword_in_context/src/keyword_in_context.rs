use crate::proc_args;
use crate::read_phrase_files;
use crate::read_stop_word_files;
use crate::proc_phrases;
use crate::sort;


/// # Parameters
/// - `args`: A String vector that contains user's arguments
///
/// # Returns
/// - Result<bool,String>, where bool will have value of true if -h or --help was passed as an argument
///


pub fn execute(args:Vec<String>)-> Result<bool,String>{
    let mut phrases_file: String = "./files/sentences.txt".to_string();          // Default sentences txt
    let mut stop_words_file: String = "./files/stop_words.txt".to_string();      // Default stop_words txt
    let mut context_len: Option<i32> = None;        // None if the whole sentence is used, Some(n) if user only wants n words around the word in context 
    let mut sort_type: String = "alpha".to_string();    // Better explained in sort

    let help = proc_args::process_args(&args,&mut phrases_file, &mut stop_words_file, &mut context_len, &mut sort_type);
    if help?{
        return Ok(true) // Stop when asking for help (this is not an error)
    }

    let phrases_vector = read_phrase_files::get_phrases(phrases_file.clone())?;   // Gets sentences from file
    let stop_words_hash = read_stop_word_files::get_stop_words(stop_words_file.clone())?;    // Gets stop words from file

    // Gets all non-stop-words phrases in context   
    let mut phrases_in_context = Vec::<String>::new();
    for phrase in phrases_vector{
        let resulting_phrases = proc_phrases::processar_frase(phrase, &stop_words_hash,context_len)?;
        phrases_in_context.extend(resulting_phrases);
    }

    sort::sort_phrases(&mut phrases_in_context,&sort_type)?;   
    println!("The sentences from {} generated the following sentences in contex using stop words from {}:\n{}\n", phrases_file, stop_words_file, phrases_in_context.join("\n"));
    Ok(false)
}