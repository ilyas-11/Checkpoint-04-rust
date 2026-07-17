pub fn reverse_it(v: i32) -> String {
    let s= v.abs().to_string();
    let r:String=s.chars().rev().collect();
    if v<0{
        return format!("-{}{}",r,s);
    }
    format!("{}{}",r,s)
}
