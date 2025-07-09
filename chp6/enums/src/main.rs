enum Ipaddtype{ // eg 1
    V4,
    V6
}

struct IP{ //eg 1
    kind: Ipaddtype,
    address: String
}


enum IpAddr{ //even better way, without struct  //eg 2
    V4(String),
    V6(String)
}

enum IpAddre{ //using enums allows us to customize each varient to have different data types too. So it's better than struct in this way
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {

    let home = IP{ //eg 2
        kind: Ipaddtype::V4,
        address: String::from("12:33:33")
    };

    let office = IP{ //eg 2
        kind: Ipaddtype::V6,
        address: String::from("::1")
    };


    let office2 = IpAddr::V6(String::from("::1"));


    let community = IpAddre::V4(11,12,13,14)l
}
