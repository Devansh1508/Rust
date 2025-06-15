fn main() {
    println!("Hello, world!");
    let ans:bool = is_even(4);
    if ans {
        println!("yes it is a even number");
    }
    else {
        println!("no it is not a even number");
    }
}

fn is_even(num:i32)->bool{
    return if num%2==0{true} else {false};
}
