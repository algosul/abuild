use abuild::{command, command::Commands};
fn main() -> command::Result<()> {
    let matches = Commands::builder()
        .command("new", command::commands::New {})?
        .command("init", command::commands::Init {})?
        .build()?
        .to_clap_command()
        .get_matches();
    println!("{matches:#?}");
    Ok(())
}
