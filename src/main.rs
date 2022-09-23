pub mod rule;
pub mod connection;
pub mod log;
pub mod client;
pub mod common;
use client::Client;
use ipnetwork::Ipv4Network;
use clap::{App, load_yaml};
use rule::Rule;
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut client = Client::new_one();
 
    let test = "192.168.0.1/24".parse::<Ipv4Network>();
    match test{
        Ok(res) => println!("{}", res),
        Err(why) => println!("{}", why)
    }    

    if let Ok(adds) = matches.values_of_t::<Rule>("add"){
        for i in adds{
            println!("Value for -a: {}", i);
            client.add_rules(i).unwrap();
        }
    }
    if let Some(delete) = matches.value_of("delete"){
        
        let d: usize =  delete.parse::<usize>().unwrap();
        println!("Value for -d: {}", d);
        client.delete_rules(d).unwrap();
    }
    if matches.contains_id("commit"){
        println!("Value for -c ");
        client.commit_rules().unwrap();
    }
    if matches.contains_id("print"){
        println!("Value for -p ");
        client.print_rules();
    }
}
