/// Replicates the Fn traits for stable build
pub trait StableFnOnce<Input> {
    type Output;
    fn stable_call_once(self,args:Input) -> Self::Output;
}
/// Replicates the Fn traits for stable build
pub trait StableFnMut<Input>: StableFnOnce<Input> {
    fn stable_call_mut(&mut self,args:Input) -> Self::Output;
}
/// Replicates the Fn traits for stable build
pub trait StableFn<Input>:StableFnMut<Input> {
    fn stable_call(&self,args:Input) -> Self::Output;
}


pub fn as_cloning_stable_fn<Input,Output>(f: impl StableFnOnce<Input,Output=Output> + Clone)
    -> impl StableFn<Input,Output=Output>
{
    struct Wrapper<T>(T);
    impl<Input,Output,T> StableFnOnce<Input> for Wrapper<T>
    where
        T: StableFnOnce<Input,Output=Output>
    {
        type Output = Output;
        fn stable_call_once(self, args:Input) -> Output {
            let Wrapper(t) = self;
            t.stable_call_once(args)
        }
    }
    impl<Input,Output,T> StableFnMut<Input> for Wrapper<T>
    where
        T: StableFnOnce<Input,Output=Output> + Clone
    {
        fn stable_call_mut(&mut self, args:Input) -> Output {
            let Wrapper(t) = self;
            t.clone().stable_call_once(args)
        }
    }
    impl<Input,Output,T> StableFn<Input> for Wrapper<T>
    where
        T: StableFnOnce<Input,Output=Output> + Clone
    {
        fn stable_call(&self, args:Input) -> Output {
            let Wrapper(t) = self;
            t.clone().stable_call_once(args)
        }
    }
    Wrapper(f)
}
#[cfg(feature="nightly")]
pub fn as_cloning_fn<Input,Output>(f: impl FnOnce<Input,Output=Output> + Clone)
    -> impl Fn<Input,Output=Output>
{
    struct Wrapper<T>(T);
    impl<Input,Output,T> FnOnce<Input> for Wrapper<T>
    where
        T: FnOnce<Input,Output=Output>
    {
        type Output = Output;
        extern "rust-call" fn call_once(self, args:Input) -> Output {
            let Wrapper(t) = self;
            t.call_once(args)
        }
    }
    impl<Input,Output,T> FnMut<Input> for Wrapper<T>
    where
        T: FnOnce<Input,Output=Output> + Clone
    {
        extern "rust-call" fn call_mut(&mut self, args:Input) -> Output {
            let Wrapper(t) = self;
            t.clone().call_once(args)
        }
    }
    impl<Input,Output,T> StableFn<Input> for Wrapper<T>
    where
        T: StableFnOnce<Input,Output=Output> + Clone
    {
        extern "rust-call" fn call(&self, args:Input) -> Output {
            let Wrapper(t) = self;
            t.clone().call_once(args)
        }
    }
    Wrapper(f)
}