use std::{collections::HashMap, hash::Hash};


pub fn frequency_c(word: &str) {
    let mut map:HashMap<char,i32> = HashMap::new();
    for ch in word.chars(){
        *map.entry(ch).or_insert(0) +=1;
    }
    println!("{{");
    for (ch, count) in &map{
        println!(" '{}' :  {}",ch,count);
    }
    println!("}}");
}


pub fn frequency_w (string : &str) {
    let mut map:HashMap<&str,i32> = HashMap::new();
    for s in string.split(' '){
        *map.entry(s).or_insert(0) +=1;
    }
    println!("{{");
    for (k,v) in &map {
        println!("{} : {}",k,v);
    } 
    println!("}}");
}

pub fn frequency_w_highest (word : &str) -> Option<char> {
    let mut map:HashMap<char,i32> = HashMap::new();
    for ch in word.chars(){
        *map.entry(ch).or_insert(0) +=1;
    }
    map.iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k,_)| *k)
}


pub fn is_anagram (word1 : &str, word2 : &str) -> bool {
    let mut map1:HashMap<char,i32> = HashMap::new();
    let mut map2:HashMap<char,i32> = HashMap::new();
    for ch in word1.chars(){
        *map1.entry(ch).or_insert(0) +=1 ;
    }
    for ch in word2.chars(){
        *map2.entry(ch).or_insert(0) +=1 ;
    }
    if map1 == map2 {
        return true;
    }
    false
}

pub fn first_non_repeating_w (word : &str) -> Option<char> {
    let mut map: HashMap<char, i32> = HashMap::new();
    for ch in word.chars(){
        *map.entry(ch).or_insert(0) +=1;
    }
    for ch in word.chars(){
        if map[&ch] == 1{
            return Some(ch);
        }
    }
    None
}