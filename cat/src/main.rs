fn main() {
    if let Err(e) = cat::Config::try_from_args().and_then(|config| config.run()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
