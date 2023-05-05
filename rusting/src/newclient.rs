use std::fs;
use tokio::fs::{OpenOptions};

use tokio::net::TcpStream;
use tokio::io::{ AsyncWriteExt};

use std::{thread, time};


#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String, types: String, epoch: i32, behavior: String)
{ 
    let addressclone = address.clone();

    
    while TcpStream::connect(addressclone.clone()).await.is_err() {
        let three_millis = time::Duration::from_millis(3);
        thread::sleep(three_millis);
        println!("s");
    }

    let mut stream = TcpStream::connect(address).await.unwrap();

    println!("aaa{}", addressclone);


    loop
    {
    let id = [self_ip.to_string(), "messageEOF".to_string()].join(" ");
    stream.write(id.as_bytes()).await.unwrap();
    }
}