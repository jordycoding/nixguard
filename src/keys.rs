use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn generate_keypair() -> (String, String) {
    let priv_key = Command::new("wg")
        .arg("genkey")
        .output()
        .expect("Error generating private key")
        .stdout;

    let priv_key_string = String::from_utf8(priv_key).unwrap().replace('\n', "");

    let mut command = Command::new("wg")
        .arg("pubkey")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");
    let command_stdin = command.stdin.as_mut().unwrap();
    command_stdin.write_all(priv_key_string.as_bytes());
    let output = String::from_utf8(
        command
            .wait_with_output()
            .expect("Error generating public key")
            .stdout,
    )
    .unwrap()
    .replace('\n', "");
    (priv_key_string, output)
}
