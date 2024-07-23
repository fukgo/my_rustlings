fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    //[T; N]表示一个包含N个T类型元素的数组
    let a = [0; 100];
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
