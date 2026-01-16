use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const ADR_TEMPLATE: &str = "#### Context
This section describes the forces at play, including technological, political, social, and project local.
These forces are probably in tension, and should be called out as such. The language in this section is value-neutral. It is simply describing facts.
#### Decision
This section describes our response to these forces. It is stated in full sentences, with active voice. 'We will...'
#### Status
i.e. accepted, proposed, etc
#### Consequences
This section describes the resulting context, after applying the decision. All consequences should be listed here,
not just the 'positive' ones. A particular decision may have positive, negative, and neutral consequences,
but all of them affect the team and project in the future.
";
const RFC_TEMPLATE: &str = "#### Summary
#### Motivation
#### Guide-level explanation
#### Reference-level explanation
#### Drawbacks
#### Rationale and alternatives
#### Prior art
#### Unresolved questions
#### Future possibilities
";

#[derive(Parser)]
#[command(
    name = "cargo-fak",
    bin_name = "cargo",
    version,
    author = "louise <email@luisignac.io>",
    about = "Cargo Faaaaaak! is an ADR/RFC tool for your Rust projects. You don't need help, you are human, you are fine. What you need is better tools, Cargo Faaaaaak! aims to help you out.",
    subcommand_required = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "adr", about = "Create a new ADR record.")]
    Adr(RecordArgs),
    #[command(name = "rfc", about = "Create a new RFC record.")]
    Rfc(RecordArgs),
}

#[derive(Args)]
struct RecordArgs {
    #[arg(help = "A title for the record")]
    title: String,
    #[arg(long, value_name = "DIR", help = "Directory to write records")]
    dir: Option<PathBuf>,
    #[arg(
        long,
        value_name = "TEMPLATE",
        help = "Template contents or path to template file"
    )]
    template: Option<String>,
    #[arg(long, help = "Print the created record path")]
    print_path: bool,
    #[arg(long, help = "Resolve output path without writing files")]
    dry_run: bool,
}

#[derive(Clone, Copy)]
enum RecordKind {
    Adr,
    Rfc,
}

fn get_key() -> Result<String> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system time before UNIX EPOCH")?;
    Ok(format!(
        "{}{:09}",
        duration.as_secs(),
        duration.subsec_nanos()
    ))
}

fn sanitize_title(title: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if ch.is_whitespace() || ch == '-' || ch == '_' {
            if !slug.is_empty() && !last_was_dash {
                slug.push('-');
                last_was_dash = true;
            }
        } else if !slug.is_empty() && !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        "untitled".to_string()
    } else {
        slug
    }
}

fn default_dir(kind: RecordKind) -> &'static str {
    match kind {
        RecordKind::Adr => "./docs/adr",
        RecordKind::Rfc => "./docs/rfc",
    }
}

fn default_template(kind: RecordKind) -> &'static str {
    match kind {
        RecordKind::Adr => ADR_TEMPLATE,
        RecordKind::Rfc => RFC_TEMPLATE,
    }
}

fn resolve_dir(kind: RecordKind, dir: Option<PathBuf>) -> PathBuf {
    if let Some(dir) = dir {
        return dir;
    }

    let specific_env = match kind {
        RecordKind::Adr => "CARGO_FAK_ADR_DIR",
        RecordKind::Rfc => "CARGO_FAK_RFC_DIR",
    };

    if let Ok(value) = env::var(specific_env) {
        if !value.is_empty() {
            return PathBuf::from(value);
        }
    }

    if let Ok(value) = env::var("CARGO_FAK_DIR") {
        if !value.is_empty() {
            return PathBuf::from(value);
        }
    }

    PathBuf::from(default_dir(kind))
}

fn build_filename(dir: &Path, key: &str, sanitized_title: &str, attempt: u32) -> PathBuf {
    let base = if attempt == 0 {
        format!("{}_{}", key, sanitized_title)
    } else {
        format!("{}_{}_{}", key, sanitized_title, attempt)
    };
    dir.join(format!("{}.md", base))
}

fn create_record_file(dir: &Path, key: &str, title: &str) -> Result<(PathBuf, fs::File)> {
    let sanitized_title = sanitize_title(title);
    let mut attempt = 0u32;

    loop {
        let path = build_filename(dir, key, &sanitized_title, attempt);
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(why) if why.kind() == io::ErrorKind::AlreadyExists => {
                attempt += 1;
                continue;
            }
            Err(why) => {
                return Err(why).with_context(|| format!("couldn't create {}", path.display()))
            }
        }
    }
}

fn preview_record_path(dir: &Path, key: &str, title: &str) -> PathBuf {
    let sanitized_title = sanitize_title(title);
    let mut attempt = 0u32;

    loop {
        let path = build_filename(dir, key, &sanitized_title, attempt);
        if !path.exists() {
            return path;
        }
        attempt += 1;
    }
}

fn load_template(template: Option<String>, default_template: &str) -> Result<String> {
    match template {
        Some(value) => {
            let path = Path::new(&value);
            if path.is_file() {
                fs::read_to_string(path)
                    .with_context(|| format!("couldn't read template {}", path.display()))
            } else {
                Ok(value)
            }
        }
        None => Ok(default_template.to_string()),
    }
}

fn handle_record(kind: RecordKind, args: RecordArgs) -> Result<()> {
    let dir = resolve_dir(kind, args.dir);
    let key = get_key()?;

    if args.dry_run {
        let path = preview_record_path(&dir, &key, &args.title);
        if args.print_path {
            println!("{}", path.display());
        }
        return Ok(());
    }

    fs::create_dir_all(&dir).with_context(|| format!("couldn't create {}", dir.display()))?;
    let template_contents = load_template(args.template, default_template(kind))?;
    let (path, mut file) = create_record_file(&dir, &key, &args.title)?;
    file.write_all(template_contents.as_bytes())
        .context("failed to write template")?;
    if args.print_path {
        println!("{}", path.display());
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Adr(args) => handle_record(RecordKind::Adr, args),
        Commands::Rfc(args) => handle_record(RecordKind::Rfc, args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(test_name: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        dir.push(format!(
            "cargo-fak-test-{}-{}-{}",
            test_name,
            process::id(),
            nanos
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn sanitize_title_basic() {
        assert_eq!(sanitize_title("My ADR"), "my-adr");
        assert_eq!(sanitize_title("Make/Plan: v2"), "make-plan-v2");
    }

    #[test]
    fn sanitize_title_fallback() {
        assert_eq!(sanitize_title("   "), "untitled");
        assert_eq!(sanitize_title("///"), "untitled");
    }

    #[test]
    fn build_filename_includes_attempt_suffix() {
        let dir = Path::new("docs/adr");
        let base = build_filename(dir, "123", "my-adr", 0);
        assert_eq!(base, PathBuf::from("docs/adr/123_my-adr.md"));

        let attempt = build_filename(dir, "123", "my-adr", 2);
        assert_eq!(attempt, PathBuf::from("docs/adr/123_my-adr_2.md"));
    }

    #[test]
    fn create_record_file_avoids_collision() {
        let dir = temp_dir("collision");
        let key = "123";
        let title = "My ADR";
        let (first_path, first_file) = create_record_file(&dir, key, title).unwrap();
        drop(first_file);
        let (second_path, second_file) = create_record_file(&dir, key, title).unwrap();
        drop(second_file);

        assert_eq!(first_path, build_filename(&dir, key, "my-adr", 0));
        assert_eq!(second_path, build_filename(&dir, key, "my-adr", 1));
        assert!(first_path.exists());
        assert!(second_path.exists());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_dir_argument() {
        let cli = Cli::parse_from(["cargo", "adr", "Title", "--dir", "custom/dir"]);
        match cli.command {
            Commands::Adr(args) => {
                assert_eq!(args.dir, Some(PathBuf::from("custom/dir")));
            }
            Commands::Rfc(_) => {
                panic!("expected adr subcommand");
            }
        }
    }
}
