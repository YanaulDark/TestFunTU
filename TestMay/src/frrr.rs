pub fn frrr() {
    println!("Yeah, I'm like cats too");

}

pub fn fac(n: u32) -> u32 {
    if n == 1{
        return 1
    } else {
        return n*fac(n-1)
    }
}
