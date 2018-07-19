/// Nameable closures
/// When called, it only refers to its state.
/// Correspond to unnamable closures: 
/// ```ignore
/// move |...| { /*only refers captured variables*/ }
/// ```
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
pub struct Closure<State,Input,Output> {
    f: fn(&State,Input) -> Output,
    t: State
}
impl<State,Input,Output> Copy for Closure<State,Input,Output>
where 
    State: Copy {}
impl<State,Input,Output> Clone for Closure<State,Input,Output>
where
    State: Clone
{
    fn clone(&self) -> Closure<State,Input,Output> {
        Closure::new(self.f, self.t.clone())
    }
}
///A variant that does not own its state
///Correspond to unnamable closures: 
/// ```ignore
/// |...| { /* only refers captured variables */ }
/// ```
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
pub struct ClosureRef<'a,State,Input,Output>
where
    State: 'a
{
    f: fn(&State,Input) -> Output,
    t: &'a State
}
impl<'a,State,Input,Output> Copy for ClosureRef<'a,State,Input,Output> {}
impl<'a,State,Input,Output> Clone for ClosureRef<'a,State,Input,Output>
{
    fn clone(&self) -> ClosureRef<'a,State,Input,Output> {
        *self
    }
}
///When called, it is allow to mutate its state.
///Correspond to unnamable closures: 
/// ```ignore
/// move |...| { /* mutates captured variables */ }
/// ```
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
pub struct ClosureMut<State,Input,Output> {
    f: fn(&mut State,Input) -> Output,
    t: State
}
impl<State,Input,Output> Copy for ClosureMut<State,Input,Output>
where 
    State: Copy {}
impl<State,Input,Output> Clone for ClosureMut<State,Input,Output>
where
    State: Clone
{
    fn clone(&self) -> ClosureMut<State,Input,Output> {
        ClosureMut::new(self.f, self.t.clone())
    }
}
///A variant that does not own its state
///Note it is not possible to derive Copy or Clone
///Correspond to unnamable closures: 
/// ```ignore
/// |...| { /*mutates captured variables */ }
/// ```
/// 
/// Example:
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
pub struct ClosureRefMut<'a,State,Input,Output>
where
    State: 'a
{
    f: fn(&mut State,Input) -> Output,
    t: &'a mut State
}
///When called, it consumes its state. So it can only be
///called once.
/// 
/// To create, use `closure_once!` and `closure_once_mut!` macros.
/// The only difference is that in `closure_once_mut!` the state
/// variable will be declared with the `mut` keyword, so you can
/// mutate it inside the body.
/// 
///Correspond to unnamable closures: 
/// ```ignore
/// move |...| { /*consumes captured variables */ }
/// ```
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
/// # struct MyStream();
/// # impl MyStream{ 
/// #   fn new() -> MyStream { MyStream() }
/// #   fn write_all(&mut self, _:&[u8]) {}
/// #   fn read_exact(&mut self, _:&mut [u8], _:usize) {}
/// # }
/// let send_data = closure_once_mut!(stream, data, stream.write_all(data), MyStream::new());
/// let read_data = closure_once_mut!(stream, (buf,len), stream.read_exact(buf, len), MyStream::new());
/// ```
pub struct ClosureOnce<State,Input,Output> {
    f: fn(State,Input) -> Output,
    t: State
}
impl<State,Input,Output> Copy for ClosureOnce<State,Input,Output>
where 
    State: Copy {}
impl<State,Input,Output> Clone for ClosureOnce<State,Input,Output>
where
    State: Clone
{
    fn clone(&self) -> ClosureOnce<State,Input,Output> {
        ClosureOnce::new(self.f, self.t.clone())
    }
}

impl<State,Input,Output> Closure<State,Input,Output> {
    pub fn new(f:fn(&State,Input) -> Output, t:State) -> Closure<State,Input,Output> {
        Closure{f,t}
    }
}
impl<'a,State,Input,Output> ClosureRef<'a,State,Input,Output> {
    pub fn new(f:fn(&State,Input) -> Output, t:&'a State) -> ClosureRef<'a,State,Input,Output>
    where
        State: 'a
    {
        ClosureRef{f,t}
    }
}
impl<State,Input,Output> ClosureMut<State,Input,Output> {
    pub fn new(f:fn(&mut State,Input) -> Output, t:State) -> ClosureMut<State,Input,Output> {
        ClosureMut{f,t}
    }
    pub fn new_ref<'a>(f:fn(&mut State,Input) -> Output, t:&'a mut State) -> ClosureRefMut<'a,State,Input,Output> {
        ClosureRefMut{f,t}
    }
}
impl<State,Input,Output> ClosureOnce<State,Input,Output> {
    pub fn new(f:fn(State,Input) -> Output, t:State) -> ClosureOnce<State,Input,Output> {
        ClosureOnce{f,t}
    }
}

//All Closures implements `FnOnce`
impl<State,Input,Output> FnOnce<Input> for Closure<State,Input,Output> {
    type Output=Output;
    extern "rust-call" fn call_once(self,i:Input) -> Output {
        let Closure{f,t} = self;
        f(&t,i)
    }
}
impl<'a,State,Input,Output> FnOnce<Input> for ClosureRef<'a,State,Input,Output> {
    type Output=Output;
    extern "rust-call" fn call_once(self,i:Input) -> Output {
        let ClosureRef{f,t} = self;
        f(&t,i)
    }
}
impl<State,Input,Output> FnOnce<Input> for ClosureMut<State,Input,Output> {
    type Output=Output;
    extern "rust-call" fn call_once(self,i:Input) -> Output {
        let ClosureMut{f,mut t} = self;
        f(&mut t,i)
    }
}
impl<'a,State,Input,Output> FnOnce<Input> for ClosureRefMut<'a,State,Input,Output> {
    type Output=Output;
    extern "rust-call" fn call_once(self,i:Input) -> Output {
        let ClosureRefMut{f,mut t} = self;
        f(&mut t,i)
    }
}
impl<State,Input,Output> FnOnce<Input> for ClosureOnce<State,Input,Output> {
    type Output=Output;
    extern "rust-call" fn call_once(self,i:Input) -> Output {
        let ClosureOnce{f,t} = self;
        f(t,i)
    }
}

//All closures except ClosureOnce implements FnMut
impl<State,Input,Output> FnMut<Input> for Closure<State,Input,Output> {
    extern "rust-call" fn call_mut(&mut self,i:Input) -> Output {
        let Closure{f,t} = self;
        f(t,i)
    }
}
impl<'a,State,Input,Output> FnMut<Input> for ClosureRef<'a,State,Input,Output> {
    extern "rust-call" fn call_mut(&mut self,i:Input) -> Output {
        let ClosureRef{f,t} = self;
        f(t,i)
    }
}
impl<State,Input,Output> FnMut<Input> for ClosureMut<State,Input,Output> {
    extern "rust-call" fn call_mut(&mut self,i:Input) -> Output {
        let ClosureMut{ref f,ref mut t} = self;
        f(t,i)
    }
}
impl<'a,State,Input,Output> FnMut<Input> for ClosureRefMut<'a,State,Input,Output> {
    extern "rust-call" fn call_mut(&mut self,i:Input) -> Output {
        let ClosureRefMut{ref f,ref mut t} = self;
        f(t,i)
    }
}

//Only Closure and ClosureRef implements Fn
impl<State,Input,Output> Fn<Input> for Closure<State,Input,Output> {
    extern "rust-call" fn call(&self,i:Input) -> Output {
        let Closure{f,t} = self;
        f(t,i)
    }
}
impl<'a,State,Input,Output> Fn<Input> for ClosureRef<'a,State,Input,Output> {
    extern "rust-call" fn call(&self,i:Input) -> Output {
        let ClosureRef{f,t} = self;
        f(t,i)
    }
}
