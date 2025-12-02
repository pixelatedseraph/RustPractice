use std::collections::HashMap;

use crate::hashmap::is_anagram;

mod strings;
mod hashmap;
fn main() {
    /* let mut map = HashMap::new();
    map.insert("ace",1 );
    map.entry("apple").or_insert(0);
    if let Some(V) = map.get("apple"){
        println!("{}",V);
    }
    // *map.entry("apple").or_insert(0)+=1;
    println!("{:?}",map.get("apple"));
    for (k,v) in &map {
        println!("{k}:{v}");
    } */
  /*  let string = "banana";
   hashmap::frequency_c(string);
   println!(" ");
   let string2 = "hello world rust world is rusty";
   hashmap::frequency_w(string2);
   let most_rep = hashmap::frequency_w_highest(string).unwrap_or(' ');
   println!("{}",most_rep); */
   println!("{:?}",is_anagram("hello", "olleh"));
   println!("{:?}",is_anagram("hii", "olleh"));

}
