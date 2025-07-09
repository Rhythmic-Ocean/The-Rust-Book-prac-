pub mod client;//telling rust to go look into client.rs for relevent modules

//by default the compiler looks in lib.rs, u have to manually tell it where else to look like above.

pub mod network;

#[cfg(test)]
mod tests {
    use crate::client; //or super::client; works just as fine, super:: is relative pathing, crate:: is abs pathing
    #[test]
    fn it_works() {
        client::connect();
    }
}

//crate:: starts looking from the root, which is the folder contating scr for main.rs /lib.rs
