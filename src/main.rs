use {
    crate::server::LanguageBackend,
    clap::{Parser, Subcommand, ValueEnum},
    std::path::PathBuf,
};

pub mod exporter;
pub mod parser;
pub mod player;
pub mod server;
pub mod types;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Print {
                input,
                target,
                path,
            } => {
                let raw = std::fs::read_to_string(&input).unwrap();
                let (_, file) = crate::parser::file(&raw).unwrap();
                let html = crate::exporter::export_html(&file);

                match target {
                    PrintOutput::Stdout => print!("{}", html),
                    PrintOutput::File => {
                        let path = path.unwrap_or_else(|| input.clone().with_extension("html"));
                        std::fs::write(path.as_path(), html).unwrap();
                    }
                }
            }
            Commands::Play { input } => {
                let raw = std::fs::read_to_string(&input).unwrap();
                let (_, file) = crate::parser::file(&raw).unwrap();
                player::play(file);
            }
            Commands::Serve {} => {
                start_language_server().await;
            }
        }
    } else {
        println!(
            "Use `{} --help` to get more information about how to use this program.",
            std::env::args().next().unwrap_or(String::from("novel"))
        )
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Exports a novel file at <INPUT> to a human-readable HTML file.
    Print {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long, default_value = "file")]
        target: PrintOutput,
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    Serve {},
    Play {
        #[arg(short, long)]
        input: PathBuf,
    },
}

#[derive(ValueEnum, Clone, Default)]
enum PrintOutput {
    #[default]
    File,
    Stdout,
}

// --- //

/// Starts the novel language server.
async fn start_language_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = tower_lsp::LspService::new(|client| LanguageBackend { client });
    tower_lsp::Server::new(stdin, stdout, socket)
        .serve(service)
        .await;
}
