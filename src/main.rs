mod game;
mod wordlist;

use wordlist::generate_word_list;

fn main() {
    println!("Welcome to Word Scramble Game!");
    let word = generate_word_list();
    game::play_game(word);
}
