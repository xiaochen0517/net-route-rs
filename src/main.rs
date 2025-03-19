fn main() {
    net_route_rs_lib::run().unwrap_or_else(|err| {
        eprintln!("错误: {}", err.message);
        eprintln!("堆栈跟踪: {}", err.backtrace);
        std::process::exit(1);
    });
}
