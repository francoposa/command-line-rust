use catr::get_args;

fn main() {
    if let Err(e) = get_args().and_then(catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
