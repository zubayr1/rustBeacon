
use std::{thread, time};
use futures::executor::block_on;
use std::collections::HashSet;

#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

#[path = "../probability/create_adv_prob.rs"]
mod create_adv_prob;

#[path = "./client.rs"]
mod client;

#[path = "./server.rs"]
mod server;

#[path = "./newclient.rs"]
mod newclient;

#[path = "./newserver.rs"]
mod newserver;

const INITIAL_PORT: u32 = 7081;

pub fn create_keys() // schnorr key generation
{

    schnorrkel::_create_keys_schnorrkel();

}


async fn handle_client(ip: String, self_ip: String, types: String, port: u32, epoch: i32, behavior: String) // clinet: initiating data sending.
{    
    newclient::match_tcp_client([ip.to_string(), port.to_string()].join(":"), self_ip, types, epoch, behavior);   
    
}




pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{  
    let mut blacklisted = HashSet::new(); // create blacklisted list (should change in recursion)



    let ip_address_clone = ip_address.clone();


    let args_clone = args.clone();

    let self_ip = args[6].clone();


    let  port_count: u32 = 0;


    let  behavior = args[8].clone();
  

    for _index in 1..(args[7].parse::<i32>().unwrap()+1) // iterate for all epoch
    {        
        
        if args[5]=="prod" // in prod mode
        {
            

            for ip in ip_address_clone.clone() //LEADER SENDS TO EVERY IP (should change in recursion because in recursion, its distributed communication) 
            {
                let self_ip_clone = self_ip.clone();
                let behavior_clone =behavior.clone();
                
                
                let ip_address_clone = ip_address.clone();
                        let args_clone1 = args_clone.clone();
                        let self_ip_clone1 = self_ip.clone();  

                        
                        thread::scope(|s| { // tokio thread, since leader is both client and server
                            s.spawn(|| {
                                 newserver::handle_server("otherserver".to_string(), ip_address_clone.clone(), args_clone1.clone(), self_ip_clone1.clone(), INITIAL_PORT+port_count, _index, blacklisted.clone());
                                
                                // blacklisted.extend(blacklisted_child);
                            });
            
                            s.spawn(|| {
                                
        
                                let future = handle_client(ip.clone(), self_ip_clone.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior_clone.clone());
        
                                block_on(future);
                            });
                        });

                    
                                
            }
               
        }
        else 
        {                
            handle_client("127.0.0.1".to_string(), self_ip.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior.clone()).await;
        }


        println!("--------------------------------");

    }
    
    for i in blacklisted.iter()
    {
        println!("{}", i);
    }
    
    

    

}