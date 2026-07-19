pub fn insertion_sort(slice: &mut [i32], steps: usize) {
   for i in 1..=steps{
    let k = slice[i];
    let mut j = i;
    while j>0 && slice[j-1]>k{
        slice[j]=slice[j-1];
        j-=1;
    }
    slice[j]=k;
   }
}