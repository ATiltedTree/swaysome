use clap::Clap;
use swayipc::Connection;

/// Per-screen workspaces for sway
#[derive(Debug, Clap)]
#[clap(version, author, about)]
enum Args {
    /// Initialize a workspace on all outputs
    Init { name: String },

    /// Move the current window to a workspace
    Move { name: String },

    /// Focus a workspace
    Focus { name: String },
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut connection = Connection::new()?;

    let get_current_output_index = |connection: &mut Connection| {
        connection.get_outputs().map(|outputs| {
            outputs
                .iter()
                .position(|x| x.focused)
                .expect("No focused output")
        })
    };

    match args {
        Args::Init { name } => {
            for (i, output) in connection.get_outputs()?.iter().enumerate() {
                let cmd = format!("focus output {}", output.name);
                connection.run_command(cmd)?;

                let cmd = format!("workspace {}{}", i, name);
                connection.run_command(cmd)?;
            }
        }
        Args::Move { name } => {
            let output = get_current_output_index(&mut connection)?;
            let cmd = format!("move container to workspace {}{}", output, name);
            connection.run_command(cmd)?;
        }
        Args::Focus { name } => {
            let output = get_current_output_index(&mut connection)?;
            let cmd = format!("workspace {}{}", output, name);
            connection.run_command(cmd)?;
        }
    }

    Ok(())
}
