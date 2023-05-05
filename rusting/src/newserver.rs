use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::ReadHalf;
use std::{thread, time};

use tokio::fs::{OpenOptions};

use std::collections::HashSet;

#[tokio::main] //3 instances
pub async fn handle_server(server_type: String, ip_address: Vec<String>, args: Vec<String>, self_ip: String, port: u32, epoch: i32, mut blacklisted: HashSet<String>) {
    // while TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.is_err()
    // {
    //     let three_millis = time::Duration::from_millis(3);
    //     thread::sleep(three_millis);
    // }

    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.unwrap();

    let (mut socket, _) = listener.accept().await.unwrap(); // starts listening

    let (reader, mut writer) = socket.split(); // tokio socket split to read and write concurrently
        
        let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
        let mut line: String  = String :: new();

    loop { //loop to get all the data from client until EOF is reached
                
        let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();
        
                       
        if line.contains("EOF") //REACTOR to be used here
        {
            println!("EOF Reached");
            
           

            writer.write_all(line.as_bytes()).await.unwrap();
            println!("{}", line);
          

            line.clear();

            break;
        }
        
        
    }


    return;
}