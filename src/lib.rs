// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature="nightly",feature(fn_traits))]
#![cfg_attr(feature="nightly",feature(unboxed_closures))]

#![doc="
This crate supports nameable closures, without requiring any language
changes.

# Why do we need this

The Rust language supports closures, however its concret types are
anonymous and developers cannot expect same size, and so not able to
put similar closures in containers that requires `Sized` constraint.

However, there is an exception: if a closure does not refer to its
captured variables, it can be coerce to a `fn` type, which is a pointer
type and it is `Sized`.

This library extends this idea by requesting an additional `State` field,
which is just the tuple of the captured variables. So if two closure have
the same signiture for function calls and have the same state type, they
are considered the same type.

# How to use

There are 5 structures being defined, and they are correspond to different
use cases.

## Closure

This struct works for closures that only intended to refer to the state field,
not modifying them nor owns them. Furthermore, it does not own its state, 
so dropping this struct will not drop its state.

## ClosureRef

This structure works like the above, but it owns its state. So dropping it will
drop its state.

## ClosureMut

This struct works for closures that will mutate its state, but would not drop its state
when called. So it can be called multiple times, but will have different effects on each
call. Because it does not own its state, the state will not be dropped when the struct 
was dropped. 

Amount all 5 variants, this is the only struct that does not support `Copy` and `Clone` 
at all, because it is not possible to copy or clone a mutable reference.

## ClosureRefMut

This struct works like the above, but it owns its state. Because it owns its state, so it
can be `Copy` or `Clone` if its state is `Copy` or `Clone`, without problems.

## ClosureOnce

This struct owns its state, and will drop its state when called.

# The `closure!` macro

To create closures, use the `closure!` macro.
The format of the macro is:

```text
closure!([ref]? [mut]? state_variable=expression => [closure definition])
```

The closure definition is like the usual (including the `move` keyword), but type 
annotations are not supported. However, we are providing a type that is namable, 
so you can always specify the result type to constraint the variables involved, so 
this should not be a big due.

If the macro sees a `ref` keyword in front of the closure, it will expect a `move` 
keyword before the closure body. If `ref` is specified, the state variable is a reference
to its value.

If `move` is not specified, the state expression must be typed as a reference and match the 
mutation specification of the state variable. The closure body can only access the state
variable and the variables in the closure definition header.

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

Examples:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::ClosureRef;
# use namable_closures::StableFn;
# struct Point{x:i32,y:i32}
# impl Point{ fn new(x:i32,y:i32) -> Point {Point{x:x,y:y}} }
// state refered as reference in body, but moved to the closure
let add_ten:ClosureRef<i32,(i32,),i32>
    = closure!(ref state=10 => move |i| i+*state);
assert_eq!(add_ten.stable_call((1,)),11);
let offset:ClosureRef<Point,(i32,i32),Point>
    = closure!(ref state=Point::new(10,20) => move |a,b| Point::new(state.x+a,state.y+b));
let p = offset.stable_call((1,2));
assert_eq!(p.x,11);
assert_eq!(p.y,22);

# use namable_closures::Closure;
// state refered as reference in body, and not moving
let state = 10;
let add_ten:Closure<i32,(i32,),i32>
    = closure!(state=&state => |i| i+10);
assert_eq!(add_ten.stable_call((1,)),11);
let state = Point::new(10,20);
let offset:Closure<Point,(i32,i32),Point>
    = closure!(state=&state => |a,b| Point::new(state.x+a,state.y+b));
let p = offset.stable_call((1i32,2i32));
assert_eq!(p.x,11);
assert_eq!(p.y,22);

# use namable_closures::ClosureRefMut;
# use namable_closures::StableFnMut;
// state refered as mutable reference in body, but moved to closure
let mut accumulate:ClosureRefMut<i32,(i32,),i32>
    = closure!(ref mut state=0 => move |c| {*state+=c;*state});
assert_eq!(accumulate.stable_call_mut((1,)),1);
assert_eq!(accumulate.stable_call_mut((2,)),3);

# use namable_closures::ClosureMut; 
// state refered as mutable reference in body, but not moving
let mut state = 0;
{
  let mut match_cnt:ClosureMut<i32,(i32,i32),()>
      = closure!(mut state=&mut state => |a,b| if a==b { *state+=1 });
  for i in 0..10 { match_cnt.stable_call_mut((i,i*3%10)); }
}
assert_eq!(state,2);

# use namable_closures::ClosureOnce;
# use std::io;
# use namable_closures::StableFnOnce;
# struct MyStream();
# impl MyStream{
#   fn new() -> MyStream { MyStream() }
#   fn write_all(&mut self, _:&[u8]) -> Result<usize,io::Error> { Ok(0) }
#   fn read_exact_ex(&mut self, _:&mut [u8], _:usize) -> Result<(),io::Error> { Ok(()) }
# }
# struct RoleSet();
# impl RoleSet { fn from_config() -> RoleSet { RoleSet() }}
# struct Passwd();
# impl Passwd { fn get_from_cache() -> Passwd { Passwd() }}
# fn authenticate(_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
# fn check_user(_:RoleSet,_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
// state moved to body and so to the closure
let sign_on:ClosureOnce<Passwd,(String,),Result<(),io::Error>>
    = closure!(passwd=Passwd::get_from_cache() => move |user| authenticate(user,passwd));
sign_on.stable_call_once((\"123\".to_string(),));
let auth:ClosureOnce<RoleSet,(String,Passwd),Result<(),io::Error>>
    = closure!(role_set=RoleSet::from_config() => move |user,passwd| check_user(role_set,user,passwd));
let send_data:ClosureOnce<MyStream,(&[u8],),Result<usize,io::Error>>
    = closure!(mut stream=MyStream::new() => move |data| stream.write_all(data));
let read_data:ClosureOnce<MyStream,(&mut [u8],usize),Result<(),io::Error>>
    = closure!(mut stream=MyStream::new() => move |buf,len| stream.read_exact_ex(buf, len));
```
"]
#![cfg_attr(feature="nightly", doc="
The same examples that uses the unstable features:

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

# use namable_closures::Closure;
// state refered as reference in body, and not moving
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

# use namable_closures::ClosureRefMut;
// state refered as mutable reference in body, but moved to closure
let mut accumulate:ClosureRefMut<i32,(i32,),i32>
    = closure!(ref mut state=0 => move |c| {*state+=c;*state});
assert_eq!(accumulate(1),1);
assert_eq!(accumulate(2),3);

# use namable_closures::ClosureMut; 
// state refered as mutable reference in body, but not moving
let mut state = 0;
{
  let mut match_cnt:ClosureMut<i32,(i32,i32),()>
      = closure!(mut state=&mut state => |a,b| if a==b { *state+=1 });
  for i in 0..10 { match_cnt(i,i*3%10); }
}
assert_eq!(state,2);

# use namable_closures::ClosureOnce;
# use std::io;
# struct MyStream();
# impl MyStream{
#   fn new() -> MyStream { MyStream() }
#   fn write_all(&mut self, _:&[u8]) -> Result<usize,io::Error> { Ok(0) }
#   fn read_exact_ex(&mut self, _:&mut [u8], _:usize) -> Result<(),io::Error> { Ok(()) }
# }
# struct RoleSet();
# impl RoleSet { fn from_config() -> RoleSet { RoleSet() }}
# struct Passwd();
# impl Passwd { fn get_from_cache() -> Passwd { Passwd() }}
# fn authenticate(_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
# fn check_user(_:RoleSet,_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
// state moved to body and so to the closure
let sign_on:ClosureOnce<Passwd,(String,),Result<(),io::Error>>
    = closure!(passwd=Passwd::get_from_cache() => move |user| authenticate(user,passwd));
let auth:ClosureOnce<RoleSet,(String,Passwd),Result<(),io::Error>>
    = closure!(role_set=RoleSet::from_config() => move |user,passwd| check_user(role_set,user,passwd));
let send_data:ClosureOnce<MyStream,(&[u8],),Result<usize,io::Error>>
    = closure!(mut stream=MyStream::new() => move |data| stream.write_all(data));
let read_data:ClosureOnce<MyStream,(&mut [u8],usize),Result<(),io::Error>>
    = closure!(mut stream=MyStream::new() => move |buf,len| stream.read_exact_ex(buf, len));
```
")]

#[doc="
The macro to create closures.

Examples:

```rust
# #[macro_use] extern crate namable_closures;
# use namable_closures::ClosureRef;
# use namable_closures::StableFn;
# struct Point{x:i32,y:i32}
# impl Point{ fn new(x:i32,y:i32) -> Point {Point{x:x,y:y}} }
// state refered as reference in body, but moved to the closure
let add_ten:ClosureRef<i32,(i32,),i32>
    = closure!(ref state=10 => move |i| i+*state);
assert_eq!(add_ten.stable_call((1,)),11);
let offset:ClosureRef<Point,(i32,i32),Point>
    = closure!(ref state=Point::new(10,20) => move |a,b| Point::new(state.x+a,state.y+b));
let p = offset.stable_call_once((1,2));
assert_eq!(p.x,11);
assert_eq!(p.y,22);

# use namable_closures::Closure;
// state refered as reference in body, and not moving
let state = 10;
let add_ten:Closure<i32,(i32,),i32>
    = closure!(state=&state => |i| i+10);
assert_eq!(add_ten.stable_call((1,)),11);
let state = Point::new(10,20);
let offset:Closure<Point,(i32,i32),Point>
    = closure!(state=&state => |a,b| Point::new(state.x+a,state.y+b));
let p = offset.stable_call_once((1i32,2i32));
assert_eq!(p.x,11);
assert_eq!(p.y,22);

# use namable_closures::ClosureRefMut;
# use namable_closures::StableFnOnce;;
// state refered as mutable reference in body, but moved to closure
let mut accumulate:ClosureRefMut<i32,(i32,),i32>
    = closure!(ref mut state=0 => move |c| {*state+=c;*state});
assert_eq!(accumulate.stable_call_mut((1,)),1);
assert_eq!(accumulate.stable_call_once((2,)),3);

# use namable_closures::ClosureMut;
# use namable_closures::StableFnMut;
// state refered as mutable reference in body, but not moving
let mut state = 0;
{
  let mut match_cnt:ClosureMut<i32,(i32,i32),()>
      = closure!(mut state=&mut state => |a,b| if a==b { *state+=1 });
  for i in 0..10 { match_cnt.stable_call_mut((i,i*3%10)); }
}
assert_eq!(state,2);

# use namable_closures::ClosureOnce;
# use std::io;
# struct MyStream();
# impl MyStream{
#   fn new() -> MyStream { MyStream() }
#   fn write_all(&mut self, _:&[u8]) -> Result<usize,io::Error> { Ok(0) }
#   fn read_exact_ex(&mut self, _:&mut [u8], _:usize) -> Result<(),io::Error> { Ok(()) }
# }
# struct RoleSet();
# impl RoleSet { fn from_config() -> RoleSet { RoleSet() }}
# struct Passwd();
# impl Passwd { fn get_from_cache() -> Passwd { Passwd() }}
# fn authenticate(_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
# fn check_user(_:RoleSet,_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
// state moved to body and so to the closure
let sign_on:ClosureOnce<Passwd,(String,),Result<(),io::Error>>
    = closure!(passwd=Passwd::get_from_cache() => move |user| authenticate(user,passwd));
let auth:ClosureOnce<RoleSet,(String,Passwd),Result<(),io::Error>>
    = closure!(role_set=RoleSet::from_config() => move |user,passwd| check_user(role_set,user,passwd));
let send_data:ClosureOnce<MyStream,(&[u8],),Result<usize,io::Error>>
    = closure!(mut stream=MyStream::new() => move |data| stream.write_all(data));
let read_data:ClosureOnce<MyStream,(&mut [u8],usize),Result<(),io::Error>>
    = closure!(mut stream=MyStream::new() => move |buf,len| stream.read_exact_ex(buf, len));
```
"]
#[cfg_attr(feature="nightly", doc="
The same examples that uses unstable features:

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

# use namable_closures::Closure;
// state refered as reference in body, and not moving
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

# use namable_closures::ClosureRefMut;
// state refered as mutable reference in body, but moved to closure
let mut accumulate:ClosureRefMut<i32,(i32,),i32>
    = closure!(ref mut state=0 => move |c| {*state+=c;*state});
assert_eq!(accumulate(1),1);
assert_eq!(accumulate(2),3);

# use namable_closures::ClosureMut; 
// state refered as mutable reference in body, but not moving
let mut state = 0;
{
  let mut match_cnt:ClosureMut<i32,(i32,i32),()>
      = closure!(mut state=&mut state => |a,b| if a==b { *state+=1 });
  for i in 0..10 { match_cnt(i,i*3%10); }
}
assert_eq!(state,2);

# use namable_closures::ClosureOnce;
# use std::io;
# struct MyStream();
# impl MyStream{
#   fn new() -> MyStream { MyStream() }
#   fn write_all(&mut self, _:&[u8]) -> Result<usize,io::Error> { Ok(0) }
#   fn read_exact_ex(&mut self, _:&mut [u8], _:usize) -> Result<(),io::Error> { Ok(()) }
# }
# struct RoleSet();
# impl RoleSet { fn from_config() -> RoleSet { RoleSet() }}
# struct Passwd();
# impl Passwd { fn get_from_cache() -> Passwd { Passwd() }}
# fn authenticate(_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
# fn check_user(_:RoleSet,_:String,_:Passwd) -> Result<(),io::Error> { Ok(()) }
// state moved to body and so to the closure
let sign_on:ClosureOnce<Passwd,(String,),Result<(),io::Error>>
    = closure!(passwd=Passwd::get_from_cache() => move |user| authenticate(user,passwd));
sign_on(\"123\".to_string());
let auth:ClosureOnce<RoleSet,(String,Passwd),Result<(),io::Error>>
    = closure!(role_set=RoleSet::from_config() => move |user,passwd| check_user(role_set,user,passwd));
let send_data:ClosureOnce<MyStream,(&[u8],),Result<usize,io::Error>>
    = closure!(mut stream=MyStream::new() => move |data| stream.write_all(data));
send_data(&[1u8]);
let read_data:ClosureOnce<MyStream,(&mut [u8],usize),Result<(),io::Error>>
    = closure!(mut stream=MyStream::new() => move |buf,len| stream.read_exact_ex(buf, len));
```
")]
#[macro_export]
macro_rules! closure {
    ($state:ident=$state_val:expr => move || $body:expr) => {
        ClosureOnce::new(|$state,()| $body, $state_val)
    };
    ($state:ident=$state_val:expr => move |$arg:pat| $body:expr) => {
        ClosureOnce::new(|$state,($arg,)| $body, $state_val)
    };
    ($state:ident=$state_val:expr => move |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        ClosureOnce::new(|$state,($arg1,$($arg2),*)| $body, $state_val)
    };
    (mut $state:ident=$state_val:expr => move || $body:expr) => {
        ClosureOnce::new(|mut $state,()| $body, $state_val)
    };
    (mut $state:ident=$state_val:expr => move |$arg:pat| $body:expr) => {
        ClosureOnce::new(|mut $state,($arg,)| $body, $state_val)
    };
    (mut $state:ident=$state_val:expr => move |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        ClosureOnce::new(|mut $state,($arg1,$($arg2),*)| $body, $state_val)
    };
    (mut $state:ident=$state_val:expr => || $body:expr) => {
        ClosureMut::new(|$state,()| $body, $state_val)
    };
    (mut $state:ident=$state_val:expr => |$arg:pat| $body:expr) => {
        ClosureMut::new(|$state,($arg,)| $body, $state_val)
    };
    (mut $state:ident=$state_val:expr => |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        ClosureMut::new(|$state,($arg1,$($arg2),*)| $body, $state_val)
    };
    (ref mut $state:ident=$state_val:expr => move || $body:expr) => {
        ClosureRefMut::new(|$state,()| $body, $state_val)
    };
    (ref mut $state:ident=$state_val:expr => move |$arg:pat| $body:expr) => {
        ClosureRefMut::new(|$state,($arg,)| $body, $state_val)
    };
    (ref mut $state:ident=$state_val:expr => move |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        ClosureRefMut::new(|$state,($arg1,$($arg2),*)| $body, $state_val)
    };
    ($state:ident=$state_val:expr => || $body:expr) => {
        Closure::new(|$state,()| $body, $state_val)
    };
    ($state:ident=$state_val:expr => |$arg:pat| $body:expr) => {
        Closure::new(|$state,($arg,)| $body, $state_val)
    };
    ($state:ident=$state_val:expr => |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        Closure::new(|$state,($arg1,$($arg2),*)| $body, $state_val)
    };
    (ref $state:ident=$state_val:expr => move || $body:expr) => {
        ClosureRef::new(|$state,()| $body, $state_val)
    };
    (ref $state:ident=$state_val:expr => move |$arg:pat| $body:expr) => {
        ClosureRef::new(|$state,($arg,)| $body, $state_val)
    };
    (ref $state:ident=$state_val:expr => move |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        ClosureRef::new(|$state,($arg1,$($arg2),*)| $body, $state_val)
    };
    (ref $state:ident=$state_val:expr => |$($arg:pat),*| $body:expr) => {
        compile_error!("Use of ref keyword require move keyword for the closure body")
    };
    (ref mut $state:ident=$state_val:expr => |$($arg:pat),*| $body:expr) => {
        compile_error!("Use of ref keyword require move keyword for the closure body")
    };
}

#[macro_export]
macro_rules! closure_rec {
    ($me:ident.state=$state_val:expr => || $body:expr) => {
        ClosureOnceRec::new(|$me,()| $body, $state_val)
    };
    ($me:ident.state=$state_val:expr => |$arg:pat| $body:expr) => {
        ClosureOnceRec::new(|$me,($arg,)| $body, $state_val)
    };
    ($me:ident.state=$state_val:expr => |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        ClosureOnceRec::new(|$me,($arg1,$($arg2),*)| $body, $state_val)
    };
    (mut $me:ident.state=$state_val:expr => || $body:expr) => {
        ClosureOnceRec::new(|mut $me,()| $body, $state_val)
    };
    (mut $me:ident.state=$state_val:expr => |$arg:pat| $body:expr) => {
        ClosureOnceRec::new(|mut $me,($arg,)| $body, $state_val)
    };
    (mut $me:ident.state=$state_val:expr => |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        ClosureOnceRec::new(|mut $me,($arg1,$($arg2),*)| $body, $state_val)
    };
    (mut $me:ident.state=$state_val:expr => mut || $body:expr) => {
        ClosureMutRec::new(|$me,()| $body, $state_val)
    };
    (mut $me:ident.state=$state_val:expr => mut |$arg:pat| $body:expr) => {
        ClosureMutRec::new(|$me,($arg,)| $body, $state_val)
    };
    (mut $me:ident.state=$state_val:expr => mut |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        ClosureMutRec::new(|$me,($arg1,$($arg2),*)| $body, $state_val)
    };
    ($me:ident.state=$state_val:expr => mut || $body:expr) => {
        ClosureRecMut::new(|$me,()| $body, $state_val)
    };
    ($me:ident.state=$state_val:expr => mut |$arg:pat| $body:expr) => {
        ClosureRecMut::new(|$me,($arg,)| $body, $state_val)
    };
    ($me:ident.state=$state_val:expr => mut |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        ClosureRecMut::new(|$me,($arg1,$($arg2),*)| $body, $state_val)
    };
    ($me:ident.state=$state_val:expr => ref || $body:expr) => {
        ClosureRec::new(|$me,()| $body, $state_val)
    };
    ($me:ident.state=$state_val:expr => ref |$arg:pat| $body:expr) => {
        ClosureRec::new(|$me,($arg,)| $body, $state_val)
    };
    ($me:ident.state=$state_val:expr => ref |$arg1:pat,$($arg2:pat),+| $body:expr) => {
        ClosureRec::new(|$me,($arg1,$($arg2),*)| $body, $state_val)
    };
}

#[macro_export]
macro_rules! call {
    (ref $c:ident ()) => {
        $c.ident.stable_call(())
    };
    (ref $c:ident ($arg:expr)) => {
        $c.stable_call(($arg,))
    };
    (ref $c:ident ($arg1:expr,$($arg2:expr),+)) => {
        $c.stable_call(($arg1,$($arg2),*))
    };
    (mut $c:ident ()) => {
        $c.stable_call_mut(())
    };
    (mut $c:ident ($arg:expr)) => {
        $c.stable_call_mut(($arg,))
    };
    (mut $c:ident ($arg1:expr,$($arg2:expr),+)) => {
        $c.stable_call_mut(($arg1,$($arg2),*))
    };
    ($c:ident ()) => {
        $c.stable_call_once(())
    };
    ($c:ident ($arg:expr)) => {
        $c.stable_call_once(($arg,))
    };
    ($c:ident ($arg1:expr,$($arg2:expr),+)) => {
        $c.stable_call_once(($arg1,$($arg2),*))
    };
}

#[macro_export]
macro_rules! regulate {
    (|| ref $c:ident) => {
        || $c.stable_call(())
    };
    (|$arg:ident| ref $c:ident) => {
        |$arg| $c.stable_call(($arg,))
    };
    (|$arg1:ident,$($arg2:ident),+| ref $c:ident) => {
        |$arg1,$($arg2),*| $c.stable_call(($arg1,$($arg2),*))
    };
    (|| mut $c:ident) => {
        || $c.stable_call_mut(())
    };
    (|$arg:ident| mut $c:ident) => {
        |$arg| $c.stable_call_mut(($arg,))
    };
    (|$arg1:ident,$($arg2:ident),+| mut $c:ident) => {
        |$arg1,$($arg2),*| $c:stable_call_mut(($arg1,$($arg2),*))
    };
    (|| $c:ident) => {
        || $c.stable_call_once(())
    };
    (|$arg:ident| $c:ident) => {
        |$arg| $c.stable_call_once(($arg,))
    };
    (|$arg1:ident,$($arg2:ident),+| $c:ident) => {
        |$arg1,$($arg2),*| $c.stable_call_once(($arg1,$($arg2),*))
    };
}

pub mod closures;
pub mod closure_rec;
pub mod stable_fn;

pub use closures::{Closure,ClosureMut,ClosureOnce,ClosureRef,ClosureRefMut};
pub use stable_fn::{StableFn,StableFnMut,StableFnOnce};
pub use closure_rec::{ClosureOnceRec,ClosureRecMut,ClosureMutRec,ClosureRec};