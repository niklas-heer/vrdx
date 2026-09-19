//! Human and JSON command-line entry point.

fn main() -> std::process::ExitCode {
    vrdx::records::cli::run(std::env::args_os().skip(1))
}
