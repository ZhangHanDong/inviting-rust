use std::env;
use std::fs::File;
use std::io::prelude::*;


// cargo run ./src/sum_text
// Todo
fn main()  {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let mut f = File::open(filename).unwrap(); // todo 
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap(); // Todo
   let mut sum = 0;
   for c in contents.lines(){
       let n = c.parse::<i32>().unwrap(); // TODO
       sum += n;
   }
   println!("{:?}", sum);
} 