fn main() {
    if let Err(e) = head::Cli::default().run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
