mod outermost{
    pub fn middle_function(){}

    fn middle_sec(){}

    mod inside{
        pub fn inner_function(){}

        fn inner_sec(){}
    }
}

fn try_me(){
    outermost::middle_function();
    outermost::middle_sec();
    outermost::inside::inner_function();
    outermost::inside::inner_sec();
}

//if an item is private it can only be accessed by it's direct parent module, and the parent's other child module
//if  an item is public it can be accessed through any of it's parent module