#![allow(unused_variables)]
//! 第一章：Rust语言基础
//! 1.4 语法面面观（二）：面向表达式（上）
//! 
//! 
//!    

/**

    ### 面向表达式 (Expression-Oriented programming)

    ```
    use std::collections::HashMap;

    fn add_one(i: &mut u32) {
        *i += 1;
    }

    fn plus_one(i: &u32) -> u32 {
        let i = i + 1;
        i
    }

    fn main() {
        let mut a = 41 ;
        add_one(&mut a) ;
        println!("{:?}", a) ;
        
        let a = 41;
        let b = plus_one(&a);
        println!("{:?}", b) ;
        
        let mut h = HashMap::new();
        h.insert("anwser", 42);
        println!("anwser is {:?}", h["anwser"]);
    }
    ```
*/
pub fn eop(){ 
    println!("1.4 : Expression-Oriented programming");
}


/**

    ### 分号表达式 vs 块表达式

    1. 分号表达式返回值永远为自身的单元(Unit)类型：`()`
    2. 分号表达式只有在块表达式最后一行才会进行求值，其他时候只作为「连接符」存在
    3. 块表达式只对其最后一行表达式进行求值。

    ```
    fn main(){
        ; 
        ;
        {
            ()
        }
        {
            ();
            use std::vec::Vec;
        }
        ();
        &();
        &{;}; // -> &()
        ; // ->  ()
    }
    ```
*/
pub fn semi_and_block_expr(){ 
    println!("1.4 : Semi vs Block ");
}


/**

    ### FizzBuzz in match 

    ```
    fn main() {
        for i in 1..102 {
            match (i%3, i%5) {
                (0, 0) => println!("FizzBuzz"),
                (0, _) => println!("Fizz"),
                (_, 0) => println!("Buzz"),
                (_, _) => println!("{}", i)
            }
        }
    }   
    ```
*/
pub fn fizzbuzz_match(){ 
    println!(" 1.4 : FizzBuzz in match ");
}


/**

    ### FizzBuzz in if 

    ```
    fn main() {
        for i in 1..102 {
            if i % 15 == 0 { println!("FizzBuzz") }
            else if i % 3 == 0 { println!("Fizz") }
            else if i % 5 == 0 { println!("Buzz") }
            else { println!("{}", i) }
        }
    }

    ```

*/
pub fn fizzbuzz_if(){ 
    println!(" 1.4 : FizzBuzz in if ");
}
