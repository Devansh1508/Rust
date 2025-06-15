fn fib(num: u32)-> u64{
    let mut a=0;
    let mut b=1;
    
    if num==0 { return a; }
    if num==1 {return 1;}

    for i in 1..num-1{
        let temp=b;
        b=b+a;
        a=temp;
    }

    return b;
}

