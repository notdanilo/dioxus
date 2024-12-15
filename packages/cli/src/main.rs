#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/79236386")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/79236386")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use dioxus_cli::*;

#[tokio::main]
async fn main() {
    // If we're being ran as a linker (likely from ourselves), we want to act as a linker instead.
    if let Some(link_action) = link::LinkAction::from_env() {
        return link_action.run();
    }

    let args = TraceController::initialize();

    #[cfg(debug_assertions)]
    tracing::warn!("CLI was built with debug profile. Commands will run slower.");

    let result = match args.action {
        Commands::Translate(opts) => opts.translate(),
        Commands::New(opts) => opts.create(),
        Commands::Init(opts) => opts.init(),
        Commands::Config(opts) => opts.config(),
        Commands::Autoformat(opts) => opts.autoformat(),
        Commands::Check(opts) => opts.check().await,
        Commands::Clean(opts) => opts.clean().await,
        Commands::Build(opts) => opts.run_cmd().await,
        Commands::Serve(opts) => opts.serve().await,
        Commands::Bundle(opts) => opts.bundle().await,
        Commands::Run(opts) => opts.run().await,
    };

    // Provide a structured output for third party tools that can consume the output of the CLI
    match result {
        Ok(output) => {
            tracing::debug!(json = ?output);
        }
        Err(err) => {
            tracing::error!(
                ?err,
                json = ?StructuredOutput::Error {
                    message: format!("{err:?}"),
                },
            );

            std::process::exit(1);
        }
    };
}
