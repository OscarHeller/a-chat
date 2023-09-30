mod client;
mod server;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send +Sync>>;

fn main() -> Result<()> {
    let mut args = std::env::args();

    match (args.nth(1).as_deref(), args.next().as_deref(), args.next().as_deref()) {
        (Some("server"), None, None) => {
            println!("Starting server.");
            let result = server::main();

            match result {
                Err(error) => {
                    return Err(format!("Server error: {}", error).into());
                },
                _ => {}
            }
        },
        (Some("client"), Some(ip), None) => {
            println!("Starting client to {}.", ip);
            let result = client::main(ip);

            match result {
                Err(error) => {
                    return Err(format!("Client error: {}", error).into());
                },
                _ => {}
            }
        },
        _ => {
            return Err("Usage: a-chat [client <ip:port>|server]".into());
        },
    }

    Ok(())
}