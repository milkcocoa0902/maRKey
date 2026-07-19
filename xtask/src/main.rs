use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

const LARGE_FIXTURE_LINES: usize = 100_000;
const FNV1A_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV1A_PRIME: u64 = 0x100000001b3;

fn main() {
    if let Err(error) = run() {
        eprintln!("xtask failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let Some(command) = arguments.next() else {
        return Err("usage: cargo run -p xtask -- generate-large-fixture [output]".into());
    };

    if command != "generate-large-fixture" {
        return Err(format!(
            "unknown command {:?}; expected generate-large-fixture",
            command
        )
        .into());
    }

    let output = arguments
        .next()
        .map_or_else(default_output_path, PathBuf::from);
    if arguments.next().is_some() {
        return Err("generate-large-fixture accepts at most one output path".into());
    }

    let stats = generate_large_fixture(&output, LARGE_FIXTURE_LINES)?;
    println!("generated: {}", output.display());
    println!("lines: {}", stats.lines);
    println!("bytes: {}", stats.bytes);
    println!("fnv1a64: {:016x}", stats.fnv1a64);

    Ok(())
}

fn default_output_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask must be inside the workspace root")
        .join("fixtures/generated/large.md")
}

fn generate_large_fixture(path: &Path, total_lines: usize) -> io::Result<FixtureStats> {
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "large fixture output must have a parent directory",
        )
    })?;
    fs::create_dir_all(parent)?;

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let stats = write_large_fixture(&mut writer, total_lines)?;
    writer.flush()?;

    Ok(stats)
}

fn write_large_fixture(writer: &mut impl Write, total_lines: usize) -> io::Result<FixtureStats> {
    if total_lines < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "large fixture must contain at least two lines",
        ));
    }

    let mut stats = FixtureStats::new();
    stats.write(writer, b"# Large Markdown Fixture\n")?;
    stats.write(writer, b"\n")?;

    for line_number in 3..=total_lines {
        let line = format!(
            "{line_number:06}: Markdown editor performance fixture with Japanese text \u{65e5}\u{672c}\u{8a9e}, emoji \u{1f4dd}, and repeated words alpha beta gamma delta epsilon.\n"
        );
        stats.write(writer, line.as_bytes())?;
    }

    Ok(stats)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FixtureStats {
    lines: usize,
    bytes: usize,
    fnv1a64: u64,
}

impl FixtureStats {
    const fn new() -> Self {
        Self {
            lines: 0,
            bytes: 0,
            fnv1a64: FNV1A_OFFSET_BASIS,
        }
    }

    fn write(&mut self, writer: &mut impl Write, bytes: &[u8]) -> io::Result<()> {
        writer.write_all(bytes)?;
        self.lines += bytes.iter().filter(|&&byte| byte == b'\n').count();
        self.bytes += bytes.len();
        for &byte in bytes {
            self.fnv1a64 ^= u64::from(byte);
            self.fnv1a64 = self.fnv1a64.wrapping_mul(FNV1A_PRIME);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_the_requested_number_of_lines_deterministically() {
        let mut first = Vec::new();
        let first_stats = write_large_fixture(&mut first, 10).unwrap();
        let mut second = Vec::new();
        let second_stats = write_large_fixture(&mut second, 10).unwrap();

        assert_eq!(first, second);
        assert_eq!(first_stats, second_stats);
        assert_eq!(first_stats.lines, 10);
        assert_eq!(first_stats.bytes, first.len());

        let source = String::from_utf8(first).unwrap();
        assert!(source.starts_with("# Large Markdown Fixture\n\n"));
        assert!(source.ends_with("alpha beta gamma delta epsilon.\n"));
    }

    #[test]
    fn rejects_too_few_lines() {
        let error = write_large_fixture(&mut Vec::new(), 1).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    }
}
