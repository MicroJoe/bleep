mod audio;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Agathe Porte <microjoe@microjoe.org>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// List all builtin sounds
    List,

    /// Run a command and bleep when finished
    Run,

    /// Play a builtin sound
    Play(Play),
}

#[derive(Clap)]
struct Play {
    /// Name of the builtin sound to play
    name: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::List => {
            for e in audio::BUILTINS.keys() {
                println!("{}", e);
            }
        }
        SubCommand::Play(play) => {
            audio::play_builtin(&play.name).unwrap();
        }
        SubCommand::Run => {}
    }
}
