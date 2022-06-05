enum IpAddrKind {
    V4(u32, u32, u32, u32),
    V6(String),
}


struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    //
    // route(four);
    // route(six);
    //
    // route(IpAddrKind::V4)


    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

}


fn route(ip_kind: IpAddrKind) {}