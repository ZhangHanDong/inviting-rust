//! 第二章：Rust核心概念
//! 2.7 元编程
//! 
//! 内容包括：
//!  - 反射
//!  - 宏
//!     - Rust 编译过程再解析
//!     - 声明宏
//!         - 标准库内置宏
//!         - 自定义宏
//!     - 过程宏
//!         - bang 宏
//!         - derive 宏
//!         - 属性宏
//!     - 过程宏实际项目的应用
//!         - dervie_more
//!         - metric
//!         - reflect
//! 
//! 


/**
   
   # 动态自省

   示例1:

   [https://doc.rust-lang.org/std/any/index.html](https://doc.rust-lang.org/std/any/index.html)

    示例2:

    ```rust

    use std::any::Any;

    trait Foo: Any {
        fn as_any(&self) -> &Any;
    }

    impl<T: Any> Foo for T {
        fn as_any(&self) -> &Any {
            self
        }
    }

    struct Bar {}

    struct Baz {}

    impl PartialEq for Foo {
        fn eq(&self, other: &Foo) -> bool {
            let me = self.as_any();
            let you = other.as_any();
            if me.is::<Bar>() && you.is::<Bar>() {
                true
            } else if me.is::<Baz>() && you.is::<Baz>() {
                true
            } else {
                false
            }
        }
    }

    fn main() {
        let bar: Bar = Bar {};
        let baz: Baz = Baz {};
        let foo1: &Foo = &bar;
        let foo2: &Foo = &baz;
        println!("{:?}", foo1 == foo2);
    }

    ```

    示例 3:

    ```rust
        use std::any::Any;
        struct UnStatic<'a> { x: &'a i32 }
        fn main() {
            let a = 42;
            let v = UnStatic { x: &a };
            let mut any: &Any;
            //any = &v;  // Compile Error!
        }
    ```
    
    修正：

    ```rust
    use std::any::Any;
    struct UnStatic<'a> { x: &'a i32 }
    static ANSWER: i32 = 42;
    fn main() {
        let v = UnStatic { x: &ANSWER };
        let mut a: &Any;
        a = &v;
        assert!(a.is::<UnStatic>());
    }
    ```

    示例4:

    oso 库应用

    [https://github.com/osohq/oso/blob/main/languages/rust/oso/src/host/class.rs](https://github.com/osohq/oso/blob/main/languages/rust/oso/src/host/class.rs)

    示例 5:

    bevy_reflect 库应用

    [https://github.com/bevyengine/bevy/blob/main/crates/bevy_reflect/src/lib.rs](https://github.com/bevyengine/bevy/blob/main/crates/bevy_reflect/src/lib.rs)

 */
pub fn any_refection(){}



/**
  # 声明宏

  宏展开命令： cargo rustc -- -Z unstable-options --pretty=expanded

  示例1:

  ```rust
    macro_rules! unless {
        ($arg:expr, $branch:expr) => ( if !$arg { $branch };); 
    } 
    fn cmp(a: i32, b: i32) {
        unless!( a > b, {
            println!("{} < {}", a, b);
        });
    }
    fn main() {
        let (a, b) = (1, 2);
        cmp(a, b);
    }
  ```

  支持 token 类型：

  ```text
        item — an item, like a function, struct, module, etc.
        block — a block (i.e. a block of statements and/or an expression, surrounded by braces)
        stmt — a statement
        pat — a pattern
        expr — an expression
        ty — a type
        ident — an identifier
        path — a path (e.g., foo, ::std::mem::replace, transmute::<_, int>, …)
        meta — a meta item; the things that go inside #[...] and #![...] attributes
        tt — a single token tree
        vis — a possibly empty Visibility qualifier
  ```


  示例2:

  ```rust

    macro_rules! hashmap {
        ($($key:expr => $value:expr),* ) => {
            {
                let mut _map = ::std::collections::HashMap::new();
                $(  
                    _map.insert($key, $value); 
                )*
                _map
            }
        };
    }
    fn main(){
        let map = hashmap!{
            "a" => 1,
            "b" => 2
        //  "c" => 3, // V1.0不支持结尾有逗号
        };
        assert_eq!(map["a"], 1);
    }

  ```

  示例3:

  ```rust
    macro_rules! hashmap {
        ($($key:expr => $value:expr,)*) =>
            {  hashmap!($($key => $value),*) };
        ($($key:expr => $value:expr),* ) => {
            {
                let mut _map = ::std::collections::HashMap::new();
                $(
                    _map.insert($key, $value);
                )*
            _map
        }
    };
    }
    fn main(){
        let map = hashmap!{
            "a" => 1,
            "b" => 2,
            "c" => 3, 
        };
        assert_eq!(map["a"], 1);
    }
  ```

  示例4:

  ```rust
    macro_rules! hashmap {
        ($($key:expr => $value:expr),* $(,)*) => {
            {
                let mut _map = ::std::collections::HashMap::new();
                $(
                    _map.insert($key, $value);
                )*
                _map
            }
    };
    }
    fn main(){
        let map = hashmap!{
            "a" => 1,
            "b" => 2,
            "c" => 3, 
        };
        assert_eq!(map["a"], 1);
    }

  ```

  示例5:

  ```rust
    macro_rules! unit {
        ($($x:tt)*) => (());
    }
    macro_rules! count {
        ($($key:expr),*) => (<[()]>::len(&[$(unit!($key)),*]));
    }
    macro_rules! hashmap {
        ($($key:expr => $value:expr),* $(,)*) => {
            {
            let _cap = count!($($key),*);
            let mut _map 
                = ::std::collections::HashMap::with_capacity(_cap);
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
    }
    fn main(){
        let map = hashmap!{
            "a" => 1,
            "b" => 2,
            "c" => 3, 
        };
        assert_eq!(map["a"], 1);
    }

  ```

  示例6: 

  ```rust
    macro_rules! hashmap {
        (@unit $($x:tt)*) => (());
        (@count $($rest:expr),*) => 
            (<[()]>::len(&[$(hashmap!(@unit $rest)),*]));
        ($($key:expr => $value:expr),* $(,)*) => {
            {
                let _cap = hashmap!(@count $($key),*);
                let mut _map = 
                    ::std::collections::HashMap::with_capacity(_cap);
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
    }
    fn main(){
    let map = hashmap!{
        "a" => 1,
        "b" => 2,
        "c" => 3, 
    };
    assert_eq!(map["a"], 1);
    }
  ```

  示例7:

  ```rust
    macro_rules! sum {
        ($e:expr) => ({
            let a = 2;
            $e + a
        })
    }
    fn main(){
        // error[E0425]: cannot find value `a` in this scope
        let four = sum!(a);
    }
  ```
*/
pub fn declarative_macros(){}




/**

介绍：[serde.rs](https://serde.rs/)

参阅 :  [https://github.com/ZhangHanDong/proc_codegen](https://github.com/ZhangHanDong/proc_codegen)

过程宏三件套：

- [syn](https://github.com/dtolnay/syn)
- [quote](https://github.com/dtolnay/quote)
- [proc-macro2](https://github.com/alexcrichton/proc-macro2)

示例：封装 Diesel 方便 crud

```rust

    // find_by_or!{ Person -> people::[name:String || company_name:String]   }

    use super::*;

    pub struct DbOpByOrBy {
        pub model: Type,
        pub table: Ident,
        pub bracket_token: token::Bracket,
        pub content: FieldContentOr,
    }

    pub struct FieldContentOr {
        pub name1: Ident,
        pub ty1: Type,
        pub name2: Ident,
        pub ty2: Type,
    }

    impl Parse for DbOpByOrBy {
        fn parse(input: ParseStream) -> Result<Self> {
            let content;
            let model: Type = input.parse()?;
            input.parse::<Token![->]>()?;
            let table: Ident = input.parse()?;
            input.parse::<Token![::]>()?;
            let bracket_token = bracketed!(content in input);
            let content = content.parse()?;
            Ok(DbOpByOrBy {
                model,
                table,
                bracket_token,
                content,
            })
        }
    }

    impl Parse for FieldContentOr {
        fn parse(input: ParseStream) -> Result<Self> {
            let name1: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let ty1: Type = input.parse()?;
            input.parse::<Token![||]>()?;
            let name2: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let ty2: Type = input.parse()?;
            Ok(FieldContentOr {
                name1,
                ty1,
                name2,
                ty2,
            })
        }
    }

    // in lib.rs

    // find_by_or!{ Person -> people::[name:String || company_name:String]   }

    #[proc_macro]
    pub fn find_by_or(input: TokenStream) -> TokenStream {
        let DbOpByOrBy {
            model,
            table,
            bracket_token,
            content,
        } = parse_macro_input!(input as DbOpByOrBy);
        let (name1, name2) = (content.name1, content.name2);
        let (ty1, ty2) = (content.ty1, content.ty2);
        let fn_name = format!("find_by_{}_or_{}", name1, name2);
        let fn_name = Ident::new(&fn_name, proc_macro2::Span::call_site());

        let expanded = quote! {
            impl #model {
                pub fn #fn_name(conn: &PgConnection, #name1: #ty1, #name2: #ty2) -> QueryResult<#model> {
                    #table::table
                    .filter(#table::dsl::#name1.eq(#name1))
                    .or_filter(#table::dsl::#name2.eq(#name2))
                    .get_result(conn)
                }
            }
        };
        TokenStream::from(expanded)
    }


```

有用的第三方库：

- [derive_more](https://github.com/JelteF/derive_more)

*/
pub fn derive_proc_macros(){}



/**

    # 属性宏

    案例：[magnet/metered-rs](https://github.com/magnet/metered-rs)

    有用的第三方库介绍：[darling](https://github.com/TedDriggs/darling)

    案例： [elichai/log-derive](https://github.com/elichai/log-derive)
 

*/
pub fn attributes_proc_macros(){}