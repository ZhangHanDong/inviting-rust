//! 第二章：Rust核心概念
//! 1.2 生命周期与借用检查
//! 
//! 借用检查相关代码

/**
    ### 理解词法作用域

    基本数据类型： https://doc.rust-lang.org/std/index.html#primitives

    ```
    fn main(){
        let mut v = vec![];
        v.push(1);
        {
            println!("{:?}", v[0]);
            v.push(2);
        }
    }
    ```
*/
pub fn understand_scope(){ 
    println!(" 理解词法作用域 ");
}


/**
    ### 理解借用检查 NLL

    示例：替换字符串中的问号

    ```
    fn main(){
        let s = "abc?d";
        let mut chars = s.chars().collect::<Vec<char>>();
        
        // 处理字符串
        for (i, c) in chars.iter_mut().enumerate() {
            // 定义 a-z 字母集
            let mut words = ('a'..='z').into_iter();
            // 此处 `chars[i]` 是对chars的不可变借用
            if chars[i] == '?' {
                // 此处 `chars[i]` 是对chars的不可变借用
                let left = if i==0 {None} else { Some(chars[i-1]) };
                // 此处 `chars[i]` 是对chars的不可变借用
                let right = if i==s.len()-1 {None} else {Some(chars[i+1])};
                // 此处 `chars[i]` 是对chars的可变借用，要修改chars数组了
                // 从a-z 字母集中查找和左右两边不一样的字母去替换当前字符，避免重复
                chars[i] = words.find(|&w| Some(w) != left && Some(w) != right).unwrap();
            }
        }
        
        let s = chars.into_iter().collect::<String>();
        println!("{:?}", s);
    }
    ```
*/
pub fn understand_nll(){ 
    println!(" 理解 非词法作用域借用检查： NLL ");
}


/**

    理解普通生命周期参数：

    示例1: 

    ```rust
    fn return_str<'a>() -> &'a str {
        let mut s = "Rust".to_string();
        for i in 0..3 {
            s.push_str("Good ");
        }
        &s[..]                   //"Rust Good Good Good"
    }
    fn main() {
        let x = return_str();
    }

    ```

    示例2: 

    ```rust
    fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
        let result = String::from("really long string");
        // error
        result.as_str()
    }

    fn main() {
        let x = "hello";
        let y = "rust";
        foo(x, y);
    }
    ```

    示例3: 

    ```rust
    fn the_longest(s1: &str, s2: &str) -> &str {
        if s1.len() > s2.len() { s1 } else { s2 }
    }
    fn main() {
        let s1 = String::from("Rust");
        let s1_r = &s1;
        {
            let s2 = String::from("C");
            let res = the_longest(s1_r, &s2);
        println!("{} is the longest", res);
    }
    
    ```

    示例4: 

    ```rust
    fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() { s1 } else { s2}
    }
    fn main() {
        let s1 = String::from("Rust");
        let s1_r = &s1;
        {
            let s2 = String::from("C");
            let res = the_longest(s1_r, &s2);
        println!("{} is the longest", res); // Rust is the longest
    }

    ```
*/
pub fn understand_lifetime(){ 
    println!(" 理解 生命周期参数 ");
}



/**

说明： 生命周期参数：early bound vs late bound

Quiz 11: https://dtolnay.github.io/rust-quiz/11


示例一：late bound lifetime

```rust
struct Buffer {
    buf: Vec<u8>,
    pos: usize,
}

impl Buffer {
    fn new() -> Buffer {
        Buffer {
            buf: vec![1,2,3, 4, 5,6],
            pos: 0,
        }
    }

    fn read_bytes<'a>(&'a mut self) -> &'a [u8] {
        self.pos += 3;
        &self.buf[self.pos-3..self.pos]
    }
}

fn print(b1 :&[u8], b2: &[u8]) {
    println!("{:#?} {:#?}", b1, b2)
}

fn main() {
    let mut buf = Buffer::new();
    // let b1 = buf.read_bytes(); // don't work
    let b1 = &(buf.read_bytes().to_owned());
    let b2 = buf.read_bytes();
    print(b1,b2)
}
```

示例三：

```rust
fn main() {
    let v = vec![1,2,3, 4, 5,6];
    let mut buf = Buffer::new(&v);
    let b1 = buf.read_bytes();
    // let b1 = &buf.read_bytes().to_owned();
    let b2 = buf.read_bytes();
    print(b1,b2)
}

fn print(b1 :&[u8], b2: &[u8]) {
    println!("{:#?} {:#?}", b1, b2)
}

struct Buffer<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Buffer<'a> {
    fn new(b: &'a [u8]) -> Buffer {
        Buffer {
            buf: b,
            pos: 0,
        }
    }

    fn read_bytes(&mut self) -> &'a [u8] {
        self.pos += 3;
        &self.buf[self.pos-3..self.pos]
    }
}
```
*/
pub fn understand_lifetime_early_late_bound(){ 
    println!(" 理解生命周期参数：early bound vs late bound ");
}


/**

    ### T vs &T

    ```rust
    use std::fmt::Debug;

    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);

    fn print<T>(t: T)
    where
        T: Debug,
    {
        println!("`print`: t is {:?}", t);
    }

    fn print_ref<'a, T>(t: &'a T)
    where
    T: Debug + 'a,
    {
    println!("`print_ref`: t is {:?}", t);
    }

    fn main() {
        let x = 7;
        let ref_x = Ref(&x);
        print_ref(&ref_x);
        print(ref_x);
    }
    ```

    示例：Rust Quiz 5 ：https://zhuanlan.zhihu.com/p/51616607

    以下代码输出什么？

    ```rust
    trait Trait {
        fn f(self);
    }

    impl<T> Trait for fn(T) {
        fn f(self) {
            print!("1");
        }
    }

    impl<T> Trait for fn(&T) {
        fn f(self) {
            print!("2");
        }
    }

    fn main() {
        let a: fn(_) = |_: u8| {};
        let b: fn(_) = |_: &u8| {};
        let c: fn(&_) = |_: &u8| {};
        a.f();
        b.f();
        c.f();
    }
    ```

    
    示例： trait对象中的生命周期参数

    ```rust
    trait Foo<'a> {}
    struct FooImpl<'a> {
        s: &'a [u32],
    }
    impl<'a> Foo<'a> for FooImpl<'a> {
    }
    // 为 trait对象 增加 'a ，因为 Box 默认是 static 的，而FooImpl 中的 s 则是引用
    // 表明该trait对象（结构体实例）与其结构体中的引用的生命周期是一样长的（<=）
    fn foo<'a>(s: &'a [u32]) -> Box<dyn Foo<'a> + 'a> {
        Box::new(FooImpl { s: s })
    }
    fn main(){}
    ```
*/


/**
    ### HRTB (higher ranked trait bounds)

    示例一：

    ```rust
    use std::fmt::Debug;
    trait DoSomething<T> {
        fn do_sth(&self, value: T);
    }
    impl<'a, T: Debug> DoSomething<T> for &'a usize {
        fn do_sth(&self, value: T) {
            println!("{:?}", value);
        }
    }
    fn foo<'a>(b: Box<DoSomething<&'a usize>>) {
        let s: usize = 10;
        b.do_sth(&s) // error[E0597]: `s` does not live long enough
    }
    fn main(){
        let x  = Box::new(&2usize);
        foo(x);
    }
    ```

    修正：

    ```rust
    use std::fmt::Debug;
    trait DoSomething<T> {
        fn do_sth(&self, value: T);
    }
    impl<'a, T: Debug> DoSomething<T> for &'a usize {
        fn do_sth(&self, value: T) {
            println!("{:?}", value);
        }
    }
    fn bar(b: Box<for<'f> DoSomething<&'f usize>>) {
        let s: usize = 10;
        b.do_sth(&s);
    }
    fn main(){
        let x  = Box::new(&2usize);
        bar(x);
    }
    ```

    示例 2:

    ```rust
    use rand;
    use std::io::Read;

    trait Checksum<R: Read> {
        fn calc(&mut self, r: R) -> Vec<u8>;
    }

    struct Xor;

    impl<R: Read> Checksum<R> for Xor {
        fn calc(&mut self, mut r: R) -> Vec<u8> {
            let mut res: u8 = 0;
            let mut buf = [0u8; 8];
            loop {
                let read = r.read(&mut buf).unwrap();
                if read == 0 {
                    break;
                }
                for b in &buf[..read] {
                    res ^= b;
                }
            }

            vec![res]
        }
    }

    struct Add;

    impl<R: Read> Checksum<R> for Add {
        fn calc(&mut self, mut r: R) -> Vec<u8> {
            let mut res: u8 = 0;
            let mut buf = [0u8; 8];
            loop {
                let read = r.read(&mut buf).unwrap();
                if read == 0 {
                    break;
                }
                for b in &buf[..read] {
                    let tmp = res as u16 + *b as u16;
                    res = tmp as u8;
                }
            }

            vec![res]
        }
    }

    fn main() {
        let mut buf = [0u8; 8];
        let mut checker: Box<dyn for<'a> Checksum<&'a [u8]>> = if rand::random() {
            println!("Initializing Xor Checksum");
            Box::new(Xor)
        } else {
            println!("Initializing Add Checksum");
            Box::new(Add)
        };

        let mut data = "Sedm lumpu slohlo pumpu za uplnku".as_bytes();
        let mut i = 0;

        loop {
            let chunk_size = data.read(&mut buf).unwrap();
            if chunk_size == 0 {
                break;
            }
            let cs = checker.calc(&buf[..chunk_size]);
            println!("Checksum {} is {:?}", i, cs);
            i += 1;
        }
    }
    ```
*/
pub fn understand_lifetime_hrtb(){ 
    println!(" 理解生命周期参数：early bound vs late bound ");
}