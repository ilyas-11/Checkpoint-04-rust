pub fn prev_prime(nbr: u64) -> u64  {
    if nbr <=2{
        return 0;
    }
    let mut n =nbr-1;
    while n>=2{
        if is_prime(n){
            return n;
        }
        n-=1
    }
    0

}
fn is_prime(n :u64)->bool{
    if n<2{
        return false ;
    }
    let mut i=2;
    while i*i<=n{
        if n%i==0{
            return false;
        }
        i+=1
    } 
    
    true
}