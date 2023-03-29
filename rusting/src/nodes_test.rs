// use std::time::{SystemTime, UNIX_EPOCH};
// use std::thread;
// use std::io::{Read, Write};
// use std::str::from_utf8;
// use futures::executor::block_on;

use tokio::net::TcpListener;
// use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::ReadHalf;




#[tokio::main]
async fn handle_server() {
    let listener = TcpListener::bind("0.0.0.0:22").await.unwrap();

    println!("server");
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        println!("continue");

            let (reader, mut writer) = socket.split();
           
            let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
            let mut line: String  = String :: new();
            // In a loop, read data from the socket and write the data back.
            loop {
                
                let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();

                
                if line.contains("EOF")
                {
                    break;
                }
                
                
            }
            println!("{}", line);

    }
}




pub async fn initiate(arg_id: String, arg_total: String)
{
    
    // let start = SystemTime::now();

    // let since_the_epoch = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");

    // if since_the_epoch.as_millis()%(arg_total.parse::<u128>().unwrap())==arg_id.parse::<u128>().unwrap()+1
    // {
    if arg_id<arg_total
    {
        handle_server();

        
    }
    
    

    

}