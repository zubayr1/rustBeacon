use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::fs::File;
use std::str;

use std::sync::atomic::{AtomicU8, Ordering};

use schnorr_fun::{
    fun::{marker::*, Scalar, nonce},
    Schnorr,
    Message
};

use sha2::Sha256;
use rand::rngs::ThreadRng;

static CHECK: AtomicU8 = AtomicU8::new(0);

fn create_sign(sk: String) -> String
{
    let id = "333";

    return id.to_string()+" "+&sk;
}

fn communicate_to_client(mut stream: TcpStream, sk: String) {

    let mut data = [0 as u8; 50]; // using 50 byte buffer
    
    while match stream.read(&mut data) {
        Ok(size) => {
            let imcoing_message = str::from_utf8(&data[0..size]).unwrap();

            println!("{}", imcoing_message);

            let sk1 = sk.clone();
            let plaintext = create_sign(sk1)+ " "+ "Hello from node3";

            let msg : &[u8]= plaintext.as_bytes();
    
            stream.write(msg).unwrap();


            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}

    
}

fn handle_server(node3_port: u32, sk: String) {
    let anycast = String::from("0.0.0.0:");

    let address = [anycast.to_string(), node3_port.to_string()].join("");

    let listener = TcpListener::bind(address).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server node3 listening on port {}", node3_port);
    for stream in listener.incoming() {
        let sk1 = sk.clone();
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    communicate_to_client(stream, sk1)
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
}


fn match_tcp_client(address: String, node_port: u32, pubkeys: Vec<String>)
{
    match TcpStream::connect(address) {
        Ok(mut stream) => {

            let msg = b"Hello from node3!";

            stream.write(msg).unwrap();
          
            let mut data = [0 as u8; 16]; // using 16 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Reply: {} to node3", text);
                    CHECK.store(1, Ordering::Relaxed);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    CHECK.store(0, Ordering::Relaxed);
                    handle_client( node_port, pubkeys);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
            
        }
    }
    return;
}


fn handle_client(node_port: u32, pubkeys: Vec<String>) {
    if CHECK.load(Ordering::Relaxed)==0 
    {

        let localhost = String::from("localhost:");

        match_tcp_client([localhost.to_string(), node_port.to_string()].join(""), node_port, pubkeys);

    }
    


}


// init function
pub fn initiate_node3(random_number: u32, node_port_start: u32, pubkeys:&mut Vec<String>) {

    let mut file = File::open("./node3_sk.txt").expect("cant open the file");

    let mut sk = String::new();

    file.read_to_string(&mut sk).expect("cant read..");

    
    if random_number%4==2
    {       
        let sk1 = sk.clone();
        let handle1 = thread::spawn( move || {

            handle_server(node_port_start+1+3, sk1);          
            
    
        });
        let sk2 = sk.clone();
        let handle2 = thread::spawn(move || {
            
    
            handle_server(node_port_start+2+3, sk2);
            
    
        });
        let sk3 = sk.clone();
        let handle4 = thread::spawn(move || {
            
    
            handle_server(node_port_start+3+4, sk3);
            
    
        });
            
        
        handle1.join().unwrap();
        handle2.join().unwrap();
        handle4.join().unwrap();
        
    }
    else 
    {         
        let pubkeys1 = pubkeys.clone();
        let handle1 = thread::spawn( move || {

            handle_client(node_port_start+1+3, pubkeys1);         
            
    
        });
        let pubkeys2 = pubkeys.clone();
        let handle2 = thread::spawn(move || {
            
    
            handle_client(node_port_start+2+3, pubkeys2);
            
    
        });
        let pubkeys3 = pubkeys.clone();
        let handle4 = thread::spawn(move || {
            
    
            handle_client(node_port_start+3+4, pubkeys3);
            
    
        }); 
        
        handle1.join().unwrap();
        handle2.join().unwrap();  
        handle4.join().unwrap(); 
    }
}

pub fn create_keys()
{
    // Use synthetic nonces
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen.clone());

    // Generate your public/private key-pair
    let keypair = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));
    
    let message = Message::<Public>::plain("111", b"node1");
    // Sign the message with our keypair
    let signature = schnorr.sign(&keypair, message);
    
    println!("node3 {:?}", keypair);
    println!("node3 {:?}", signature);

    
}