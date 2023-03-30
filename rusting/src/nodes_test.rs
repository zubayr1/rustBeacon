// use std::time::{SystemTime, UNIX_EPOCH};
// use std::thread;
// use std::io::{Read, Write};
// use std::str::from_utf8;
// use futures::executor::block_on;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::ReadHalf;



#[tokio::main]
async fn handle_server(ip_address: Vec<String>, args: Vec<String>) {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    
    println!("server");
    
    let mut count =0;

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        println!("continue");

        let arg_ip = args[6].clone();

        let (reader, mut writer) = socket.split();
        
        let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
        let mut line: String  = String :: new();
        // In a loop, read data from the socket and write the data back.
        let ip_address_clone;
        let line_clone;
            loop {
                
                let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();

                

                if line.contains("EOF")
                {
                    println!("EOF Reached");
                    writer.write_all(line.as_bytes()).await.unwrap();
                    println!("{}", line);
                    
                    ip_address_clone = ip_address.clone();
                    
                    line_clone = line.clone();


                    line.clear();

                    break;
                }
                
                
            }

            if count<=1
            {
                for ip in ip_address_clone.clone() // Broadcast to everyone
                {   
                    if ip!=arg_ip.clone()
                    {
                        let address;
                        if args[5]=="dev"
                        {
                            address = ["127.0.0.1".to_string(), "8080".to_string()].join(":");
                        }
                        else 
                        {
                            address = [ip.to_string(), "8080".to_string()].join(":")
                        }
    
                        let mut stream = TcpStream::connect(address).await.unwrap();
                        
                        let message = [line_clone.to_string(), "EOF".to_string()].join(" ");
                        
                        stream.write_all(message.as_bytes()).await.unwrap();
    
                            
                    }                                
                    
                }
            }
            
            count+=1;                    
            
            

    }
}




pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{
    
    // let start = SystemTime::now();

    // let since_the_epoch = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");

    // if since_the_epoch.as_millis()%(arg_total.parse::<u128>().unwrap())==arg_id.parse::<u128>().unwrap()+1
    // {
    if args[2]<args[3]
    {
        handle_server(ip_address, args.clone());

        
    }
    
    

    

}