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
    stream.write_all(b"client hello!").await.unwrap();
    stream.write_all(b"EOF").await.unwrap();
    stream.shutdown().await.unwrap();
   
}



async fn handle_client(ip: String, environment: String) //be leader: 1 instance
{
    if environment=="dev"
    {
        match_tcp_client(["127.0.0.1".to_string(), "8080".to_string()].join(":"));

    }
    else 
    {
        match_tcp_client([ip.to_string(), "8080".to_string()].join(":"));

    }
       
    
}



#[tokio::main] //3 instances
async fn handle_server() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
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




pub async fn initiate(ip_address: Vec<String>, arg_id: String, arg_total: String, environment: String)
{
    
    // let start = SystemTime::now();

    // let since_the_epoch = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");

    // if since_the_epoch.as_millis()%(arg_total.parse::<u128>().unwrap())==arg_id.parse::<u128>().unwrap()+1
    // {
    if arg_id=="1" && (arg_id<arg_total)
    {
        for ip in ip_address //LEADER SENDS TO EVERY IP
        {
            println!("{}", ip);
            if ip!="18.117.92.19"
            {
                handle_client(ip, environment.clone()).await;
            }
            
        }

        
    }
    else
    {
       handle_server();

    }
    

    

}