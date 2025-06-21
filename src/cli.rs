use std::{collections::HashMap, io::{self, Write}, net::Ipv4Addr, str::FromStr, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, time::sleep};
use url::Url;

use crate::{req::{reqres::DIDRequest, verbs::ReqVerb}, tcp::stream::read_expected_size};

/// This CLI is a `did` small client mainly used for testing. To use the CLI,
/// call the `start_cli` function in a tokio environment.
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     start_cli().await;
/// }
/// ```
pub async fn start_cli() {
    let mut run = true;
    let mut ctx: HashMap<String, String> = HashMap::new();

    ctx.insert("did".into(), "imapotato2".into());
    ctx.insert("ip".into(), "0.0.0.0".into());
    while run {
        let input = get_input();
       
        if input.is_empty() {
            continue;
        }

        let args = input.split(" ").collect::<Vec<&str>>();
        let command = *args.get(0).unwrap();

        run = command != "exit";
        match command {
            "help" => {
                println!("set \t<key> <value>\t\t\tWill set a runtime value");
                println!("    \t<did>");
                println!("    \t<ip>");
                println!("get \t<key>        \t\tReads settable properties");
                println!("send\t<to(ip)> <verb> <path> <body>\t\tSend a DID req");
                println!("exit\t");
            },
            "set" if args.len() == 3 => {
                ctx.insert(
                    args.get(1).unwrap().to_string(),
                    args.get(2).unwrap().to_string()
                );
            },
            "get" if args.len() == 2 => {
                let key = args.get(1).unwrap().to_string();

                if ctx.contains_key(&key) {
                    println!("\t= {}", ctx.get(&key).unwrap());
                } else {
                    println!("Unknown property: {key}");
                }
            },
            "send" if args.len() == 5 => {
                let ip = args.get(1).unwrap();
                let verb = args.get(2).unwrap();
                let path = args.get(3).unwrap();
                let body = args.get(4).unwrap();

                let local_ip = ctx.get("ip").unwrap();
                let local_did = ctx.get("did").unwrap();

                let mut sock = TcpStream::connect(format!("{ip}:5000")).await
                    .unwrap();


                sock.writable().await.unwrap();
                sock.write_all(DIDRequest {
                    verb: ReqVerb::from_str(verb).unwrap(),
                    url: Some(Url::from_str(
                            &format!("did://{ip}{path}")
                        ).unwrap()),
                    ip: Ipv4Addr::from_str(local_ip).unwrap(),
                    did: local_did.clone(),
                    body: body.to_string(),
                    req_size: 0
                }.to_string().as_bytes()).await.unwrap();
            
                println!("waiting for a response");

                sock.readable().await.unwrap();
                let response = read_expected_size(&mut sock).await;
                println!("-> {response}");
            },
            _ => println!("Unknown command: {command}")
        };
    }
}

fn get_input() -> String {
    let mut buf = String::new();

    print!("did://> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
