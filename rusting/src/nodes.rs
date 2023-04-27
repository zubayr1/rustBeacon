
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::ReadHalf;
use std::{thread, time};

use std::fs;
use tokio::fs::{OpenOptions};
use futures::executor::block_on;

// use rand::{rngs::OsRng};
// use schnorrkel::{Keypair,Signature, signing_context, PublicKey};
// use schnorrkel::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};

use std::collections::HashSet;

#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

const INITIAL_PORT: u32 = 7081;

pub fn create_keys()
{

    schnorrkel::create_keys_schnorrkel();

}


#[tokio::main]
async fn match_tcp_client(address: String, self_ip: String, types: String, epoch: i32, behavior: String)
{
    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    let mut text = ["epoch".to_string(), epoch.to_string()].join(": ");

    println!("{}", text);

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    println!("client");

    file.write_all("client".as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    
    text = ["server address is".to_string(), address.to_string()].join(": ");

    println!("{}", text);

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    //reading pubkey and sign
    let pubkey = fs::read_to_string("../pubkey.txt").expect("Unable to read file");
    let sign = fs::read_to_string("../sign.txt").expect("Unable to read file");


    let stream = TcpStream::connect(address).await.unwrap();

    let (_, mut write) = tokio::io::split(stream); 

    println!("connection done");

    file.write_all("connection done".as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();
    
    
    if types == "none"
    {   
        if behavior=="1"
        {
            let false_key = schnorrkel::create_adversarial_key();

            write.write_all(false_key.as_bytes()).await.unwrap();
        }
        else
        {
            write.write_all(pubkey.as_bytes()).await.unwrap();
        }
        
        write.write_all(sign.as_bytes()).await.unwrap();
        let id = [self_ip.to_string(), "messageEOF".to_string()].join(" ");
        write.write_all(id.as_bytes()).await.unwrap();
    }
    else 
    {
        write.write_all(types.as_bytes()).await.unwrap();
        write.write_all(types.as_bytes()).await.unwrap();
        write.write_all(b"EOF").await.unwrap();
    }

    
        
    
}



async fn handle_client(ip: String, self_ip: String, types: String, port: u32, epoch: i32, behavior: String) //be leader: 1 instance
{    
    match_tcp_client([ip.to_string(), port.to_string()].join(":"), self_ip, types, epoch, behavior);   
    
}



#[tokio::main] //3 instances
async fn handle_server(server_type: String, ip_address: Vec<String>, args: Vec<String>, self_ip: String, port: u32, epoch: i32, mut blacklisted: HashSet<String>) -> HashSet<String>{
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.unwrap();
    
    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();


    let mut text = ["epoch".to_string(), epoch.to_string()].join(": ");

    println!("{}", text);

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    
    text = ["server at port".to_string(), port.to_string()].join(": ");

    println!("{}", text);

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();
    
    let mut count =0;

    let mut messageperepochcount = 0;
    
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        println!("---continue---");

        
        file.write_all("---continue---".as_bytes()).await.unwrap();
        file.write_all(b"\n").await.unwrap();


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
                    
                    file.write_all("EOF Reached".as_bytes()).await.unwrap();
                    file.write_all(b"\n").await.unwrap();


                    writer.write_all(line.as_bytes()).await.unwrap();
                    println!("{}", line);
                    
                    file.write_all(line.as_bytes()).await.unwrap();
                    file.write_all(b"\n").await.unwrap();
                    
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
         
                   
                if schnorrkel::verify_schnorrkel_sign(pubkeystr, signstr)
                {
                    println!("Identity Verified");
                    
                    file.write_all("Identity Verified".as_bytes()).await.unwrap();
                    file.write_all(b"\n").await.unwrap();

                    let id_info: Vec<&str> = line_collection[2].split(" ").collect();

                    if count<=1
                    {
                        count+=1;
                        for ip in ip_address_clone.clone() // Broadcast to everyone
                        {   
                            if ip!=self_ip.clone() && ip!=id_info[0].to_string().clone()
                            {
                                let address;
                                if args[5]=="dev"
                                {
                                    address = ["127.0.0.1".to_string(), port.to_string()].join(":");
                                }
                                else 
                                {
                                    address = [ip.to_string(), port.to_string()].join(":")
                                }
            
                                let mut stream = TcpStream::connect(address).await.unwrap();
                                
                                let message = ["Re: text EOF".to_string(), self_ip.to_string()].join(" ");
                                
                                stream.write_all(message.as_bytes()).await.unwrap();
            
                                    
                            }                                
                            
                        }
                    }
                }
                else 
                {
                    println!("Identity Verification Failed. Aborting Broadcasting...");

                    
                    file.write_all("Identity Verification Failed. Aborting Broadcasting...".as_bytes()).await.unwrap();
                    file.write_all(b"\n").await.unwrap();

                    let id_info: Vec<&str> = line_collection[2].split(" ").collect();

                    blacklisted.insert(id_info[0].to_string());

                    if count<=1
                    {
                        count+=1;

                        for ip in ip_address_clone.clone() // Broadcast to everyone
                        {   
                            if ip!=self_ip.clone() && ip!=id_info[0].to_string().clone()
                            {
                                let address;
                                if args[5]=="dev"
                                {
                                    address = ["127.0.0.1".to_string(), port.to_string()].join(":");
                                }
                                else 
                                {
                                    address = [ip.to_string(), port.to_string()].join(":")
                                }
            
                                let mut stream = TcpStream::connect(address).await.unwrap();

                                let message = ["Re: Identity Verification Failed".to_string(), id_info[0].to_string().to_string()].join(" ");
                                
                                let broadcast_about_false_leader = [message.to_string(), "EOF".to_string()].join(" ");
                                
                                stream.write_all(broadcast_about_false_leader.as_bytes()).await.unwrap();
                                            
                            }                                
                            
                        }
                    }
                }
            }

            messageperepochcount+=1;
            
            if server_type=="selfserver"
            {
                if messageperepochcount==1 
                {
                   return blacklisted;
                }
            }
            else 
            {
                if messageperepochcount>=args[3].clone().parse::<i32>().unwrap()
                {
                    return blacklisted;
                }
            }
            
            
            

    }
}




pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{  
    let mut blacklisted = HashSet::new(); 

    let mut round_robin_count=0;

    let total = args[3].clone();

    let ip_address_clone = ip_address.clone();


    let args_clone = args.clone();

    let self_ip = args[6].clone();

    let mut count:usize = 0;

    let mut port_count: u32 = 0;


    let behavior = args[8].clone();
  

    for _index in 1..(args[7].parse::<i32>().unwrap()+1)
    {
       
        let mut leader = ip_address_clone[count].clone();

        port_count+=1;

        
        if args[5]=="prod"
        {

            for i in blacklisted.clone()
            {
                println!("{}", i);
            }
            println!("[[[[[");

            while blacklisted.contains(&leader) {
                round_robin_count+=1;   
                round_robin_count%=total.clone().parse::<i32>().unwrap();

                count+=1;
                count%=total.parse::<usize>().unwrap(); 


                leader = ip_address_clone[count].clone();

                println!("{}", leader);
                
            }

            println!("{} {} {}", round_robin_count, count, leader);
            
            if round_robin_count+1==args[2].parse::<i32>().unwrap()
            {
                for ip in ip_address_clone.clone() //LEADER SENDS TO EVERY IP
                {
                    let self_ip_clone = self_ip.clone();
                    let behavior_clone =behavior.clone();

                    
                    if !blacklisted.clone().contains(&ip.clone())
                    {
                        if ip==self_ip.clone()
                        {
                            let ip_address_clone = ip_address.clone();
                            let args_clone1 = args_clone.clone();
                            let self_ip_clone1 = self_ip.clone();  

                           
                            thread::scope(|s| {
                                s.spawn(|| {
                                    let blacklisted_child = handle_server("selfserver".to_string(), ip_address_clone.clone(), args_clone1.clone(), self_ip_clone1.clone(), INITIAL_PORT+port_count, _index, blacklisted.clone());
                                    
                                    blacklisted.extend(blacklisted_child);
                                });
                
                                s.spawn(|| {
                                    let three_millis = time::Duration::from_millis(3);
                                    thread::sleep(three_millis);
            
                                    let future = handle_client(ip.clone(), self_ip_clone.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior_clone.clone());
            
                                    block_on(future);
                                });
                            });


                        }
                        else 
                        {   
                            let three_millis = time::Duration::from_millis(3);
                            thread::sleep(three_millis);
                            handle_client(ip.clone(), self_ip_clone.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior_clone.clone()).await;
                        }

                        

                    }
                    else 
                    {   
                        let three_millis = time::Duration::from_millis(3);
                        thread::sleep(three_millis);
                        handle_client(ip.clone(), self_ip_clone.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior_clone.clone()).await;
                    }
                    
                    
                                    
                }
                
            }
            else
            {
                let blacklisted_child = handle_server("otherserver".to_string(), ip_address.clone(), args_clone.clone(), self_ip.clone(), INITIAL_PORT+port_count, _index, blacklisted.clone());
                
                blacklisted.extend(blacklisted_child.into_iter());
            }

            
        }
        else 
        {                
            handle_client("127.0.0.1".to_string(), self_ip.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior.clone()).await;
        }


        println!("--------------{}------------------", blacklisted.len());

    }
    
    for i in blacklisted.iter()
    {
        println!("{}", i);
    }
    
    

    

}