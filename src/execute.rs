use std::process::Stdio;

use tokio::process::Command;

use crate::result::CassetteError;

/// Executes the command and writes `stdout` to vec
pub async fn execute_to_vec(command: Command) -> Result<Vec<u8>, CassetteError> {
    let mut command = command;

    // Specifies the I/O for the command.
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    // Waits for ffprobe to finish and retrieves the output
    let out = command.output().await?;

    // Checks the status with which ffprobe finished
    if !out.status.success() {
        let err = String::from_utf8(out.stderr)?;
        return Err(CassetteError::Io(std::io::Error::other(err)));
    }

    // Returns the ffprobe result as bytes
    let res = out.stdout;
    Ok(res)
}

/// Executes the command, ignoring `stdout`. `stderr` is still taken into account!
pub async fn execute_quiet(command: Command) -> Result<(), CassetteError> {
    let mut command = command;

    // Specifies the I/O for the command.
    command.stdout(Stdio::null());
    command.stderr(Stdio::piped());

    // Waits for ffprobe to finish and retrieves the output
    let out = command.output().await?;

    // Checks the status with which ffprobe finished
    if !out.status.success() {
        let err = String::from_utf8(out.stderr)?;
        return Err(CassetteError::Io(std::io::Error::other(err)));
    }

    Ok(())
}
