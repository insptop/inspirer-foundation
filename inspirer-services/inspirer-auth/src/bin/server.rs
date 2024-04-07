use inspirer_auth::app::App;
use inspirer_framework::cli;

fn main() -> eyre::Result<()> {
    cli::run_with_name::<App>("inspirer-auth")
}