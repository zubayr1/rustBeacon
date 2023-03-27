use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

pub fn create_keys()
{

}



fn match_tcp_client(address: String)
{
    let address_clone = address.clone();
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            
            let msg = b"Hello from node!";

            stream.write(msg).unwrap();
           
            let mut data = [0 as u8; 16]; // using 16 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Reply: {} to node1", text);

                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    
                    handle_client( address_clone);
                }
            }
        },
        Err(e) => {
             println!("Failed to connect: {}", e);
            
        }
    }
   
}



fn handle_client(ip: String) //be leader
{
    match_tcp_client([ip, "4422".to_string()].join(":"));
}


fn communicate_to_client(mut stream: TcpStream) {

    let mut data = [0 as u8; 50]; // using 50 byte buffer
    
    while match stream.read(&mut data) {
        
        Ok(size) => {
            let incoing_message = std::str::from_utf8(&data[0..size]).unwrap();

            println!("{}", incoing_message);
            

            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}

    
}



fn handle_server(ip: String)
{
    let address = ["0.0.0.0".to_string(), "4422".to_string()].join(":");
    println!("{}", address);
    let listener = TcpListener::bind(address).unwrap();
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    communicate_to_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
    return;
}




pub async fn initiate(ip_address: Vec<String>, arg_id: String, arg_total: String)
{

    let start = SystemTime::now();

    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    if since_the_epoch.as_millis()%(arg_total.parse::<u128>().unwrap())==arg_id.parse::<u128>().unwrap()+1
    {
        for ip in ip_address
        {
            
            let handler = thread::spawn( move || {

                handle_client(ip); 
        
         
             });

             
        
             handler.join().unwrap();
        }
    }
    else
    {
        for ip in ip_address
        {
            let handler = thread::spawn( move || {

                handle_server(ip); 
        
         
             });

            
        
             handler.join().unwrap();
        }
    }
    

    

}