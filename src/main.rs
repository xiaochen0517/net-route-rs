fn main() {
    net_route_rs_lib::run().unwrap_or_else(|err| {
        eprintln!("{}", err.message);
        // 只有在开发模式下才打印堆栈跟踪
        #[cfg(debug_assertions)]
        {
            eprintln!("堆栈跟踪: {}", err.backtrace);
        }
        std::process::exit(1);
    });
}
