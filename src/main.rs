use skill_md_clarifier_validator::{OperationalError, audit_target};
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        println!(
            "skill-md-clarifier-validator {}\n\nUSAGE:\n    skill-md-clarifier-validator <skill-root-or-SKILL.md>\n\nEXIT CODES:\n    0  deterministic checks passed\n    1  deterministic findings were reported\n    2  usage, target resolution, read, or parse failure",
            env!("CARGO_PKG_VERSION")
        );
        return ExitCode::SUCCESS;
    }
    if args.len() == 1 && args[0] == "--version" {
        println!("skill-md-clarifier-validator {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }
    if args.len() != 1 {
        return write_error(OperationalError {
            code: "INVALID_ARGUMENTS",
            message:
                "Expected exactly one <skill-root-or-SKILL.md> argument. Use --help for usage."
                    .to_string(),
            path: None,
        });
    }

    match audit_target(Path::new(&args[0])) {
        Ok(report) => {
            let exit_code = report.exit_code();
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            if serde_json::to_writer_pretty(&mut handle, &report).is_err()
                || writeln!(handle).is_err()
            {
                return ExitCode::from(2);
            }
            ExitCode::from(exit_code as u8)
        }
        Err(error) => write_error(error),
    }
}

fn write_error(error: OperationalError) -> ExitCode {
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let envelope = error.envelope();
    let _ = serde_json::to_writer_pretty(&mut handle, &envelope);
    let _ = writeln!(handle);
    ExitCode::from(2)
}
