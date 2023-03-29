//imports
use std::thread;

use std::fs::File;
use std::io::{ prelude::*, BufReader};
use futures::executor::block_on;
// use std::time::{Duration, Instant};
use std::env::{self};
use chrono::prelude::*;

//import own files
mod nodes;
mod nodes_test;


fn run_nodes(arg_id: String, arg_total: String, environment: String, leader: String)
{
    // let start = Instant::now();
    // let earlystop = Duration::new(1, 0);

    let mut ids: Vec<String> = Vec::new();
    let mut ip_address: Vec<String> = Vec::new();
    let mut pubkeys: Vec<String> = Vec::new();

    let nodesfile = File::open("./nodes_information.txt").expect("cant open the file");
    let reader = BufReader::new(nodesfile);
    
    for line in reader.lines() 
    {
        let line_uw = line.unwrap();
        
        let textsplit = line_uw.split("-");

        let mut count=0;
        for db in textsplit {
            count+=1;

            if count==1
            {   
                ids.push(db.to_string());
            }
            if count==2
            {
                ip_address.push(db.to_string());
            }

            if count==3
            {
                pubkeys.push(db.to_string());
                count=0;
            }
            
      }
    }
    

//    loop
//    {
        let ip_clone = ip_address.clone();
        let arg_id_clone = arg_id.clone();
        let arg_total_clone = arg_total.clone();
        

        if environment=="dev"
        {
            let handle1 = thread::spawn(move || {
            
    
                let future = nodes::initiate(ip_clone, arg_id_clone, arg_total_clone, environment, leader);
    
            
                block_on(future);
                
        
            });
            let handle2 = thread::spawn(move || {
                
        
                let future1 = nodes_test::initiate(arg_id, arg_total);
    
            
                block_on(future1);
                
        
            });
                
            
            handle1.join().unwrap();
                
            
            handle2.join().unwrap();
        } 
        else 
        {
            let future = nodes::initiate(ip_clone, arg_id_clone, arg_total_clone, environment, leader);
    
            
            block_on(future);
        }
         

        // let duration = start.elapsed();


        // if duration>= earlystop
        // {
        //     break;
        // }
   //}
    


    
    
}


fn create_keys()
{
    nodes::create_keys();
    
}

fn main() 
{
    println!("Starting");    
    
    let args: Vec<String> = env::args().collect();

    let keys: &str = "keys";

    println!("execution type");

    println!("{}", args[1]);
        

    loop 
    {
        let utc: DateTime<Utc>  = Utc::now();
        // make arg time
        let month = &args[4][0..2].to_string();
        let date = &args[4][2..4].to_string();
        let hour = &args[4][4..6].to_string();
        let min = &args[4][6..8].to_string();
        
        if utc >= Utc.with_ymd_and_hms(2023, month.parse::<u32>().unwrap(), 
        date.parse::<u32>().unwrap(), hour.parse::<u32>().unwrap(), min.parse::<u32>().unwrap(), 00).unwrap()
        {
            break;
        }
    }

    println!("launched");
    
    if args[1].trim() == keys
    {
        create_keys();
    }
    else 
    {
        run_nodes(args[2].clone(), args[3].clone(), args[5].clone(), args[6].clone());
    }



    
    
}
