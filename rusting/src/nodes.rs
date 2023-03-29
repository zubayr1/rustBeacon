// use std::time::{SystemTime, UNIX_EPOCH};
// use std::thread;
// use std::io::{Read, Write};
// use std::str::from_utf8;
// use futures::executor::block_on;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::ReadHalf;




pub fn create_keys()
{

}


#[tokio::main]
async fn match_tcp_client(address: String)
{
    println!("client");
    let mut stream = TcpStream::connect(address).await.unwrap();


    stream.write_all(b"client hello!").await.unwrap();
   
}



async fn handle_client(ip: String) //be leader
{
    
    match_tcp_client([ip.to_string(), "8080".to_string()].join(":"));
       
    
}






#[tokio::main]
async fn handle_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("server");
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        println!("continue");

            let (reader, mut writer) = socket.split();
           
            let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
            let mut line: String  = String :: new();
            // In a loop, read data from the socket and write the data back.
            loop {
                
                let bytes_read: usize = reader.read_line(&mut line).await.unwrap();

                println!("{}", bytes_read);
                if bytes_read == 0
                {
                    break;
                }

                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
                
            }
    }
}




pub async fn initiate(ip_address: Vec<String>, arg_id: String, arg_total: String)
{
    
    // let start = SystemTime::now();

    // let since_the_epoch = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");

    // if since_the_epoch.as_millis()%(arg_total.parse::<u128>().unwrap())==arg_id.parse::<u128>().unwrap()+1
    // {
    if arg_id=="1" && (arg_id<arg_total)
    {
        let ip = ip_address[0].clone(); //TAKE FIRST IP AS LEADER
        handle_client(ip).await;

        
    }
    else
    {
       handle_server();

    }
    

    

}