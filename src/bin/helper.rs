use std::process::ExitCode;

fn main() -> ExitCode {
    run()
}

fn run() -> ExitCode {
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_exits_successfully() {
        assert_eq!(run(), ExitCode::SUCCESS);
    }
}
