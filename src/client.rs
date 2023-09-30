use futures::select;
use futures::FutureExt;

use async_std::{
    io::{stdin, BufReader},
    net::{TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) fn main(ip: impl ToSocketAddrs) -> Result<()> {
    task::block_on(try_main(ip))
}

async fn try_main(addr: impl ToSocketAddrs) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;

    println!("Welcome to a-chat!\n\nThe first message you send will be your username.\n\nTo send a message, type a username or comma-separated list of usernames,\nfollowed by a colon and space, e.g. 'friend: how are you?', or \n'friend,enemy: 'what is up?'");

    let (reader, mut writer) = (&stream, &stream);
    let reader = BufReader::new(reader);
    let mut lines_from_server = futures::StreamExt::fuse(reader.lines());

    let stdin = BufReader::new(stdin());
    let mut lines_from_stdin = futures::StreamExt::fuse(stdin.lines());
    loop {
        select! {
            line = lines_from_server.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    println!("{}", line);
                },
                None => break,
            },
            line = lines_from_stdin.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                },
                None => break,
            }
        }
    }
     
    Ok(())
}