use std::{
    io::Write,
    process::{Command, Stdio},
};

use ini::Ini;

pub fn generate_client_config(
    client_private_key: String,
    server_public_key: &str,
    ip_addr: &str,
) -> Ini {
    let mut conf = Ini::new();
    conf.with_section(Some("Interface"))
        .set("PrivateKey", client_private_key)
        .set("Address", ip_addr)
        .set("DNS", "1.1.1");
    conf.with_section(Some("Peer"))
        .set("PublicKey", server_public_key)
        .set("AllowedIPs", "0.0.0.0")
        .set("Endpoint", "Placeholder");
    conf
}
