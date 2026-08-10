use std::process::ExitCode;

use ansi_term::Color::{Cyan, Fixed, Green};
use anyhow::Result;
use clap::Parser;
use styx::{controller::Controller, runner::Runner, terminal::get_default_shell};
use tokio::signal;
use tracing::error;

/// A secure web-based, collaborative terminal.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[cfg(not(feature = "distribution"))]
    /// Address of the remote styx server.
    #[clap(long, default_value = "https://xstyx.io", env = "SSHX_SERVER")]
    server: String,

    /// Local shell command to run in the terminal.
    #[clap(long)]
    shell: Option<String>,

    /// Quiet mode, only prints the URL to stdout.
    #[clap(short, long)]
    quiet: bool,

    /// Session name displayed in the title (defaults to user@hostname).
    #[clap(long)]
    name: Option<String>,

    /// Enable read-only access mode - generates separate URLs for viewers and
    /// editors.
    #[clap(long)]
    enable_readers: bool,
}

impl Args {
    fn server_url(&self) -> &str {
        #[cfg(feature = "distribution")]
        {
            // Compile-time embedded relay — no override possible.
            env!("SSHX_DEFAULT_SERVER")
        }
        #[cfg(not(feature = "distribution"))]
        {
            &self.server
        }
    }
}

fn print_greeting(shell: &str, controller: &Controller) {
    let version_str = match option_env!("CARGO_PKG_VERSION") {
        Some(version) => format!("v{version}"),
        None => String::from("[dev]"),
    };
    if let Some(write_url) = controller.write_url() {
        println!(
            r#"
  {styx} {version}

  {arr}  Read-only link: {link_v}
  {arr}  Writable link:  {link_e}
  {arr}  Shell:          {shell_v}
"#,
            styx = Green.bold().paint("styx"),
            version = Green.paint(&version_str),
            arr = Green.paint("➜"),
            link_v = Cyan.underline().paint(controller.url()),
            link_e = Cyan.underline().paint(write_url),
            shell_v = Fixed(8).paint(shell),
        );
    } else {
        println!(
            r#"
  {styx} {version}

  {arr}  Link:  {link_v}
  {arr}  Shell: {shell_v}
"#,
            styx = Green.bold().paint("styx"),
            version = Green.paint(&version_str),
            arr = Green.paint("➜"),
            link_v = Cyan.underline().paint(controller.url()),
            shell_v = Fixed(8).paint(shell),
        );
    }
}

#[tokio::main]
async fn start(args: Args) -> Result<()> {
    let server = args.server_url().to_owned();
    let quiet = args.quiet;
    let enable_readers = args.enable_readers;

    let shell = match args.shell {
        Some(shell) => shell,
        None => get_default_shell().await,
    };

    let name = args.name.unwrap_or_else(|| {
        let mut name = whoami::username();
        if let Ok(host) = whoami::fallible::hostname() {
            let host = host.split('.').next().unwrap_or(&host);
            name += "@";
            name += host;
        }
        name
    });

    let runner = Runner::Shell(shell.clone());
    let mut controller = Controller::new(&server, &name, runner, enable_readers).await?;
    if quiet {
        if let Some(write_url) = controller.write_url() {
            println!("{}", write_url);
        } else {
            println!("{}", controller.url());
        }
    } else {
        print_greeting(&shell, &controller);
    }

    let exit_signal = signal::ctrl_c();
    tokio::pin!(exit_signal);
    tokio::select! {
        _ = controller.run() => unreachable!(),
        Ok(()) = &mut exit_signal => (),
    };
    controller.close().await?;

    Ok(())
}

fn main() -> ExitCode {
    let args = Args::parse();

    let default_level = if args.quiet { "error" } else { "info" };

    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or(default_level.into()))
        .with_writer(std::io::stderr)
        .init();

    match start(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            error!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(not(feature = "distribution"))]
    fn default_server_is_upstream_in_dev() {
        use clap::Parser;
        let args = super::Args::parse_from(["styx"]);
        assert_eq!(args.server_url(), "https://xstyx.io");
    }

    #[test]
    #[cfg(feature = "distribution")]
    fn distribution_server_is_custom() {
        use clap::Parser;
        let args = super::Args::parse_from(["styx"]);
        let server = args.server_url();
        assert!(server.starts_with("https://"));
        assert!(!server.contains("xstyx.io"));
    }

    #[test]
    #[cfg(feature = "distribution")]
    fn distribution_has_no_server_flag() {
        use clap::Parser;
        let result = super::Args::try_parse_from(["styx", "--server", "https://example.com"]);
        assert!(result.is_err(), "distribution build must not accept --server flag");
    }

    #[test]
    #[cfg(feature = "distribution")]
    fn distribution_ignores_env_var() {
        use clap::Parser;
        std::env::set_var("SSHX_SERVER", "https://evil.com");
        let args = super::Args::parse_from(["styx"]);
        let server = args.server_url();
        assert!(!server.contains("evil.com"));
        std::env::remove_var("SSHX_SERVER");
    }

    #[test]
    #[cfg(feature = "distribution")]
    fn distribution_url_is_https() {
        use clap::Parser;
        let args = super::Args::parse_from(["styx"]);
        assert!(args.server_url().starts_with("https://"));
    }
}
