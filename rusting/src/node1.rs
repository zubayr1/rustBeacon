use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::sync::atomic::{AtomicU8, Ordering};
use std::str;

use std::fs::File;

use schnorr_fun::{
    fun::{marker::*, Scalar, nonce, Point},
    Schnorr,
    Message, Signature
};
use core::str::FromStr;

use sha2::Sha256;
use rand::rngs::ThreadRng;

static CHECK: AtomicU8 = AtomicU8::new(0); 

fn create_sign(sk: String) -> String
{
    let id = "111";

    
    return id.to_string() + " " + &sk;
}


fn communicate_to_client(mut stream: TcpStream, sk: String) {

    let mut data = [0 as u8; 50]; // using 50 byte buffer
    
    while match stream.read(&mut data) {
        
        Ok(size) => {
            let imcoing_message = str::from_utf8(&data[0..size]).unwrap();

            println!("{}", imcoing_message);
            let sk1 = sk.clone();
            let plaintext = create_sign(sk1)+ " "+ "Hello from node1";

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

fn handle_server(node1_port: u32, sk: String) 
{
    let anycast = String::from("0.0.0.0:");

    let address = [anycast.to_string(), node1_port.to_string()].join("");

    let listener = TcpListener::bind(address).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server node1 listening on port {}", node1_port);
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
    return;
}


fn match_tcp_client(address: String, node_port: u32, pubkeys: Vec<String>)
{
    match TcpStream::connect(address) {
        Ok(mut stream) => {

            let msg = b"Hello from node1!";

            stream.write(msg).unwrap();
           
            let mut data = [0 as u8; 16]; // using 16 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Reply: {} to node1", text);

                    CHECK.store(1, Ordering::Relaxed);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    
                    handle_client( node_port, pubkeys);
                }
            }
        },
        Err(e) => {
             println!("Failed to connect: {}", e);
            
        }
    }
   
}

fn handle_client(node_port: u32, pubkeys: Vec<String>) {

    if CHECK.load(Ordering::Relaxed)==0 
    {
        
        let localhost = String::from("localhost:");
    
        match_tcp_client([localhost.to_string(), node_port.to_string()].join(""), node_port, pubkeys);

        
    }   

    
}



// init function
pub fn initiate_node1(random_number: u32, node_port_start: u32, pubkeys:&mut Vec<String>) 
{
    
    let mut skfile = File::open("./node1_sk.txt").expect("cant open the file");

    let mut sk = String::new();

    skfile.read_to_string(&mut sk).expect("cant read..");


    if random_number%4==0
    {      
        let sk1 = sk.clone();

        let handle2 = thread::spawn( move || {

           handle_server(node_port_start+1+2, sk1); 

    
        });
        let sk2 = sk.clone();

        let handle3 = thread::spawn(move || {
            
    
            handle_server(node_port_start+1+3, sk2);
            
    
        });
        let sk3 = sk.clone();
        let handle4 = thread::spawn(move || {
            
    
            handle_server(node_port_start+1+4, sk3);
            
    
        });
            
        
        handle2.join().unwrap();
        handle3.join().unwrap();
        handle4.join().unwrap();
        
        
    }
    else
    {
        let pubkeys1 = pubkeys.clone();
        let handle2 = thread::spawn( move || {

            handle_client(node_port_start+1+2, pubkeys1);          
            
    
        });
        let pubkeys2 = pubkeys.clone();
        let handle3 = thread::spawn(move || {
            
    
            handle_client(node_port_start+1+3, pubkeys2);
            
    
        });
        let pubkeys3 = pubkeys.clone();
        let handle4 = thread::spawn(move || {
            
    
            handle_client(node_port_start+1+4, pubkeys3);
            
    
        });
            
        
        handle2.join().unwrap();
        handle3.join().unwrap();  
        handle4.join().unwrap();  
    }
}


pub fn create_keys()
{
    // Use synthetic nonces
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen.clone());

    let val = Scalar::random(&mut rand::thread_rng());

    // Generate your public/private key-pair
    let keypair = schnorr.new_keypair(val.clone());

    let message = Message::<Public>::plain("111", b"node1");
    // Sign the message with our keypair
    let signature = schnorr.sign(&keypair, message);
    
    println!("node1 {:?}", keypair);
    println!("node1 {:?}", signature);



    // let schnorr = Schnorr::<Sha256>::verify_only();
    // let public_key = Point::<EvenY, Public>::from_str("78a3da536f44b6f5af4d0decbeb2cd2c792d48ba83ed4d0d6f8c0a02b5757758").unwrap();
    // let signature = Signature::<Public>::from_str("0d293959af1074c1332bc328c37553024dbb39799bd4be36781608fc65bee65951dd0c75eb322e95a3963afbcb863214f74ec205e5f3f01b258bb6e183fa9cef").unwrap();
    // println!("{}",schnorr.verify(&public_key, message, &signature));

}

