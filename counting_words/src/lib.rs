use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
 let  mut map= HashMap::new();
    for word in words.split_whitespace(){
        let mut s= String::new();
        for (i,c) in word.to_lowercase().chars().enumerate(){
            if c>='a'&& c<='z'{
                s.push(c)
            }else if c>='0'&& c<='9'{
                s.push(c)
            }else if c == '\''&& i<word.len()-1&& i>0{
                s.push(c)
            }
        }
        if !s.is_empty(){
            *map.entry(s).or_insert(0)+=1;
        } 
    }
    println!("*****{:?}",map);
    map
}