//imports
use std::fs::File;
use std::io::{ prelude::*, BufReader};
use futures::executor::block_on;
use std::time::{Duration, Instant};
use std::env;

//import own files
mod nodes;



fn run_nodes()
{
    
    let start = Instant::now();
    let earlystop = Duration::new(1, 0);

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
    

   loop
   {
        let ip_clone = ip_address.clone();
        let future = nodes::initiate(ip_clone);
        block_on(future);

        let duration = start.elapsed();


        if duration>= earlystop
        {
            break;
        }
   }
    


    
    
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
        
    
    if args[1].trim() == keys
    {
        create_keys();
    }
    else 
    {
        run_nodes();
    }



    
    
}
