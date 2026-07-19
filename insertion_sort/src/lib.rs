pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    let n = slice.len();
    let mut l = steps;
    if n>=1{
        l=steps.min(n);
    }
    for i in 1..=l{
        let key = slice[i];
        let mut j=i;
        while j > 0 && slice[j-1]>key{
            slice[j]=slice[j-1];
            j-=1;
        }
        slice[j]=key;
    }
}
