use clap::{Command, FromArgMatches, Subcommand};

#[derive(Subcommand, Debug, Clone)]
enum Subcommands {
    /// Update FRS
    Update,
    /// Uninstall FRS 
    Uninstall
}

fn main() {
    let cli = Subcommands::augment_subcommands(Command::new("frs"));

    let matches = cli.get_matches();
    let derived_subcommands = Subcommands::from_arg_matches(&matches)
        .map_err(|err| err.exit())
        .unwrap();
    println!("Derived subcommands: {derived_subcommands:#?}");

    match derived_subcommands {
        Subcommands::Update => {
            println!("we should run update now")
        }

        Subcommands::Uninstall => {
            println!("we should run uninstall now")
        }
    }
}
