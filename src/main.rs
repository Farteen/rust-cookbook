extern crate rayon;
use rayon::prelude::*;

fn main() {
    let legend = "Did you ever hear the tragedy of Darth Plague is The Wise?";
    let words: Vec<_> = legend.split_whitespace().collect();

    words.par_iter().for_each(|val| println!("{}", val));

    let words_with_a: Vec<_> = words
    .par_iter()
    .filter(|val| val.find('a').is_some())
    .collect();
    println!("The following words contain the letter 'a': {:?}", words_with_a);
}