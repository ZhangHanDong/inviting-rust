# 分层错误处理

错误处理：顾名思义，处理错误。既然要处理错误，那肯定是指开发者可以处理的情况。

-  Option: 「有」与「无」
-  Result:「对」与「错」

### Option

```rust
fn get_shortest(names: Vec<&str>) -> Option<&str> {
    if names.len() > 0 {
        let mut shortest = names[0];
        for name in names.iter() {
            if name.len() < shortest.len() {
                shortest = *name;
            }
        }
        Some(shortest)
    } else {
        None
    }
}
fn show_shortest(names: Vec<&str>) -> &str {
    match get_shortest(names) {
        Some(shortest) => shortest,
        None => "Not Found",
    }
}
fn main(){
    assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
    assert_eq!(show_shortest(Vec::new()), "Not Found");
}
```

使用 Match 在 「盒内」处理 Option

```rust
fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
    match get_shortest(names) {
        Some(shortest) => Some(shortest.len()),
        None => None,
    }
}
fn main(){
    assert_eq!(get_shortest_length(vec!["Uku","Felipe"]),Some(3));
    assert_eq!(get_shortest_length(Vec::new()), None);
}
```

使用标准库内建组合子处理：

```rust
fn double(value: f64) -> f64 {
    value * 2.
}
fn square(value: f64) -> f64 {
    value.powi(2 as i32)
}
fn inverse(value: f64) -> f64 {
    value * -1.
}
fn log(value: f64) -> Option<f64> {
    match value.log2() {
        x if x.is_normal() => Some(x),
        _ => None
    }
}
fn sqrt(value: f64) -> Option<f64> {
    match value.sqrt() {
        x if x.is_normal() => Some(x),
        _ => None
    }
}
fn main () {
    let number: f64 = 20.;
    let result = Option::from(number)
        .map(inverse).map(double).map(inverse)
        .and_then(log).map(square).and_then(sqrt);
    match result {
        Some(x) => println!("Result was {}.", x),
        None    => println!("This failed.")
    }
}
```

### Result

- Error trait:[https://doc.rust-lang.org/stable/std/error/trait.Error.html](https://doc.rust-lang.org/stable/std/error/trait.Error.html)
- Result Error Handle : [read-sum crate]
- `?` and [std::ops::Try](https://doc.rust-lang.org/stable/std/ops/trait.Try.html)

```rust
use std::num::ParseIntError;
// fn square(number_str: &str) -> Result<i32, ParseIntError>
// {
//    number_str.parse::<i32>().map(|n| n.pow(2))
// }
type ParseResult<T> = Result<T, ParseIntError>;
fn square(number_str: &str) -> ParseResult<i32>
{
    number_str.parse::<i32>().map(|n| n.pow(2))
}
fn main() {
    match square("10") {
        Ok(n) => assert_eq!(n, 100),
        Err(err) => println!("Error: {:?}", err),
    }
}
```
