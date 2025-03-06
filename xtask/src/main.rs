use clap::{Parser, Subcommand};
use snafu::{ResultExt, Snafu};
use std::{fs, path::Path, process::Command};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Failed to execute command: {}", source))]
    CommandExecution { source: std::io::Error },
    #[snafu(display("Failed to write to file: {}", source))]
    FileWrite { source: std::io::Error },
    #[snafu(display("Failed to create directory: {}", source))]
    CreateDir { source: std::io::Error },
    #[snafu(display("Failed to copy file: {}", source))]
    FileCopy { source: std::io::Error },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    Gen,
    Test,
    NTest,
    Example(ExampleArgs),
}

#[derive(clap::Args)]
struct ExampleArgs {
    #[clap(long, value_enum, default_value_t = DestDir::Mo2)]
    dest_mode: DestDir,
    dest: Option<String>,
    #[clap(long, default_value = "module_state")]
    example_name: String,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum DestDir {
    /// root Build directory.
    Build,
    /// D drive Mod Organizer 2 directory.
    Mo2,
}

impl DestDir {
    fn path(&self) -> &Path {
        match self {
            Self::Build => Path::new("./build/mods/module_state_example/SKSE/Plugins/"),
            Self::Mo2 => {
                Path::new("D:\\GAME/ModOrganizer Skyrim SE/mods/module_state_example/SKSE/Plugins/")
            }
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    unsafe {
        std::env::set_var(
            "LIBCLANG_PATH",
            "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\Llvm\\x64\\lib",
        );
    }

    match cli.command {
        Some(Commands::Build) | None => build(),
        Some(Commands::Gen) => {
            unsafe { std::env::set_var("RUST_BACKTRACE", "1") };
            run_command(
                "cargo",
                &[
                    "build",
                    "-p",
                    "commonlibsse_ng_sys",
                    "--features",
                    "generate,vcpkg",
                    "--no-default-features",
                ],
                Some("./target/gen_results.txt"),
            )?;
            run_command(
                "cargo",
                &[
                    "fix",
                    "--edition",
                    "--allow-dirty",
                    "--allow-staged",
                    "-p",
                    "commonlibsse_ng_sys",
                ],
                None,
            )
        }
        Some(Commands::Test) => run_command(
            "cargo",
            &["test", "--workspace", "--features", "debug", "--no-default-features"],
            Some("./test_results.txt"),
        ),
        Some(Commands::NTest) => run_command(
            "cargo",
            &["nextest", "run", "--workspace", "--features", "debug", "--no-default-features"],
            Some("./test_results.txt"),
        ),
        Some(Commands::Example(args)) => run_example(args),
    }
}

fn build() -> Result<()> {
    println!("Building...");
    run_command("cargo", &["build"], Some("./target/build_results.txt"))
}

fn run_command(cmd: &str, args: &[&str], output_file: Option<&str>) -> Result<()> {
    println!("Running: {} {:?}", cmd, args);
    let output = Command::new(cmd).args(args).output().context(CommandExecutionSnafu)?;

    if let Some(output_file) = output_file {
        fs::write(output_file, &output.stdout).context(FileWriteSnafu)?;
        fs::write(output_file, &output.stderr).context(FileWriteSnafu)?;
    } else {
        std::io::Write::write_all(&mut std::io::stdout(), &output.stdout)
            .context(FileWriteSnafu)?;
        std::io::Write::write_all(&mut std::io::stderr(), &output.stderr)
            .context(FileWriteSnafu)?;
    }

    if !output.status.success() {
        eprintln!("Command failed: {} {:?}", cmd, args);
    }

    Ok(())
}

fn run_example(args: ExampleArgs) -> Result<()> {
    println!("Running example...");

    let example_name = args.example_name;
    run_command("cargo", &["build", "-p", "commonlibsse_ng", "--example", &example_name], None)?;

    let dest_dir = args.dest_mode.path();

    fs::create_dir_all(dest_dir).context(CreateDirSnafu)?;

    let dll_filename = format!("{example_name}.dll");
    let pdb_filename = format!("{example_name}.pdb");
    let cargo_build_dir = Path::new("./target/debug/examples");

    let dll_path = cargo_build_dir.join(&dll_filename);
    let pdb_path = cargo_build_dir.join(&pdb_filename);

    fs::copy(dll_path, dest_dir.join(dll_filename)).context(FileCopySnafu)?;
    fs::copy(pdb_path, dest_dir.join(pdb_filename)).context(FileCopySnafu)?;

    Ok(())
}
