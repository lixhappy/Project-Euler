fn main() {
    let mut _ans_sum : u32 = 0;
    for i in 1..1000
    {
        if i%3==0 || i%5==0
            {_ans_sum += i;}
    }
    println!("Answer sum = {}",_ans_sum);
}
