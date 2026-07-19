pub fn rot21(input: &str) -> String {
    input.chars().map(|c|{
        if c.is_ascii_lowercase(){
            (((c as u8 -b'a'+21)%26)+b'a')as char
        }else if c.is_ascii_uppercase(){
            (((c as u8 -b'A'+21)%26)+b'A')as char
        }else{
            c
        }
    }).collect()
}