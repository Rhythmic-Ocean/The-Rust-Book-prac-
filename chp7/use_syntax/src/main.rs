pb mod a{
    pub mod series{
        pub mod of{
            pub fn nested_modules(){}
        }
    }
}

use a::series::of;

use a::series::of::nested_modules;


fn main() {

    let here = of::nested_modules();
    let there = nested_modules();

}

//we can also use "use" syntax for enums cuz they require namespace too

enum Traf{
    Red,
    Blue
}
use Traf::Red;

fn some_fnc(){
    let red = Red;
}

//use * to access all the variants/ mods/ (items) globally


use Traf::*

fn some_other_fnc(){
    let blue = Blue;
}