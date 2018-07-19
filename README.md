# Make Rust Closure Types Namable

This crate provides some types and macros that can create closures which type is namable.

To use, as usual,

```rust
#[macro_use] extern crate namable_closures;
use namable_closures::Closure;
fn main() {
    let add_ten:Closure<i32,(i32,),i32> = closure!(state, i,  i+*state, 10);
    println!("{}",add_ten(1)); //11
}
```

There are 5 variants of the types, each of them have 3 type variables. The `State`
variable correspond to the captured environment of a closure. The `Input` must be
a unit or tuple type, correspond to the arguments of the closure. The `Output` is
the return type of the closure.

In additional to letting you name the closure type, the function body cannot capture
any variable by default, so all captures must be explicit.

# Nightly only

As this crate depends on unstable featues, unless `fn_traits` and `unboxed_closures`
become stable this crate only works on nightly build.
