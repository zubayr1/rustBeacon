// use std::time::{SystemTime, UNIX_EPOCH};
// use std::thread;
// use std::io::{Read, Write};
// use std::str::from_utf8;
// use futures::executor::block_on;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::ReadHalf;

use std::fs;

use rand::{rngs::OsRng};
use schnorrkel::{Keypair,Signature, signing_context, PublicKey};
use schnorrkel::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};


pub fn create_keys()
{
    let keypair: Keypair = Keypair::generate_with(OsRng);

    let context = signing_context(b"signature context");
    let message: &[u8] = "zake kal".as_bytes();
    let signature: Signature = keypair.sign(context.bytes(message));

    let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = keypair.public.to_bytes();
    let signature_bytes:  [u8; SIGNATURE_LENGTH]  = signature.to_bytes();

 

    //convert to string for valid utf-8
    let mut pubkeystr="[".to_string();

    let mut flag=0;
    for i in public_key_bytes
    {   if flag==0
        {
            pubkeystr = [pubkeystr.to_string(), i.to_string()].join("");
        }
        else {
            pubkeystr = [pubkeystr.to_string(), i.to_string()].join(", ");
        }
        flag=1;
        
    }
    pubkeystr = [pubkeystr.to_string(), "]".to_string()].join("");
    pubkeystr = [pubkeystr.to_string(), "//".to_string()].join("");


    let mut signstr="[".to_string();
    flag=0;

    for i in signature_bytes
    {
        if flag==0
        {
            signstr = [signstr.to_string(), i.to_string()].join("");
        }
        else {
            signstr = [signstr.to_string(), i.to_string()].join(", ");
        }
        flag=1;
    }
    signstr = [signstr.to_string(), "]".to_string()].join("");
    signstr = [signstr.to_string(), "//".to_string()].join("");

  

    let public_key: PublicKey = PublicKey::from_bytes(&public_key_bytes).unwrap();

    let signature:  Signature = Signature::from_bytes(&signature_bytes).unwrap();

    println!("{:?}", public_key);
    println!("{:?}", signature);

    fs::write("../pubkey.txt", pubkeystr).expect("Unable to write file");
    fs::write("../sign.txt", signstr).expect("Unable to write file");


    

}


#[tokio::main]
async fn match_tcp_client(address: String, types: String)
{
    println!("client");

    //reading pubkey and sign
    let pubkey = fs::read_to_string("../pubkey.txt").expect("Unable to read file");
    let sign = fs::read_to_string("../sign.txt").expect("Unable to read file");


    let mut stream = TcpStream::connect(address).await.unwrap();

    if types == "none"
    {   

        stream.write_all(pubkey.as_bytes()).await.unwrap();
        stream.write_all(sign.as_bytes()).await.unwrap();
        stream.write_all(b"messageEOF").await.unwrap();
    }
    else 
    {
        stream.write_all(types.as_bytes()).await.unwrap();
        stream.write_all(types.as_bytes()).await.unwrap();
        stream.write_all(b"EOF").await.unwrap();
    }
    

    
    
}



async fn handle_client(ip: String, environment: String, types: String) //be leader: 1 instance
{
    if environment=="dev"
    {
        match_tcp_client(["127.0.0.1".to_string(), "8080".to_string()].join(":"), types);

    }
    else 
    {
        match_tcp_client([ip.to_string(), "8080".to_string()].join(":"), types);

    }
       
    
}



#[tokio::main] //3 instances
async fn handle_server(ip_address: Vec<String>, args: Vec<String>) {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    
    println!("server");
    
    let mut count =0;

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        println!("---continue---");

        let arg_ip = args[6].clone();

        let (reader, mut writer) = socket.split();
        
        let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
        let mut line: String  = String :: new();
        

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

            let line_collection: Vec<&str> = line_clone.split("//").collect();


            if line_collection.len()>=3
            {
                let pubkeystr = line_collection[0];
                let signstr = line_collection[1];
    
                // let text = line_collection[2];
    
    
                let pubkeybytes: Vec<u8> = serde_json::from_str(pubkeystr).unwrap();
                let signstrbytes: Vec<u8> = serde_json::from_str(signstr).unwrap();
               
                let public_key: PublicKey = PublicKey::from_bytes(&pubkeybytes).unwrap();
    
                let signature:  Signature = Signature::from_bytes(&signstrbytes).unwrap();
    
                
                let context = signing_context(b"signature context");
                let message: &[u8] = "zake kal".as_bytes();
    
                if public_key.verify(context.bytes(message), &signature).is_ok()
                {
                    println!("Identity Verified");
                    if count<=1
                    {
                        count+=1;

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
                                
                                let message = ["text".to_string(), "EOF".to_string()].join(" ");
                                
                                stream.write_all(message.as_bytes()).await.unwrap();
            
                                    
                            }                                
                            
                        }
                    }
                }
                else 
                {
                    println!("Identity Verification Failed. Aborting Broadcasting...");
                }
            }


            

    }
}




pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{
  //  arg_id: String, arg_total: String, environment: String, leader: String
    // let start = SystemTime::now();

    // let since_the_epoch = start
    //     .duration_since(UNIX_EPOCH)
    //     .expect("Time went backwards");

    // if since_the_epoch.as_millis()%(arg_total.parse::<u128>().unwrap())==arg_id.parse::<u128>().unwrap()+1
    // {
    
    let mut round_robin_count=0;

    let total = args[3].clone();

    let ip_address_clone = ip_address.clone();

    let environment = args[5].clone();

    let args_clone = args.clone();

    let self_ip = args[6].clone();

    for i in 1..(args[7].parse::<i32>().unwrap()+1)
    {
        round_robin_count+=1;
        round_robin_count%=total.parse::<i32>().unwrap();


        if round_robin_count==i
        {
            for ip in ip_address_clone.clone() //LEADER SENDS TO EVERY IP
            {
                if ip!=self_ip
                {
                    handle_client(ip, environment.clone(), "none".to_string()).await;
                }
                                
            }

            
        }
        else
        {
           handle_server(ip_address.clone(), args_clone.clone());

        }

        
    }
    
    

    

}