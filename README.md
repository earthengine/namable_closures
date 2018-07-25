# Make Rust Closure Types Namable

This crate provides some types and macros that can create closures which type is namable.

# Update
2018-07-25 v0.2.2 Add Recursion support (experimential)

# Usage

To use, as usual,

```rust
#[macro_use] extern crate namable_closures;
use namable_closures::Closure;
fn main() {
    let add_ten:Closure<i32,(i32,),i32> = closure!(state=10 => move |i| i+*state);
    println!("{}",add_ten.stable_call_once(1)); //11
}
```

There are 5 variants of the types, each of them have 3 type variables. The `State`
variable correspond to the captured environment of a closure. The `Input` must be
a unit or tuple type, correspond to the arguments of the closure. The `Output` is
the return type of the closure.

In additional to letting you name the closure type, the function body cannot capture
any variable by default, so all captures must be explicit.

<table>
<tr>
<th>Macro Grammar</th>
<th>Struct</th>
</tr>
<tr>
<td><code>closure!(state=exp => |x,y| body(x,y,state))</code></td>
<td><code>Closure</code></td>
</tr>
<tr>
<td><code>closure!(state=exp => move |x,y| body(x,y,state))</code></td>
<td><code>ClosureOnce</code></td>
</tr>
<tr>
<td><code>closure!(mut state=exp => |x,y| body(x,y,state))</code></td>
<td><code>ClosureMut</code></td>
</tr>
<tr>
<td><code>closure!(mut state=exp => move |x,y| body(x,y,state))</code></td>
<td><code>ClosureOnce</code> (with mutable <code>state</code>)</td>
</tr>
<tr>
<td><code>closure!(ref state=exp => move |x,y| body(x,y,state))</code></td>
<td><code>ClosureRef</code></td>
</tr>
<tr>
<td><code>closure!(ref mut state=exp => move |x,y| body(x,y,state))</code></td>
<td><code>ClosureRefMut</code></td>
</tr>
</table>

# DO NOT USE `Closure` or `ClosureMut` unless you know what you are doing

In most cases you will need the `move` keyword, this will let your closure own the state. If you don't need to own the state, make sure your state expression represents a `&T` (for `Closure`s) or `&mut T` (for `ClosureMut`) types. 

# Nightly only features

A shadow set of traits was defined to use before `fn_traits` and `unboxed_closures`
features being stablized. Use of them is a bit ugly:

```rust
let myclosure:Closure<i32,(),i32> = closure!(s=&0 => || *s);
fn1_expect_closure(||myclosure.stable_call());
let myclosure:ClosureRef<(),(i32,),i32> = closure!(s=&() => |i| i);
fn2_expect_closure(|i|myclosure.stable_call((i,)));
let myclosure:ClosureRef<(),(i32,i32),i32> = closure!(s=&() => |i,j| i+j);
fn3_expect_closure(|i,j|myclosure.stable_call((i,j)));
```

If you don't mind having to compile in nightly Rust, you can add the following to `cargo.toml`:

```toml
[dependencies.namable_closures]
features = ["nightly"]
```

Now you can write the following:

```rust
let myclosure:Closure<i32,(),i32> = closure!(s=&0 => || *s);
fn1_expect_closure(||myclosure());
let myclosure:ClosureRef<(),(i32,),i32> = closure!(s=&() => |i| i);
fn2_expect_closure(|i|myclosure(i));
let myclosure:ClosureRef<(),(i32,i32),i32> = closure!(s=&() => |i,j| i+j);
fn3_expect_closure(myclosure);
```
