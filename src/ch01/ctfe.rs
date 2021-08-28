#![allow(unused_variables)]
//! 第一章：Rust语言基础
//! 1.4 语法面面观（二）：面向表达式（中）
//!
//!
//!    

/**
    ### 必须是常量表达式才能在常量上下文使用

    ```
    fn main(){
        let an = (42,).0;
        const AN: i32 = an; // Error: attempt to use a non-constant value in a constant
    }
    ```
*/
pub fn must_const_expr() {
    println!(" 1.4 : Must Const erpr in Const Context ");
}

/**

    ### const fn


    ```
    const fn len() -> usize { 3 }

    fn main(){
        // 数组长度是常量上下文
        let array: [i32; len()] = [1, 2, 3];
    }
    ```

*/
pub fn const_array_len() {
    println!("1.4 : Expression-Oriented programming: const array len");
}

/**
    ### const fn : fib

    ```
    const fn gcd(a: u32, b: u32) -> u32 {
        match (a, b) {
            (x, 0) | (0, x) => x,

            (x, y) if x % 2 == 0 && y % 2 == 0 => 2*gcd(x/2, y/2),
            (x, y) | (y, x) if x % 2 == 0 => gcd(x/2, y),

            (x, y) if x < y => gcd((y-x)/2, x),
            (x, y) => gcd((x-y)/2, y),
        }
    }


    const fn fib(n: u128) -> u128 {
        const fn helper(n: u128, a: u128, b: u128, i: u128) -> u128 {
            if i <= n {
                helper(n, b, a + b, i + 1)
            } else {
                b
            }
        }
        helper(n, 1, 1, 2)
    }

    const X: u128 = fib(10);
    const GCD: u32 = gcd(21, 7);

    fn main(){
        println!("{}", X);
        println!("{}", GCD);
    }

    ```

*/
pub fn const_fib() {
    println!("1.4 : Expression-Oriented programming: const fib");
}

/**

    ### const fn

    ```
    const UNIT_TUPLE: [(u64, &str); 6] = {
        let mut i = 0;
        [
            (1 << (10 * { i += 1; i }), "KiB"),
            (1 << (10 * { i += 1; i }), "MiB"),
            (1 << (10 * { i += 1; i }), "GiB"),
            (1 << (10 * { i += 1; i }), "TiB"),
            (1 << (10 * { i += 1; i }), "PiB"),
            (1 << (10 * { i += 1; i }), "EiB")
        ]
    };

    const fn square_area(a: i32) -> i32 {
        let area = a * a;
        area
    }

    const AREA: i32 = square_area(5);

    fn main (){
        dbg!(UNIT_TUPLE);
        dbg!(AREA);
    }

    ```
*/
pub fn const_fn_() {
    println!(" 1.4 : Const fn ");
}

/**

    ### 展示错误的 const 求值用法

    ```
    #![feature(const_fn)]

    // Error
    const fn hello() -> String{
        "Hello".to_string()
    }

    // Error
    const S : String = hello();

    fn main(){
        println!(" {:?} ", S);
    }

    ```
*/
pub fn const_fn_error() {
    println!(" 1.4 : Const fn Error ");
}

/**
    ### 修正错误的 const 求值用法

    ```

    const fn hello() -> &'static str{
        "Hello"
    }

    const Y: &str = hello();

    fn main(){
        println!("{}", Y);
    }

    ```
*/
pub fn fixed_const_fn_error() {
    println!(" 1.4 : Fixed Const fn Error ");
}

/**

    ### 其他的Const fn 用法

    ```
    #[derive(Debug)]
    struct Answer(u32);
    const A: Answer = Answer(42);

    fn main(){
        println!("{}", A);
    }

    ```
*/
pub fn others_const_fn() {
    println!(" 1.4 : Others Const fn  ");
}

/**

    ### 编译期计算原理：MIR 展示
    ```
    const fn anwser() -> u32 { 42 }

    const A: u32 = anwser();

    fn main(){
        A;
    }
    ```

*/
pub fn mir_show() {
    println!(" 1.4 : MIR show ");
}

/**

    ### If True && While True

    ```
    fn if_true(x: i32) -> i32 {
        if true {  // error[E0308]: mismatched types，expected type `i32` found type `()`
            return x+1;
        }
    }

    fn while_true(x: i32) -> i32 {
        while true {  // error[E0308]: mismatched types，expected type `i32` found type `()`
            return x+1;
        }
    }

    fn main() {
        let y = while_true(5);
        assert_eq!(y, 6);

        let y = if_true(5);
        assert_eq!(y, 6);

        let x;
        // while true { x = 1; break; }
        loop { x = 1; break; }
        println!("{}", x);

    }
    ```

*/
pub fn if_while_true() {
    println!(" 1.4 : If True && While True")
}

/**
    ### const generic

    ```
    #![feature(min_const_generics)]
    #![feature(const_in_array_repeat_expressions)]

    use core::mem::MaybeUninit;

    #[derive(Debug)]
    pub struct ArrayVec<T, const N: usize> {
        items: [MaybeUninit<T>; N],
        length: usize,
    }

    impl<T, const N: usize> ArrayVec<T,  {N} > {
        pub const fn new() -> ArrayVec<T, {N} > {
            ArrayVec {
                items: [MaybeUninit::uninit(); N],
                length: 0,
            }
        }

        #[inline]
        pub const fn len(&self) -> usize { self.length }

        #[inline]
        pub const fn is_empty(&self) -> bool { self.len() == 0 }

        #[inline]
        pub const fn capacity(&self) -> usize { N }

        #[inline]
        pub const fn is_full(&self) -> bool { self.len() >= self.capacity() }

    }

    impl<T, const N: usize> Drop for ArrayVec<T, { N }> {
        #[inline]
        fn drop(&mut self) {
            // Makes sure the destructors for all items are run.
            // self.clear();
        }
    }


    fn main(){
        // let mut vector = ArrayVec::new();
        // println!("{}, {}", vector.len(), vector.capacity());
        // println!("{:?}", vector);
    }
    ```
*/
pub fn const_generic_show() {
    println!(" 1.4 : Const Generic Show")
}

/**

    ### array chunk 演示

    ```
    #![feature(array_chunks)]
    fn main() {
        let data = [1, 2, 3, 4, 5, 6];
        let sum1 = data.array_chunks().map(|&[x, y]| x * y).sum::<i32>();
        assert_eq!(sum1, (1 * 2) + (3 * 4) + (5 * 6));

        let sum2 = data.array_chunks().map(|&[x, y, z]| x * y * z).sum::<i32>();
        assert_eq!(sum2, (1 * 2 * 3) + (4 * 5 * 6));
    }

    ```
*/
pub fn array_chunk_show() {
    println!(" 1.4 : Array Chunks Show")
}
