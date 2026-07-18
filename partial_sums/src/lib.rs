pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut res = Vec::new();
    let mut sum :u64 = arr.iter().sum();
    res.push(sum); 

    for &x in arr.into_iter().rev(){
        sum -=x;
        res.push(sum); 
    }
    res
}