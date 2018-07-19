// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(fn_traits)]
#![feature(unboxed_closures)]

//! This crate supports nameable closures, without requiring any language
//! changes.
//! 
//! # Why do we need this
//! 
//! The Rust language supports closures, however its concret types are
//! anonymous and developers cannot expect same size, and so not able to
//! put similar closures in containers that requires `Sized` constraint.
//! 
//! However, there is an exception: if a closure does not refer to its 
//! captured variables, it can be coerce to a `fn` type, which is a pointer
//! type and it is `Sized`.
//! 
//! This library extends this idea by requesting an additional `State` field,
//! which is just the tuple of the captured variables. So if two closure have
//! the same signiture for function calls and have the same state type, they
//! are considered the same type.
//! 
//! # How to use
//! 
//! There are 5 structures being defined, and they are correspond to different
//! use cases.
//! 
//! ## Closure
//! 
//! This struct works for closures that only intended to refer to the state field,
//! not modifying them nor owns them. However, it does own its state, so dropping 
//! this struct will drop its state as well.
//! 
//! This struct is created by a macro `closure!`.
//! 
//! Example:
//! 
//! ```rust
//! # #[macro_use] extern crate namable_closures;
//! # use namable_closures::Closure;
//! # struct Point{x:i32,y:i32}
//! # impl Point{ fn new(x:i32,y:i32) -> Point {Point{x:x,y:y}} } 
//! # fn main() {
//! let add_ten = closure!(state, i,  i+*state, 10i32);
//! add_ten(1i32);
//! let offset = closure!(state, (a,b), Point::new((*state).x+a,(*state).y+b), Point::new(10,20));
//! let p = offset(1i32,2i32);
//! # }
//! ```
//! 
//! ## ClosureRef
//! 
//! This structure works like the above, but it does not own its state.
//! 
//! This struct is created by a macro `closure_ref!`.
//! 
//! Example:
//! 
//! ```rust
//! # #[macro_use] extern crate namable_closures;
//! # use namable_closures::ClosureRef;
//! # struct Point{x:i32,y:i32}
//! # impl Point{ fn new(x:i32,y:i32) -> Point {Point{x:x,y:y}} } 
//! # fn main() {
//! let state = 10;
//! let add_ten = closure_ref!(state, i, i+10, &state);
//! add_ten(1i32);
//! let state = Point::new(10,20);
//! let offset = closure_ref!(state, (a,b), Point::new(state.x+a,state.x+b), &state);
//! let p = offset(1i32,2i32);
//! # }
//! ```
//! 
//! ## ClosureMut
//! 
//! This struct works for closures that will mutate its state, but would not drop its state
//! when called. So it can be called multiple times, but will have different effects on each
//! call. Because it still owns its state, the state will be dropped when the struct was dropped.
//! 
//! This struct is created by a macro `closure_mut!`.
//! 
//! Example:
//! 
//! ```rust
//! # #[macro_use] extern crate namable_closures;
//! # use namable_closures::ClosureMut;
//! let mut accumulate = closure_mut!(state, c, {*state+=c;*state}, 0);
//! assert_eq!(accumulate(1),1); 
//! assert_eq!(accumulate(2),3);
//! ```
//! 
//! ## ClosureRefMut
//! 
//! This struct works like the above, but it does not own its state. Furethermore, this is the
//! only struct in this serial that does not support `Copy` and `Clone` at all, because it owns
//! a mutable reference to its state.
//! 
//! This struct is created by a macro `closure_ref_mut!`.
//!
//!  Example:
//! 
//! ```rust
//! # #[macro_use] extern crate namable_closures;
//! # use namable_closures::ClosureMut;
//! let mut state = 0;
//! {
//!   let mut match_cnt = closure_ref_mut!(state, (a,b), {if a==b { *state+=1 }}, &mut state);
//!   for i in 0..10 { match_cnt(i,i*3%10); }
//! }
//! assert_eq!(state,2);
//! ```
//! 
//! ## ClosureOnce
//! 
//! This struct owns its state, and will drop its state when called.
//! 
//! This struct is created by macros `closure_once!` and `closure_mut!`.
//! 
//! Example:
//! 
//! ```rust
//! # #[macro_use] extern crate namable_closures;
//! # use namable_closures::ClosureOnce;
//! # struct RoleSet();
//! # impl RoleSet { fn from_config() -> RoleSet { RoleSet() }}
//! # struct Passwd();
//! # impl Passwd { fn get_from_cache() -> Passwd { Passwd() }}
//! # fn authenticate(_:String,_:Passwd) {}
//! # fn check_user(_:RoleSet,_:String,_:Passwd) {}
//! let sign_on = closure_once!(passwd, user, authenticate(user,passwd), Passwd::get_from_cache());
//! let auth = closure_once!(role_set, (user,passwd), check_user(role_set,user,passwd), RoleSet::from_config());
//! # struct MyStream();
//! # impl MyStream{ 
//! #   fn new() -> MyStream { MyStream() }
//! #   fn write_all(&mut self, _:&[u8]) {}
//! #   fn read_exact(&mut self, _:&mut [u8], _:usize) {}
//! # }
//! let send_data = closure_once_mut!(stream, data, stream.write_all(data), MyStream::new());
//! let read_data = closure_once_mut!(stream, (buf,len), stream.read_exact(buf, len), MyStream::new());
//! ```

pub mod closures;

pub use closures::Closure;
pub use closures::ClosureRef;
pub use closures::ClosureMut;
pub use closures::ClosureRefMut;
pub use closures::ClosureOnce;

/// Create a nameable closure object with an immutable state
/// 
/// Example:
/// 
/// ```rust
/// # #[macro_use] extern crate namable_closures;
/// # use namable_closures::Closure;
/// # struct Point{x:i32,y:i32}
/// # impl Point{ fn new(x:i32,y:i32) -> Point {Point{x:x,y:y}} } 
/// # fn main() {
/// let add_ten = closure!(state, i,  i+*state, 10i32);
/// add_ten(1i32);
/// let offset = closure!(state, (a,b), Point::new((*state).x+a,(*state).y+b), Point::new(10,20));
/// let p = offset(1i32,2i32);
/// # }
/// ```
#[macro_export]
macro_rules! closure {
    ($state_var:ident,$arg:ident,$body:expr,$state_val:expr) => {
        Closure::new(|$state_var,($arg,)| $body, $state_val)
    };
    ($state_var:ident,$args:pat,$body:expr,$state_val:expr) => {
        ::namable_closures::Closure::new(|$state_var,$args| $body, $state_val)
    };
}
/// Create a nameable closure object refers to an immutable state
/// 
/// Example:
/// 
/// ```rust
/// # #[macro_use] extern crate namable_closures;
/// # use namable_closures::ClosureRef;
/// # struct Point{x:i32,y:i32}
/// # impl Point{ fn new(x:i32,y:i32) -> Point {Point{x:x,y:y}} } 
/// # fn main() {
/// let state = 10;
/// let add_ten = closure_ref!(state, i, i+10, &state);
/// add_ten(1i32);
/// let state = Point::new(10,20);
/// let offset = closure_ref!(state, (a,b), Point::new(state.x+a,state.x+b), &state);
/// let p = offset(1i32,2i32);
/// # }
/// ```
#[macro_export]
macro_rules! closure_ref {
    ($state_var:ident,$arg:ident,$body:expr,$state_val:expr) => {
        ClosureRef::new(|$state_var,($arg,)| $body, $state_val)
    };
    ($state_var:ident,$args:pat,$body:expr,$state_val:expr) => {
        ClosureRef::new(|$state_var,$args| $body, $state_val)
    };
}
/// Create a nameable closure object with a mutable state
/// 
/// Example:
/// 
/// ```rust
/// # #[macro_use] extern crate namable_closures;
/// # use namable_closures::ClosureMut;
/// let mut accumulate = closure_mut!(state, c, {*state+=c;*state}, 0);
/// assert_eq!(accumulate(1),1); 
/// assert_eq!(accumulate(2),3);
/// ```
#[macro_export]
macro_rules! closure_mut {
    ($state_var:ident,$arg:ident,$body:expr,$state_val:expr) => {
        ClosureMut::new(|$state_var,($arg,)| $body, $state_val)
    };
    ($state_var:ident,$args:pat,$body:expr,$state_val:expr) => {
        ClosureMut::new(|$state_var,$args| $body, t:$state_val)
    };
}
/// Create a nameable closure object refer to a mutable state
/// 
///  Example:
/// 
/// ```rust
/// # #[macro_use] extern crate namable_closures;
/// # use namable_closures::ClosureMut;
/// let mut state = 0;
/// {
///   let mut match_cnt = closure_ref_mut!(state, (a,b), {if a==b { *state+=1 }}, &mut state);
///   for i in 0..10 { match_cnt(i,i*3%10); }
/// }
/// assert_eq!(state,2);
/// ```
#[macro_export]
macro_rules! closure_ref_mut {
    ($state_var:ident,$arg:ident,$body:expr,$state_val:expr) => {
        ClosureMut::new_ref(|$state_var,($arg,)| $body, $state_val)
    };
    ($state_var:ident,$args:pat,$body:expr,$state_val:expr) => {
        ClosureMut::new_ref(|$state_var,$args| $body, $state_val)
    };
}
/// Create a nameable closure object owns a state
/// 
/// Example:
/// 
/// ```rust
/// # #[macro_use] extern crate namable_closures;
/// # use namable_closures::ClosureOnce;
/// # struct RoleSet();
/// # impl RoleSet { fn from_config() -> RoleSet { RoleSet() }}
/// # struct Passwd();
/// # impl Passwd { fn get_from_cache() -> Passwd { Passwd() }}
/// # fn authenticate(_:String,_:Passwd) {}
/// # fn check_user(_:RoleSet,_:String,_:Passwd) {}
/// let sign_on = closure_once!(passwd, user, authenticate(user,passwd), Passwd::get_from_cache());
/// let auth = closure_once!(role_set, (user,passwd), check_user(role_set,user,passwd), RoleSet::from_config());
/// ```
#[macro_export]
macro_rules! closure_once {
    ($state_var:ident,$arg:ident,$body:expr,$state_val:expr) => {
        ClosureOnce::new(|$state_var,($arg,)| $body, $state_val)
    };
    ($state_var:ident,$args:pat,$body:expr,$state_val:expr) => {
        ClosureOnce::new(|$state_var,$args| $body, $state_val)
    };
}
/// Create a nameable closure object owns a mutable state
/// 
/// Example:
/// 
/// ```rust
/// # #[macro_use] extern crate namable_closures;
/// # use namable_closures::ClosureOnce;
/// # struct MyStream();
/// # impl MyStream{ 
/// #   fn new() -> MyStream { MyStream() }
/// #   fn write_all(&mut self, _:&[u8]) {}
/// #   fn read_exact(&mut self, _:&mut [u8], _:usize) {}
/// # }
/// let send_data = closure_once_mut!(stream, data, stream.write_all(data), MyStream::new());
/// let read_data = closure_once_mut!(stream, (buf,len), stream.read_exact(buf, len), MyStream::new());
/// ```
#[macro_export]
macro_rules! closure_once_mut {
    ($state_var:ident,$arg:ident,$body:expr,$state_val:expr) => {
        ClosureOnce::new(|mut $state_var,($arg,)| $body, $state_val)
    };
    ($state_var:ident,$args:pat,$body:expr,$state_val:expr) => {
        ClosureOnce::new(|mut $state_var,$args| $body, $state_val)
    };
}

#[cfg(test)]
mod tests {
    use {Closure,ClosureRef,ClosureMut,ClosureRefMut,ClosureOnce};

    #[test]
    fn test_closure_copy_clone() {        
        let c = closure!(a, b, *a+b, 10);
        let copied = c;
        let cloned = c.clone();
        assert_eq!(c(20),30);
        assert_eq!(copied(20),c(20));
        assert_eq!(cloned(20),c(20));
    }
    #[test]
    fn test_closure_ref_copy_clone() {
        let mut v=10;
        {
            let c = closure_ref!(a, b, *a+b, &v);
            let copied = c;
            let cloned = c.clone();
            assert_eq!(c(20),30);
            assert_eq!(copied(20),c(20));
            assert_eq!(cloned(20),c(20));
        }
        v=20;
        let c = closure_ref!(a, b, *a+b, &v);
        let copied = c;
        let cloned = c.clone();
        assert_eq!(c(20),40);
        assert_eq!(copied(20),c(20));
        assert_eq!(cloned(20),c(20));
    }
    #[test]
    fn test_closure_mut_copy_clone() {
        let mut c = closure_mut!(a, b, {*a+=b;*a}, 10);
        let mut copied = c;
        let mut cloned = c.clone();
        assert_eq!(c(20),30);
        assert_eq!(c(20),50);
        assert_eq!(copied(20),30);
        assert_eq!(copied(20),50);
        assert_eq!(cloned(20),30);
        assert_eq!(cloned(20),50);
    }
    #[test]
    fn test_closure_ref_mut() {
        let mut v = 10;
        let mut c = closure_ref_mut!(a, b, {*a+=b;*a}, &mut v);
        assert_eq!(c(20),30);
        assert_eq!(c(20),50);
    }
    #[test]
    fn test_closure_once_copy_clone() {
        let c = closure_once!(a, b, {a+b}, 10);
        let copied = c;
        let cloned = c.clone();
        assert_eq!(c(20),30);
        assert_eq!(c(20),30);
        assert_eq!(copied(20),30);
        assert_eq!(copied(20),30);
        assert_eq!(cloned(20),30);
        assert_eq!(cloned(20),30);
    }
}