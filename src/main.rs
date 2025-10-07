use {
    crate::server::LanguageBackend,
    clap::{Parser, Subcommand},
    std::path::PathBuf,
};

pub mod exporter;
pub mod parser;
pub mod server;
pub mod types;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Print { input, output } => {
                let _ = print_file(input, output);
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
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Serve {},
}

// --- //

fn print_file(input: PathBuf, output: Option<PathBuf>) -> std::io::Result<()> {
    let output = output.unwrap_or_else(|| input.clone().with_extension("html"));

    let raw = std::fs::read_to_string(input)?;
    let f = crate::parser::file(&raw);

    if let Ok((_, file)) = f {
        std::fs::write(output.as_path(), crate::exporter::export_html(&file)).unwrap();
    } else {
        eprintln!("{:#?}", f);
    }

    Ok(())
}

/// Starts the novel language server.
async fn start_language_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = tower_lsp::LspService::new(|client| LanguageBackend { client });
    tower_lsp::Server::new(stdin, stdout, socket)
        .serve(service)
        .await;
}
