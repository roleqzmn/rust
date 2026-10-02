use std::io;
use std::fs::File;
use std::io::Write;

fn power_array(x: u64) -> [u64; 10] {
    let mut result =[0u64; 10];
    result[0] = 1;
    for i in 1..10 {
        result[i] = result[i-1] * x;
    }
    return result;
}

fn if_collatz_holds(arr: [u64; 10]) -> [bool; 10] {
    let mut result = [true; 10];
    for i in 0..10 {
        let mut tmp = arr[i];
        for _ in 0..100 {
            if tmp == 1{
                break;
            }
            if tmp % 2 == 0 {
                tmp=tmp/2;
            }
            else {
                tmp = 3 * tmp + 1;
            }
        }
        if tmp != 1 {
            result[i] = false;
        }
    }
    return result;
}

fn tuple_return(x: u64) -> (u64, [u64; 10], [bool; 10]) {
    let arr = power_array(x);
    let res = if_collatz_holds(arr);
    return (x, arr, res);
}

fn main() {
    let result = loop{
        println!("Enter a number (0 to quit):");
        let mut input = String::new();

        match io::stdin().read_line(&mut input){
            Ok(length) =>{println!("Read {} bytes", length);},
            Err(_) => break true
        };

        let mut x: u64 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => break true
        };

        if x == 0{
            break false;
        }
        println!("x = {}", x);

        x+=rand::random::<u64>()%6;
        println!("x = {}", x);

        let arr = power_array(x);
        let res = if_collatz_holds(arr);

        let mut file = File::create("xyz.txt").unwrap();

        write!(file, "{:?}", res).expect("Could not write to file");
    };
    tuple_return(1);
    if result{
        println!("Ended cuz of an error");
    }
    else{
        println!("Ended cuz u wanted");
    }
    return;
}
