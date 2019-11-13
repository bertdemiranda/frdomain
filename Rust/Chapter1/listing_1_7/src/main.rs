fn main() {

    let inc1   = |n: i32| n + 1;
    let square = |n: i32| n * n;

    let seq1 : Vec<i32> = (1..=10).map(inc1).collect();
    println!("seq1 = {:?}", seq1);

    let seq2 : Vec<i32> = (1..=10).map(square).collect();
    println!("seq2 = {:?}", seq2);

    let inc1_square = |n : i32| square(inc1(n));
    println!("inc1_square(4) = {:?}", inc1_square(4));

    let square_inc1 = |n : i32| inc1(square(n));
    println!("square_inc1(4) = {:?}", square_inc1(4));    
}
