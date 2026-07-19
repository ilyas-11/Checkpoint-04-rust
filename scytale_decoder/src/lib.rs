pub fn scytale_decoder(s: String, ll: usize) -> Option<String> {
    if s.is_empty()||ll==0{
       return None;
    }
    let chars:Vec<char>=s.chars().collect();
    let len=chars.len();

    let mut coll=len/ll;
    if len%ll>0{
        coll+=1;
    }

    let mut index= 0;
    let mut map=vec![vec!['\0';coll];ll];
    for col in 0..coll{
        for row in 0..ll{
            if index<len{
                map[row][col]=chars[index];
                index+=1;
            }
        }
    }
    let mut str=String::new();
    for i in 0..ll{
        for j in 0..coll{
            if map[i][j]!='\0'{
                str.push(map[i][j]);
            }
        }
    }
    Some(str)
}
