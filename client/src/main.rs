use anyhow::{Context, Result};
use clap::Parser;
use client::{guard, Cli, Config};
use duang::Duang;
use protocol::short::{request, Request};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Read, Write},
};

fn main() -> Result<()> {
    let config = Config::try_read()?;
    let address = config.address.as_str();
    let token = get_token(address)?;
    let mut duang = Duang::builder(address)
        .method(request::Method::Persistence)
        .token(token.as_str())
        .try_build()?;

    duang.send()?;
    let socket = duang.into_socket();

    Ok(())
}

fn get_token(address: &str) -> Result<String> {
    let cli = Cli::parse();

    let mut token_path = dirs::data_dir().context("$XDG_DATA_HOME not set")?;
    token_path.push("tcp-chatroom-client/token");
    let mut token_fd = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(token_path)?;

    if !cli.manually_login && !cli.register_login {
        let mut token = String::new();
        token_fd.read_to_string(&mut token)?;
        return Ok(token);
    }

    let mut req = Request::from(request::Method::Login);

    if cli.manually_login {
        let user = guard::manually_login()?;
        req.set_data(user);
    } else {
        let user = guard::register_login()?;
        req.set_data(user);
    }

    let token: String = Duang::try_from((address, req))?.send()?.data().unwrap();

    BufWriter::new(&mut token_fd).write_all(token.as_bytes())?;

    Ok(token)
}
