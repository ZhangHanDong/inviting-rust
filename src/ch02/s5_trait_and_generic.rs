//! 第二章：Rust核心概念
//! 2.4 trait 和 泛型
//!
//!

/**

   # 概念介绍

   ### trait 四种作用

   - 接口 (interface)
   - 类型标记（Mark)
   - 泛型限定（trait bound）
   - 抽象类型（trait object）


  ### 静态分发（单态化 - Monomorphized）

   ```rust
   use std::string::ToString;

   fn print<T: ToString>(v: T) {
       println!("{}", v.to_string());
   }
   ```

   或 `impl Trait`语法

   ```rust
   use std::string::ToString;

   #[inline(never)]
   fn print(v: &impl ToString) {
       println!("{}", v.to_string());
   }
   ```

   使用 `impl Trait` 解决问题：


   ```rust
   // error codes：

   use std::fmt::Display;

   fn main() {
       println!("{}", make_value(0));
       println!("{}", make_value(1));
   }

   fn make_value<T: Display>(index: usize) -> T {
       match index {
           0 => "Hello, World",
           1 => "Hello, World (1)",
           _ => panic!(),
       }
   }

   ```

   修正：

   ```
   use std::fmt::Display;

   fn make_value(index: usize) -> impl Display {
       match index {
           0 => "Hello, World",
           1 => "Hello, World (1)",
           _ => panic!(),
       }
   }

   ```

   `impl Trait` 生命周期相关：

   ```rust

   // error
   fn make_debug<T>(_: T) -> impl std::fmt::Debug + 'static{
       42u8
   }

   // fn make_debug<'a, T: 'static>(_: &'a T) -> impl std::fmt::Debug + 'static{
   //     42u8
   // }

   fn test() -> impl std::fmt::Debug {
       let value = "value".to_string();
       make_debug(&value)
   }
   ```

  实际案例 - 模版模式：[https://github.com/actix/actix-extras/tree/master/actix-web-httpauth](https://github.com/actix/actix-extras/tree/master/actix-web-httpauth)


  # trait 一致性

   trait 和 类型 必须有一个在本地。



*/
pub fn trait_concept() {
    println!("trait 概念")
}

/**


# 动态分发

    ### trait 对象

    用泛型模拟 Class

    ```rust

    #![allow(unused)]

    use core::any::{Any,TypeId};
    use std::sync::Arc;

    /// Class definition
    struct Class {
        /// The name of the class
        name: String,
        /// The corresponding Rust type
        type_id: TypeId,
    }

    impl Class {
        /// Create a new class definition for the type `T`
        fn new<T: 'static>() -> Self {
            Self {
                name: std::any::type_name::<T>().to_string(),
                type_id: TypeId::of::<T>(),
            }
        }
    }

    /// An instance of a class
    struct Instance {
        inner: Arc<dyn Any>, // `Arc` because we don't need/want mutability
    }

    impl Instance {
        /// Construct a new `Instance` from a type that
        /// implements `Any` (i.e. any sized type).
        fn new(obj: impl Any) -> Self {
            Self {
                inner: Arc::new(obj)
            }
        }
    }

    impl Instance {
        /// Check whether this is an instance of the provided class
        fn instance_of(&self, class: &Class) -> bool {
            // self.inner.type_id() == class.type_id
            self.inner.as_ref().type_id() == class.type_id
        }
    }

    struct Foo {}
    struct Bar {}

    fn main(){


        let foo_class: Class = Class::new::<Foo>();
        let bar_class: Class = Class::new::<Bar>();
        let foo_instance: Instance = Instance::new(Foo {});

        assert!(foo_instance.instance_of(&foo_class));
        assert!(!foo_instance.instance_of(&bar_class));
    }
    ```

*/
pub fn trait_dyn_dispatch() {
    println!("trait 动态分发");
}

/**

# trait 对象本质


示例 1:

```rust
    struct CloningLab {
        subjects: Vec<Box<Mammal>>,
        // subjects: Vec<Box<Mammal + Clone>>,
    }

    trait Mammal {
        fn walk(&self);
        fn run(&self);
    }

    #[derive(Clone)]
    struct Cat {
        meow_factor: u8,
        purr_factor: u8
    }

    impl Mammal for Cat {
        fn walk(&self) {
            println!("Cat::walk");
        }
        fn run(&self) {
            println!("Cat::run")
        }
    }


```

示例2:

```rust

    #![allow(unused)]
    #![feature(raw)]

    use std::{mem, raw};

    // an example trait
    trait Foo {
        fn bar(&self) -> i32;
    }

    impl Foo for i32 {
        fn bar(&self) -> i32 {
            *self + 1
        }
    }

    fn main() {
        let value: i32 = 123;

        // let the compiler make a trait object
        let object: &dyn Foo = &value;

        // look at the raw representation
        let raw_object: raw::TraitObject = unsafe { mem::transmute(object) };

        // the data pointer is the address of `value`
        assert_eq!(raw_object.data as *const i32, &value as *const _);

        let other_value: i32 = 456;

        // construct a new object, pointing to a different `i32`, being
        // careful to use the `i32` vtable from `object`
        let synthesized: &dyn Foo = unsafe {
            mem::transmute(raw::TraitObject {
                data: &other_value as *const _ as *mut (),
                vtable: raw_object.vtable,
            })
        };

        // it should work just as if we had constructed a trait object out of
        // `other_value` directly
        assert_eq!(synthesized.bar(), 457);
    }

```

正常 trait Object 布局图：

```text

                                                           cat's vtable
Cat Layout                     Trait Object
+------------+              +--------------+             +---------------+
|            |              |              |             |               |
|            |              |              |     +-----> | drop pointer  |
| meow_factor|              |              |     |       |    size       |
|            +<--------------+data pointer |     |       |    align      |
| purr_factor|              |              |     |       |               |
|            |              | vtable pointer-----+       | run fn pointer|
|            |              |              |             |walk fn pointer|
|            |              |              |             |               |
+------------+              +--------------+             |               |
                                                         |               |
                                                         |               |
                                                         |               |
                                                         +---------------+

```

假设：trait Mammal + Clone 布局图：

注意：非法

```text

                                                                  Mammal
                                                           cat's vtable
Cat Layout                     Trait Object
+------------+              +--------------+             +---------------+
|            |              |              |             |               |
|            |              |              |     +-----> | drop pointer  |
| meow_factor|              |              |     |       |    size       |
|            +<--------------+data pointer |     |       |    align      |
| purr_factor|              |              |     |       |               |
|            |              | vtable pointer-----+       | run fn pointer|
|            |              |              |             |walk fn pointer|
|            |              |              +-----+       |               |
+------------+              +--------------+     |       |               |
                                                 |       |               |
                                                 |       |               |
                                                 |       |               |
                                                 |       +---------------+
                                                 |
                                                 |
                                                 |           Clone Vtable
                                                 |
                                                 +-----> +--------------+
                                                         |              |
                                                         | drop pointe  |
                                                         |              |
                                                         | size         |
                                                         | align        |
                                                         |              |
                                                         |  clone       |
                                                         |  fn pointer  |
                                                         |              |
                                                         +--------------+


```

    假设：trait 继承(`trait MammalClone: Mammal+Clone`)布局图：

    注意：非法

```text

                                                            MammalClone
                                                           cat's vtable
Cat Layout                     Trait Object
+------------+              +--------------+             +-----------------+
|            |              |              |             |                 |
|            |              |              |     +-----> | drop pointer    |
| meow_factor|              |              |     |       |    size         |
|            +<--------------+data pointer |     |       |    align        |
| purr_factor|              |              |     |       |                 |
|            |              | vtable pointer-----+       | run fn pointer  |
|            |              |              |             |walk fn pointer  |
|            |              |              |             |                 |
+------------+              +--------------+             |clone fn pointer |
                                                         |                 |
                                                         |                 |
                                                         |                 |
                                                         +-----------------+

 ```


*/
pub fn trait_object() {
    println!("trait 动态分发");
}

/**

#  对象安全



### 对象安全

    一个 trait 如果能实现自己，就认为它是对象安全的

为什么必须是对象安全呢？

trait对象，在运行时已经擦除了类型信息，要通过虚表调用相应的方法。不像静态分发那样，trait对象不是为每个类型都实现trait的方法，而是只实现一个副本（自动为其实现自身），结合虚函数去调用。

现在想一个问题： 假如那个类型没有实现这个方法怎么办？
实际上，会有很多种情况下，会出现这个问题。运行时确定的类型和方法应该合法的，保证trait对象在运行时可以安全地调用相关的方法。

比如trait里有泛型函数。这就搞的很复杂了，可能运行时无法确定该调用哪个函数。反正是各种情况吧。所以，为了避免出现这种问题，官方引入了对象安全的概念。
实际上就是引入了一系列的规则，也就是上面列出的那些。编译器根据这些规则，在编译期判断一个你写的trait对象，是不是合法的。

比如：trait对象其实在内部维护两个表：safe_vtable和nonself_vtable，标记有where Self: Sized的会被归类到nonself_vtable，也就是说，不会被trait对象调用。
这样的话，方法标记有where Self: Sized的trait对象自然是安全的，因为这表示 这个方法 只能为 Self: Sized 都类型实现，是有条件的，所以在运行时有可能存在无效（万一有不是Sized的类型调用，就没有该方法）调用。

如果是合法的，则代表了，这个trait对象在运行时调用方法应该是没问题的。
不会出现没有实现，或者不知道该调用哪个的情况。
这就是对象安全的概念。它和内存安全并无直接关系。
所以，对象安全的本质就是为了让trait对象可以安全地调用相应的方法。

如果没有Sized的限定，那么就会很容易写出无用的类型。比如 Box，它用做trait对象即便会编译，但是不能用它做任何事情（后面有演示代码）。
对于更复杂的trait，往往就没有这么明显了，只有在做了大量繁重的工作之后可能会突然发现某个trait对象无法正常调用方法。
所以，为trait增加Sized限定，然后编译器自动为该trait实现自身，就可以在编译期准确排除无效的trait对象。
这就是对象安全。需要注意的是，对象安全和内存安全并无直接的关联，它只是保证trait对象在运行时可以安全准确地调用相关的方法。

```rust
    trait StarkFamily {
        fn last_name(&self)  -> &'static str;
        fn totem(&self) -> &'static str;
    }

    trait TullyFamily {
        fn territory(&self) -> &'static str;
    }

    trait Children {
        fn new(first_name: &'static str) -> Self where Self: Sized;
        fn first_name(&self) -> &'static str;
    }

    impl StarkFamily for Children {
        fn last_name(&self)  -> &'static str{
            "Stark"
        }

        fn totem(&self)  -> &'static str{
            "Wolf"
        }
    }

    impl TullyFamily for Children {
        fn territory(&self)  -> &'static str{
            "Riverrun City"
        }
    }

    struct People{
        first_name: &'static str
    }

    impl Children for People {
        fn new(first_name: &'static str) -> Self where Self: Sized{
            println!("hello : {} Stark ", first_name);
            People{first_name: first_name}
        }

        fn first_name(&self) -> &'static str{
            self.first_name
        }
    }

    fn full_name(person: Box<dyn Children>) {
        println!(" --- Winter is coming, the lone {:?} dies, the pack lives ---", person.totem());
        let full = format!("{} {}", person.first_name(), person.last_name() );
        println!("I'm {:?}", full );
        println!("My Mother come from {:?}", person.territory());
    }

    fn main() {
        let sansa = People::new("Sansa");
        let aray = People::new("Aray");

        let starks: Box<dyn Children> = Box::new(sansa);
        full_name(starks);

        let starks: Box<dyn Children> = Box::new(aray);
        full_name(starks);
    }

```


对象安全规则 Rust 源码：[https://github.com/rust-lang/rust/blob/941343e0871dd04ea774e8cee7755461b144ef29/compiler/rustc_middle/src/traits/mod.rs#L643](https://github.com/rust-lang/rust/blob/941343e0871dd04ea774e8cee7755461b144ef29/compiler/rustc_middle/src/traits/mod.rs#L643)


*/
pub fn object_safety() {
    println!("对象安全本质");
}

/**

# 当不能实现 trait 对象当时候该如何？

1. 将其转化为 Enum

trait 对象代码：

```rust
    trait KnobControl {
        fn set_position(&mut self, value: f64);
        fn get_value(&self) -> f64;
    }

    struct LinearKnob {
        position: f64,
    }

    struct LogarithmicKnob {
        position: f64,
    }

    impl KnobControl for LinearKnob {
        fn set_position(&mut self, value: f64) {
            self.position = value;
        }

        fn get_value(&self) -> f64 {
            self.position
        }
    }

    impl KnobControl for LogarithmicKnob {
        fn set_position(&mut self, value: f64) {
            self.position = value;
        }

        fn get_value(&self) -> f64 {
            (self.position + 1.).log2()
        }
    }

    fn main() {
        let v: Vec<Box<dyn KnobControl>> = vec![
            //set the knobs
        ];

        //use the knobs
    }
```

转为 enum：

```rust
    enum Knob {
        Linear(LinearKnob),
        Logarithmic(LogarithmicKnob),
    }

    impl KnobControl for Knob {
        fn set_position(&mut self, value: f64) {
            match self {
                Knob::Linear(inner_knob) => inner_knob.set_position(value),
                Knob::Logarithmic(inner_knob) => inner_knob.set_position(value),
            }
        }

        fn get_value(&self) -> f64 {
            match self {
                Knob::Linear(inner_knob) => inner_knob.get_value(),
                Knob::Logarithmic(inner_knob) => inner_knob.get_value(),
            }
        }
    }
```

当 trait 不满足对象安全规则的时候，也可以用 Enum 代替。

```rust
#![allow(unused)]

use core::ops::Add;

trait KnobControl<T: Add + Add<Output = T> + Copy> {
    fn set_position(&mut self, value: T);
    fn get_value(&self, p: T) -> T;
}

struct LinearKnob<T: Add+ Add<Output = T> + Copy> {
    position: T,
}

struct LogarithmicKnob<T: Add+ Add<Output = T> + Copy>  {
    position: T,
}

impl<T: Add+ Add<Output = T> + Copy> KnobControl<T> for LinearKnob<T> {
    fn set_position(&mut self, value: T) {
        self.position = value;
    }

    fn get_value(&self, p: T) -> T {
        self.position + p
    }
}

impl<T: Add+ Add<Output = T> + Copy> KnobControl<T> for LogarithmicKnob<T> {
    fn set_position(&mut self, value: T) {
        self.position = value;
    }

    fn get_value(&self, p: T) -> T {
        (self.position + p)
    }
}

fn main() {
    enum Knob<T: Add+ Add<Output = T> + Copy> {
        Linear(LinearKnob<T>),
        Logarithmic(LogarithmicKnob<T>),
    }

    impl<T: Add+ Add<Output = T> + Copy> KnobControl<T> for Knob<T> {
        fn set_position(&mut self, value: T) {
            match self {
                Knob::Linear(inner_knob) => inner_knob.set_position(value),
                Knob::Logarithmic(inner_knob) => inner_knob.set_position(value),
            }
        }

        fn get_value(&self, p: T) -> T {
            match self {
                Knob::Linear(inner_knob) => inner_knob.get_value(p),
                Knob::Logarithmic(inner_knob) => inner_knob.get_value(p),
            }
        }
    }
}


```


2. 利用 “魔法” ，相当于加一层代理 ： 参考：[https://github.com/dtolnay/erased-serde/blob/master/explanation/main.rs](https://github.com/dtolnay/erased-serde/blob/master/explanation/main.rs)

*/
pub fn trait_object_to_enum() {
    println!("blanket impls");
}

/**

# Overlapping blanket impls

当前 Rust 不支持  `trait `为 同一个类型覆盖实现：

```rust
    trait Blanket {
        fn blanket(&self) -> &'static str;
    }

    impl Blanket for u8 {
        fn blanket(&self) -> &'static str {
            "impl1"
        }
    }

    // Compilation fails at that point
    impl Blanket for u8 {
        fn blanket(&self) -> &'static str {
            "impl2"
        }
    }

    fn main() {
        // If compilation succeeded, what would be printed?
        println!("{}", 0u8.blanket());
    }

```

再比如泛型：

```rust
    impl <T: ToString> Blanket for T { ... }

    // Compilation fails at that point
    impl <T: Clone> Blanket for T { ...}

```

以上是 Rust 不允许的。

虽然特化功能也逐渐开始支持，但不足以解决上面这种存在`trait`实现“竞争”的情况。

一个解决方案是：

```rust
    trait Blanket<I> {
        fn blanket(&self) -> &'static str;
    }

    impl Blanket<u8> for u8 {
        fn blanket(&self) -> &'static str {
            "u8"
        }
    }

    impl<T: ToString> Blanket<&ToString> for T {
        fn blanket(&self) -> &'static str {
            "ToString"
        }
    }

    trait CloneBlanket {}

    impl<T: Clone> Blanket<&CloneBlanket> for T {
        fn blanket(&self) -> &'static str {
            "Clone"
        }
    }

    trait TryIntoBlanket<T> {
        type Error;
    }

    impl<T, E, U> Blanket<&TryIntoBlanket<T, Error = E>> for U
    where
        U: TryInto<T, Error = E>,
    {
        fn blanket(&self) -> &'static str {
            "try_into"
        }
    }

    impl<T: ToString> Blanket<&ToString> for T {
        fn blanket(&self) -> &'static str {
            "to_string"
        }
    }

    impl<T: AsRef<U>, U: ?Sized> Blanket<&AsRef<U>> for T {
        fn blanket(&self) -> &'static str {
            "as_ref"
        }
    }

```

方案参考：https://codesandwich.github.io/overlapping_blanket_impls/



*/
pub fn blanket_impls() {
    println!("blanket impls");
}

/**

    ### 对象安全规则里，为什么需要 `Self: Sized`

    思考：什么情况下需要 `Self: Sized` ？

    ```rust

    trait WithConstructor {
        fn build(param: usize) -> Self where Self: Sized;

        fn new() -> Self where Self: Sized {
            Self::build(0)
        }


        fn t(&self){
            println!("T");
        }

        fn p(&self){
            self.t();
            println!("hello");
        }
    }

    struct A;

    impl WithConstructor for A {
        fn build(param: usize) -> Self{
            A
        }

    }

    fn main(){
        let a : &WithConstructor = &A ;
    }
    ```

    示例 2:

    ```rust

    trait Test {
        fn foo(self);

        fn works(self: Box<Self>){
            println!("hello Trait");
        }

        fn fails(self: Box<Self>)
        where Self: Sized
        {
            self.foo()
        }
    }

    struct Concrete;

    impl Concrete {
        fn hello(&self){
            println!("hello");
        }
    }

    struct Bar;

    impl Bar {
        fn hello(&self){
            println!("hello Bar");
        }
    }

    impl Test for Bar {
        fn foo(self) { () }
        fn works(self: Box<Self>) { self.hello()}
    }

    impl Test for Concrete {
        fn foo(self) { () }
        fn works(self: Box<Self>) { self.hello()}
    }

    fn main() {
        let concrete: Box<dyn Test> = Box::new(Concrete);
        concrete.works();
        let concrete: Box<dyn Test> = Box::new(Bar);
        concrete.works();
        // concrete.fails(); // compilation error
    }

    ```

    结论：
    1. `Self: Sized` 为了保证 trait 默认实现内部的 Self 调用都是合法的。
    2. 防止 函数体内包含了 Self 的默认实现混入虚表。因为虚表内 Self 无法确定。

    ### Sized  vs  ?Sized

    ```rust
    trait WithConstructor {
        fn build(param: usize) -> Self where Self: ?Sized;

        fn new() -> Self where Self: ?Sized {
            Self::build(0)
        }
    }
    ```

*/
pub fn trait_self_sized_bound() {
    println!("trait object vs Self: Sized");
}
