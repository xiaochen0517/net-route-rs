fn main() {
    net_route_rs_lib::run().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}
