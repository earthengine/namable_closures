#![doc="
|a     |b|
|------|-|
|`|v|e`|e|
"]

use stable_fn::{StableFn,StableFnMut,StableFnOnce};

struct ClosureRec<State,Input,Output> {
    func: fn(&ClosureRec<State,Input,Output>, Input) -> Output,
    state: State
}
impl<State,Input,Output> Copy for ClosureRec<State,Input,Output>
where
    State: Copy{}
impl<State,Input,Output> Clone for ClosureRec<State,Input,Output>
where
    State: Clone
{
    fn clone(&self) -> Self {
        let Self { func, state} = self;
        Self { func:*func, state:state.clone() }
    }
}
impl<State,Input,Output> ClosureRec<State,Input,Output> {
    pub fn new(func: fn(&Self, Input) -> Output, s: State) -> Self {
        Self { func: func, state: s}
    }
    pub fn call_with_state(&self, s:State, i:Input) -> Output {
        (self.func)(&Self::new(self.func, s), i)
    }
}

struct ClosureMutRec<State,Input,Output> {
    func: fn(&mut ClosureMutRec<State,Input,Output>, Input) -> Output,
    state: State
}
impl<State,Input,Output> Copy for ClosureMutRec<State,Input,Output>
where
    State: Copy{}
impl<State,Input,Output> Clone for ClosureMutRec<State,Input,Output>
where
    State: Clone
{
    fn clone(&self) -> Self {
        let Self { func, state} = self;
        Self { func:*func, state:state.clone() }
    }
}
impl<State,Input,Output> ClosureMutRec<State,Input,Output> {
    pub fn new(func: fn(&mut Self, Input) -> Output, s: State) -> Self {
        Self { func: func, state: s}
    }
    pub fn call_with_state(&self, s:State, i:Input) -> Output {
        (self.func)(&mut Self::new(self.func, s), i)
    }
}

struct ClosureRecMut<'a, State,Input,Output>
where
    State: 'a
{
    func: fn(&mut ClosureRecMut<'a, State,Input,Output>, Input) -> Output,
    state: &'a mut State
}
impl<'a, State,Input,Output> ClosureRecMut<'a, State,Input,Output> {
    pub fn new(func: fn(&mut ClosureRecMut<'a, State,Input,Output>, Input) -> Output, s: &'a mut State) -> Self {
        Self { func: func, state: s}
    }
}

struct ClosureOnceRec<State,Input,Output> {
    func: fn(ClosureOnceRec<State,Input,Output>, Input) -> Output,
    state: State
}
impl<State,Input,Output> Copy for ClosureOnceRec<State,Input,Output>
where
    State: Copy{}
impl<State,Input,Output> Clone for ClosureOnceRec<State,Input,Output>
where
    State: Clone
{
    fn clone(&self) -> Self {
        let Self { func, state} = self;
        Self { func:*func, state:state.clone() }
    }
}
impl<State,Input,Output> ClosureOnceRec<State,Input,Output> {
    pub fn new(func: fn(Self, Input) -> Output, s: State) -> Self {
        Self { func: func, state: s}
    }
    pub fn call_with_state(&self, s:State, i:Input) -> Output {
        (self.func)(Self::new(self.func, s), i)
    }
}

impl<State,Input,Output> StableFnOnce<Input> for ClosureRec<State,Input,Output> {
    type Output=Output;
    fn stable_call_once(self, i:Input) -> Self::Output {
        (self.func)(&self, i)
    }
}
impl<State,Input,Output> StableFnMut<Input> for ClosureRec<State,Input,Output> {
    fn stable_call_mut(&mut self, i:Input) -> Output {
        (self.func)(self, i)
    }
}
impl<State,Input,Output> StableFn<Input> for ClosureRec<State,Input,Output> {
    fn stable_call(&self, i:Input) -> Output {
        (self.func)(self, i)
    }
}


impl<State,Input,Output> StableFnOnce<Input> for ClosureMutRec<State,Input,Output> {
    type Output=Output;
    fn stable_call_once(mut self, i:Input) -> Self::Output {
        (self.func)(&mut self, i)
    }
}
impl<State,Input,Output> StableFnMut<Input> for ClosureMutRec<State,Input,Output> {
    fn stable_call_mut(&mut self, i:Input) -> Output {
        (self.func)(self, i)
    }
}
impl<State,Input,Output> StableFn<Input> for ClosureMutRec<State,Input,Output>
where
    State: Copy
{
    fn stable_call(&self, i:Input) -> Output {
        let mut s = *self;
        (s.func)(&mut s, i)
    }
}

impl<'a,State,Input,Output> StableFnOnce<Input> for ClosureRecMut<'a,State,Input,Output> {
    type Output=Output;
    fn stable_call_once(mut self, i:Input) -> Self::Output {
        (self.func)(&mut self, i)
    }
}
impl<'a,State,Input,Output> StableFnMut<Input> for ClosureRecMut<'a,State,Input,Output> {
    fn stable_call_mut(&mut self, i:Input) -> Output {
        (self.func)(self, i)
    }
}

impl<State,Input,Output> StableFnOnce<Input> for ClosureOnceRec<State,Input,Output> {
    type Output=Output;
    fn stable_call_once(self, i:Input) -> Self::Output {
        (self.func)(self, i)
    }
}
impl<State,Input,Output> StableFnMut<Input> for ClosureOnceRec<State,Input,Output>
where
    State: Copy
{
    fn stable_call_mut(&mut self, i:Input) -> Output {
        (self.func)(*self, i)
    }
}
impl<State,Input,Output> StableFn<Input> for ClosureOnceRec<State,Input,Output>
where
    State: Copy
{
    fn stable_call(&self, i:Input) -> Output {
        (self.func)(*self, i)
    }
}

#[cfg(test)]
mod test {
    use closure_rec::{ClosureRec,ClosureMutRec};
    use stable_fn::{StableFn,StableFnMut};
    #[test]
    fn test_fac() {
        let fac:ClosureRec<(),(i32,),i32> = 
            closure_rec!(me.state=() => ref |i| 
                if i==0 {1} else {me.stable_call((i-1,)) * i}
            );
        assert_eq!(fac.stable_call((10,)),3628800);
    }
    #[test]
    fn test_fib() {
        let fib:ClosureRec<(i32,i32),(i32,),i32> = 
            closure_rec!(me.state=(1,1) => ref |i| {
                let (i0,i1) = me.state;
                match i {
                    0 => i0,
                    1 => i1,
                    n => me.call_with_state((i1,i0+i1), (i-1,))
                }                
            });        
        assert_eq!(fib.stable_call((10,)),89);
        assert_eq!(fib.state, (1,1));
    }
}