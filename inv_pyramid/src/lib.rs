pub fn inv_pyramid(v: String, i: usize) -> Vec<String> {
    let mut res:Vec<String>=Vec::new();

    for x in 1..=i{
        res.push(format!("{}{}"," ".repeat(x),v.repeat(x)));
    }
    for x in (1..i).rev(){
        res.push(format!("{}{}"," ".repeat(x),v.repeat(x)));
    }

    res
}
