use std::env::{self, Args};
use torchc_lits::lits;

/// _**Command-line Interface**_
#[derive(Debug)]
pub struct Cli {
    pub cmd: Option<Box<[u8]>>,
    pub subcmd: Option<Subcmd>,
}
impl Cli {
    pub fn parse() -> Self {
        let mut args: Args = env::args();
        Self {
            // `torch ...`
            //  ^^^^^
            cmd: match args.next() {
                Some(cmd) if !cmd.is_empty() => Some(cmd.into_bytes().into_boxed_slice()),
                _ => None,
            },
            subcmd: {
                Some(match args.next() {
                    Some(subcmd) => match subcmd.to_ascii_lowercase().as_str() {
                        // `torch build`
                        //        ^^^^^
                        lits::cli::BUILD => Subcmd::Build(match args.next() {
                            Some(nested_subcmd) => {
                                match nested_subcmd.to_ascii_lowercase().as_str() {
                                    // `torch build run`
                                    //              ^^^
                                    lits::cli::RUN => Some(NestedSubcmdForBuild::Run),

                                    _ => panic!("illegal subcommand for build"),
                                }
                            }
                            None => None, // `torch build`
                        }),

                        // `torch run`
                        //        ^^^
                        lits::cli::RUN => Subcmd::Run,

                        // `torch manual`
                        //        ^^^^^^
                        lits::cli::MANUAL => Subcmd::Manual,

                        _ => panic!("illegal subcommand"),
                    },
                    None => Subcmd::Version, // `torch`
                })
            },
        }
    }
}

/// CLI subcommands.
#[derive(Debug)]
#[repr(u8)]
pub enum Subcmd {
    /// Build the release executable in the `target/` folder.
    Build(Option<NestedSubcmdForBuild>),
    /// Build and run the hidden development executable.
    Run,
    /// Advise on the use of the command-line interface.
    Manual,
    /// Displays language version information.
    Version,
}
/// Subcommands of the `Build` subcommand.
#[derive(Debug)]
#[repr(u8)]
pub enum NestedSubcmdForBuild {
    /// Run the release executable from the `target/` folder.
    Run,
}
