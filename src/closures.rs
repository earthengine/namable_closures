// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[doc="
A Closure owns its state, and only refers to the state when called.
 
Correspond to unnamable closures like:
```ignore
|...| { /* only refers captured variables */ }
```
# Example:
```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::Closure;
# use namable_closures::StableFnMut;
# use namable_closures::StableFn;
# struct Point{x:i32,y:i32}
# impl Point{ fn new(x:i32,y:i32) -> Point {Point{x:x,y:y}} }
let state = 10;
let add_ten:Closure<i32,(i32,),i32>
    = closure!(state=&state => |i| i+10);
assert_eq!(add_ten.stable_call((1,)),11);
let state = Point::new(10,20);
let mut offset:Closure<Point,(i32,i32),Point>
    = closure!(state=&state => |a,b| Point::new(state.x+a,state.y+b));
let p = offset.stable_call_mut((1i32,2i32));
assert_eq!(p.x,11);
assert_eq!(p.y,22);
```
"]
#[cfg_attr(feature="nightly", doc="
# The same example that uses the unstable features:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::Closure;
# struct Point{x:i32,y:i32}
# impl Point{ fn new(x:i32,y:i32) -> Point {Point{x:x,y:y}} }
    let state = 10;
    let add_ten:Closure<i32,(i32,),i32>
        = closure!(state=&state => |i| i+10);
        assert_eq!(add_ten(1),11);
    let state = Point::new(10,20);
    let offset:Closure<Point,(i32,i32),Point>
        = closure!(state=&state => |a,b| Point::new(state.x+a,state.y+b));
    let p = offset(1i32,2i32);
    assert_eq!(p.x,11);
    assert_eq!(p.y,22);
```
")]
pub struct Closure<'a, State, Input, Output>
where
    State: 'a
{
    f: fn(&State, Input) -> Output,
    t: &'a State,
}
impl<'a, State, Input, Output> Copy for Closure<'a, State, Input, Output> 
{}
impl<'a, State, Input, Output> Clone for Closure<'a, State, Input, Output>
{
    fn clone(&self) -> Self {
        *self
    }
}
#[doc="
A Closure does not own its state, and only refers to the state when called.
 
Correspond to unnamable closures like:
```ignore
|...| { /* only refers captured variables */ }
```
Examples:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::ClosureRef;
# use namable_closures::StableFnMut;
# use namable_closures::StableFn;
# struct Point{x:i32,y:i32}
# impl Point{ fn new(x:i32,y:i32) -> Point {Point{x:x,y:y}} }
// state refered as reference in body, but moved to the closure
let add_ten:ClosureRef<i32,(i32,),i32>
    = closure!(ref state=10 => move |i| i+*state);
assert_eq!(add_ten.stable_call((1,)),11);
let mut offset:ClosureRef<Point,(i32,i32),Point>
    = closure!(ref state=Point::new(10,20) => move |a,b| Point::new(state.x+a,state.y+b));
let p = offset.stable_call_mut((1,2));
assert_eq!(p.x,11);
assert_eq!(p.y,22);
```
"]
#[cfg_attr(feature="nightly", doc="
# The same example that uses the unstable features:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::ClosureRef;
# struct Point{x:i32,y:i32}
# impl Point{ fn new(x:i32,y:i32) -> Point {Point{x:x,y:y}} }
// state refered as reference in body, but moved to the closure
let add_ten:ClosureRef<i32,(i32,),i32>
    = closure!(ref state=10 => move |i| i+*state);
assert_eq!(add_ten(1),11);
let offset:ClosureRef<Point,(i32,i32),Point>
    = closure!(ref state=Point::new(10,20) => move |a,b| Point::new(state.x+a,state.y+b));
let p = offset(1,2);
assert_eq!(p.x,11);
assert_eq!(p.y,22);
```
")]
pub struct ClosureRef<State, Input, Output>
{
    f: fn(&State, Input) -> Output,
    t: State,
}
impl<State, Input, Output> Copy for ClosureRef<State, Input, Output> 
where
    State: Copy
{}
impl<State, Input, Output> Clone for ClosureRef<State, Input, Output>
where
    State: Clone
{
    fn clone(&self) -> Self {
        Self { f:self.f, t:self.t.clone()}
    }
}

#[doc="
A namable closure that does not own its state and can mutate it when called.

It is not possible to implement `Copy` or `Clone` for this type.
 
Correspond to unnamable closures like:
```ignore
 |...| { /* mutates captured variables */ }
```

# Example:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::ClosureMut;
# use namable_closures::StableFnMut;
let mut state = 0;
{
   let mut match_cnt:ClosureMut<i32,(i32,i32),()>
       = closure!(mut state=&mut state => |a,b| {if a==b { *state+=1 }});
   for i in 0..10 { match_cnt.stable_call_mut((i,i*3%10)); }
}
assert_eq!(state,2);
```
"]
#[cfg_attr(feature="nightly", doc="
# The same example that uses the unstable features:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::ClosureMut;

let mut state = 0;
{
   let mut match_cnt:ClosureMut<i32,(i32,i32),()>
       = closure!(mut state=&mut state => |a,b| {if a==b { *state+=1 }});
   for i in 0..10 { match_cnt(i,i*3%10); }
}
assert_eq!(state,2);
```
")]
pub struct ClosureMut<'a, State, Input, Output>
where
    State: 'a
{
    f: fn(&mut State, Input) -> Output,
    t: &'a mut State,
}

#[doc="
A namable closure that owns its state and can mutate it when called.
 
Correspond to unnamable closures like:
```ignore
 move |...| { /* mutates captured variables */ }
```

# Example:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::ClosureRefMut;
# use namable_closures::StableFnMut;
# use namable_closures::StableFnOnce;
let mut accumulate:ClosureRefMut<i32,(i32,),i32>
    = closure!(ref mut state=0 => move |c| {*state+=c;*state});
assert_eq!(accumulate.stable_call_mut((1,)),1);
assert_eq!(accumulate.stable_call_once((2,)),3);
```
"]
#[cfg_attr(feature="nightly", doc="
# The same example that uses the unstable features:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::ClosureRefMut;
let mut accumulate:ClosureRefMut<i32,(i32,),i32>
    = closure!(ref mut state=0 => move |c| {*state+=c;*state});
assert_eq!(accumulate(1),1);
assert_eq!(accumulate(2),3);
```
")]
pub struct ClosureRefMut<State, Input, Output> {
    f: fn(&mut State, Input) -> Output,
    t: State,
}
impl<State,Input,Output> Copy for ClosureRefMut<State, Input, Output>
where
    State: Copy,
{}
impl<State,Input,Output> Clone for ClosureRefMut<State, Input, Output>
where
    State: Clone
{
    fn clone(&self) -> Self {
        Self{ f: self.f, t: self.t.clone() }
    }
}

#[doc="
When called, it consumes its state. So it can only be
called once.

Correspond to unnamable closures like:
```ignore
move |...| { /*consumes captured variables */ }
```

Example:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::ClosureOnce;
# use namable_closures::StableFnOnce;
# use std::io;
# struct RoleSet();
# impl RoleSet { fn from_config() -> RoleSet { RoleSet() }}
# struct Passwd();
# impl Passwd { fn get_from_cache() -> Passwd { Passwd() }}
# fn authenticate(_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
# fn check_user(_:RoleSet,_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
let sign_on:ClosureOnce<Passwd,(String,),Result<(),io::Error>>
    = closure!(passwd=Passwd::get_from_cache() => move |user| authenticate(user,passwd));
let auth:ClosureOnce<RoleSet,(String,Passwd),Result<(),io::Error>>
    = closure!(role_set=RoleSet::from_config() => move |user,passwd| check_user(role_set,user,passwd));
# struct MyStream();
# impl MyStream{
#   fn new() -> MyStream { MyStream() }
#   fn write_all(&mut self, _:&[u8]) -> Result<usize,io::Error> { Ok(0) }
#   fn read_exact_ex(&mut self, _:&mut [u8], _:usize) -> Result<(),io::Error> { Ok(()) }
# }
let send_data:ClosureOnce<MyStream,(&[u8],),Result<usize,io::Error>>
    = closure!(mut stream=MyStream::new() => move |data| stream.write_all(data));
send_data.stable_call_once((&[1u8],));
let read_data:ClosureOnce<MyStream,(&mut [u8],usize),Result<(),io::Error>>
    = closure!(mut stream=MyStream::new() => move |buf,len| stream.read_exact_ex(buf, len));
```
"]
#[cfg_attr(feature="nightly", doc="
# The same example that uses the unstable features:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::ClosureOnce;
# use std::io;
# struct RoleSet();
# impl RoleSet { fn from_config() -> RoleSet { RoleSet() }}
# struct Passwd();
# impl Passwd { fn get_from_cache() -> Passwd { Passwd() }}
# fn authenticate(_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
# fn check_user(_:RoleSet,_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
let sign_on:ClosureOnce<Passwd,(String,),Result<(),io::Error>>
    = closure!(passwd=Passwd::get_from_cache() => move |user| authenticate(user,passwd));
let auth:ClosureOnce<RoleSet,(String,Passwd),Result<(),io::Error>>
    = closure!(role_set=RoleSet::from_config() => move |user,passwd| check_user(role_set,user,passwd));
# struct MyStream();
# impl MyStream{
#   fn new() -> MyStream { MyStream() }
#   fn write_all(&mut self, _:&[u8]) -> Result<usize,io::Error> { Ok(0) }
#   fn read_exact_ex(&mut self, _:&mut [u8], _:usize) -> Result<(),io::Error> { Ok(()) }
# }
let send_data:ClosureOnce<MyStream,(&[u8],),Result<usize,io::Error>>
    = closure!(mut stream=MyStream::new() => move |data| stream.write_all(data));
send_data(&[1u8]);
let read_data:ClosureOnce<MyStream,(&mut [u8],usize),Result<(),io::Error>>
    = closure!(mut stream=MyStream::new() => move |buf,len| stream.read_exact_ex(buf, len));
```
")]
pub struct ClosureOnce<State, Input, Output> {
    f: fn(State, Input) -> Output,
    t: State,
}
impl<State, Input, Output> Copy for ClosureOnce<State, Input, Output>
where
    State: Copy,
{
}
impl<State, Input, Output> Clone for ClosureOnce<State, Input, Output>
where
    State: Clone,
{
    fn clone(&self) -> ClosureOnce<State, Input, Output> {
        Self::new(self.f, self.t.clone())
    }
}

impl<'a, State, Input, Output> Closure<'a, State, Input, Output> {
    pub fn new(f: fn(&State, Input) -> Output, t: &'a State) -> Self {
        Self { f, t }
    }
}
impl<State, Input, Output> ClosureRef<State, Input, Output> {
    pub fn new(f: fn(&State, Input) -> Output, t: State) -> Self {
        Self { f, t }
    }
}
impl<'a, State, Input, Output> ClosureMut<'a, State, Input, Output> {
    pub fn new(f: fn(&mut State, Input) -> Output, t: &'a mut State) -> Self {
        Self { f, t }
    }
}
impl<State, Input, Output> ClosureRefMut<State, Input, Output> {
    pub fn new(f: fn(&mut State, Input) -> Output, t: State) -> Self {
        Self { f, t }
    }
}
impl<State, Input, Output> ClosureOnce<State, Input, Output> {
    pub fn new(f: fn(State, Input) -> Output, t: State) -> ClosureOnce<State, Input, Output> {
        Self { f, t }
    }
}

use stable_fn::{StableFn,StableFnMut,StableFnOnce};

//All Closures implements StableFnOnce
impl<'a, State, Input, Output> StableFnOnce<Input> for Closure<'a, State, Input, Output> {
    type Output = Output;
    fn stable_call_once(self, i: Input) -> Output {
        let Self { f, t } = self;
        f(&t, i)
    }
}
impl<State, Input, Output> StableFnOnce<Input> for ClosureRef<State, Input, Output> {
    type Output = Output;
    fn stable_call_once(self, i: Input) -> Output {
        let Self { f, t } = self;
        f(&t, i)
    }
}
impl<'a, State, Input, Output> StableFnOnce<Input> for ClosureMut<'a, State, Input, Output> {
    type Output = Output;
    fn stable_call_once(self, i: Input) -> Output {
        let Self { f, mut t } = self;
        f(&mut t, i)
    }
}
impl<State, Input, Output> StableFnOnce<Input> for ClosureRefMut<State, Input, Output> {
    type Output = Output;
    fn stable_call_once(self, i: Input) -> Output {
        let Self { f, mut t } = self;
        f(&mut t, i)
    }
}
impl<State, Input, Output> StableFnOnce<Input> for ClosureOnce<State, Input, Output> {
    type Output = Output;
    fn stable_call_once(self, i: Input) -> Output {
        let ClosureOnce { f, t } = self;
        f(t, i)
    }
}

impl<'a, State, Input, Output> StableFnMut<Input> for Closure<'a, State, Input, Output> {
    fn stable_call_mut(&mut self, i: Input) -> Output {
        let Self { f, t } = self;
        f(t, i)
    }
}
impl<State, Input, Output> StableFnMut<Input> for ClosureRef<State, Input, Output> {
    fn stable_call_mut(&mut self, i: Input) -> Output {
        let Self { f, t } = self;
        f(t, i)
    }
}
impl<'a, State, Input, Output> StableFnMut<Input> for ClosureMut<'a, State, Input, Output> {
    fn stable_call_mut(&mut self, i: Input) -> Output {
        let Self { ref f, ref mut t } = self;
        f(t, i)
    }
}
impl<State, Input, Output> StableFnMut<Input> for ClosureRefMut<State, Input, Output> {
    fn stable_call_mut(&mut self, i: Input) -> Output {
        let Self { ref f, ref mut t } = self;
        f(t, i)
    }
}
impl<State, Input, Output> StableFnMut<Input> for ClosureOnce<State, Input, Output>
where
    State: Copy
{
    fn stable_call_mut(&mut self, i: Input) -> Output {
        let Self { f, t } = *self;
        f(t, i)
    }
}

impl<'a, State, Input, Output> StableFn<Input> for Closure<'a, State, Input, Output> {
    fn stable_call(&self, i: Input) -> Output {
        let Self { f, t } = self;
        f(t, i)
    }
}
impl<State, Input, Output> StableFn<Input> for ClosureRef<State, Input, Output> {
    fn stable_call(&self, i: Input) -> Output {
        let Self { f, t } = self;
        f(t, i)
    }
}
/// Note we implemented `StableFn` but not `Fn` for `ClosureRefMut`.
/// 
/// The reason is that if we implement `Fn` in this way it will be chosen to use
/// when called, but its behavour is different than `call_mut` as it does not
/// actually mutate the state - only a copy of the state was mutated.
/// 
/// This argument does not apply to `StableFn`, as it is up to the user to decide
/// whether `stable_call` (do not mutate) or `stable_call_mut` (do mutate) should
/// be called.
impl<State, Input, Output> StableFn<Input> for ClosureRefMut<State, Input, Output>
where
    State: Copy
{
    fn stable_call(&self, i: Input) -> Output {
        let Self { f, mut t } = *self;
        f(&mut t, i)
    }
}
impl<State, Input, Output> StableFn<Input> for ClosureOnce<State, Input, Output>
where
    State: Copy
{
    fn stable_call(&self, i: Input) -> Output {
        let Self { f, t } = *self;
        f(t, i)
    }
}



#[cfg(feature="nightly")]
impl<'a, State, Input, Output> FnOnce<Input> for Closure<'a, State, Input, Output> {
    type Output = Output;
    extern "rust-call" fn call_once(self, i: Input) -> Output {
        let Self { f, t } = self;
        f(&t, i)
    }
}
#[cfg(feature="nightly")]
impl<State, Input, Output> FnOnce<Input> for ClosureRef<State, Input, Output> {
    type Output = Output;
    extern "rust-call" fn call_once(self, i: Input) -> Output {
        let Self { f, t } = self;
        f(&t, i)
    }
}
#[cfg(feature="nightly")]
impl<'a, State, Input, Output> FnOnce<Input> for ClosureMut<'a, State, Input, Output> {
    type Output = Output;
    extern "rust-call" fn call_once(self, i: Input) -> Output {
        let Self { f, mut t } = self;
        f(&mut t, i)
    }
}
#[cfg(feature="nightly")]
impl<State, Input, Output> FnOnce<Input> for ClosureRefMut<State, Input, Output> {
    type Output = Output;
    extern "rust-call" fn call_once(self, i: Input) -> Output {
        let Self { f, mut t } = self;
        f(&mut t, i)
    }
}
#[cfg(feature="nightly")]
impl<State, Input, Output> FnOnce<Input> for ClosureOnce<State, Input, Output> {
    type Output = Output;
    extern "rust-call" fn call_once(self, i: Input) -> Output {
        let Self { f, t } = self;
        f(t, i)
    }
}

#[cfg(feature="nightly")]
impl<'a, State, Input, Output> FnMut<Input> for Closure<'a, State, Input, Output> {
    extern "rust-call" fn call_mut(&mut self, i: Input) -> Output {
        let Self { f, t } = self;
        f(t, i)
    }
}
#[cfg(feature="nightly")]
impl<State, Input, Output> FnMut<Input> for ClosureRef<State, Input, Output> {
    extern "rust-call" fn call_mut(&mut self, i: Input) -> Output {
        let Self { f, t } = self;
        f(t, i)
    }
}
#[cfg(feature="nightly")]
impl<'a, State, Input, Output> FnMut<Input> for ClosureMut<'a, State, Input, Output> {
    extern "rust-call" fn call_mut(&mut self, i: Input) -> Output {
        let Self { ref f, ref mut t } = self;
        f(t, i)
    }
}
#[cfg(feature="nightly")]
impl<State, Input, Output> FnMut<Input> for ClosureRefMut<State, Input, Output> {
    extern "rust-call" fn call_mut(&mut self, i: Input) -> Output {
        let Self { ref f, ref mut t } = self;
        f(t, i)
    }
}
#[cfg(feature="nightly")]
impl<State, Input, Output> FnMut<Input> for ClosureOnce<State, Input, Output>
where
    State: Copy
{
    extern "rust-call" fn call_mut(&mut self, i: Input) -> Output {
        let Self { f, t } = *self;
        f(t, i)
    }
}

#[cfg(feature="nightly")]
impl<'a, State, Input, Output> Fn<Input> for Closure<'a, State, Input, Output> {
    extern "rust-call" fn call(&self, i: Input) -> Output {
        let Self { f, t } = self;
        f(t, i)
    }
}
#[cfg(feature="nightly")]
impl<State, Input, Output> Fn<Input> for ClosureRef<State, Input, Output> {
    extern "rust-call" fn call(&self, i: Input) -> Output {
        let Self { f, t } = self;
        f(t, i)
    }
}
//This can be done; but will cause confusion, as it is not mutating itself
// when it supposed to be so
//#[cfg(feature="nightly")]
//impl<State, Input, Output> Fn<Input> for ClosureRefMut<State, Input, Output>
//where
//    State: Copy
//{
//    extern "rust-call" fn call(&self, i: Input) -> Output {
//        let Self { f, mut t } = *self;
//        f(&mut t, i)
//    }
//}
#[cfg(feature="nightly")]
impl<State, Input, Output> Fn<Input> for ClosureOnce<State, Input, Output>
where
    State: Copy
{
    extern "rust-call" fn call(&self, i: Input) -> Output {
        let Self { f, t } = *self;
        f(t, i)
    }
}
