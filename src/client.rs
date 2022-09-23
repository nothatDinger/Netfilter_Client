use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use crate::connection::Connection;
use crate::log::Log;
use crate::rule::Rule;
const DEV_NAME: &str = "/dev/myfw";
const RULE_DB:  &str = "rule.db";

pub struct Client{
    rule_list       : Vec<Rule>,
    log_list        : Vec<Log>,
    connection_list : Vec<Connection>,
}

enum Operation {
    WriteRule,
    GetConnection,
    GetLog,
    GetNat,
}


impl Client{
    pub fn new_one()-> Client{
        let rule_list = Vec::new();
        let log_list = Vec::new();
        let connection_list = Vec::new();
        Client{rule_list,log_list,connection_list}
    }
    pub fn get_rules(&mut self) {
        self.rule_list.clear();
        let path = Path::new(RULE_DB);
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open file: {}",why),
        };
        for line_result in BufReader::new(file).lines(){
            let line_str =  match line_result {
                Ok(line_str) => line_str,
                Err(e)  => panic!("{}", e)
            };
            let rule = match line_str.parse::<Rule>() {
                Ok(rule) => rule,
                Err(e) => panic!("{}", e)
            };
            self.rule_list.push(rule)
        } 
    }
    
    pub fn commit_rules(&mut self) -> std::io::Result<()> {
        self.get_rules();
        let path = Path::new(DEV_NAME);
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open file: {} in function commit_rules",why),
        };        
        let op = Operation::WriteRule;
        file.write_all(&[(op as u8)])?;
        for rule in &self.rule_list {     
            println!("Commit rules: {}",rule.to_string());
            file.write_fmt(format_args!("{}\n",rule.to_string()))?;
        }
        Ok(())
    }
    pub fn add_rules(&mut self, rule: Rule) -> std::io::Result<()>{
        if let Ok(mut file) = File::create(RULE_DB){
            file.write_fmt(format_args!("{}\n",rule.to_string()))?;
            println!("Add rules : {}",rule.to_string());
        }
        Ok(())
    }
    pub fn delete_rules(&mut self, index: usize) -> std::io::Result<()> {
        self.get_rules();
        self.rule_list.remove(index);
        if let Ok(mut file) = File::create(RULE_DB){
            for rule in &self.rule_list {
                file.write_fmt(format_args!("{}\n",rule.to_string()))?;
                println!("Commit rules: {}",rule.to_string());
            }
        }
        Ok(())
    }
    pub fn print_rules(&mut self) {
        println!("   src_net          dst_net       src_port dst_port protocol action log");
        println!("-----------------------------------------------------------------------");
        self.get_rules();
        for i in &self.rule_list{
            let s: &str = &i.to_string();
            println!("{}",s.trim_start_matches("Rule: "))
        }
    }
    pub fn get_logs(&mut self) -> std::io::Result<()>{
        let mut file = match File::open(DEV_NAME){
            Ok(file) => file,
            Err(why) => panic!("couldn't open file: {} in function: get_logs",why),
        };
        let op = Operation::GetLog;
        file.write_all(&[(op as u8)])?;
        for line_result in BufReader::new(file).lines(){
            let line_str =  match line_result {
                Ok(line_str) => line_str,
                Err(e)  => panic!("{}", e)
            };
            let log = match line_str.parse::<Log>() {
                Ok(log) => log,
                Err(e) => panic!("{}", e)
            };
            self.log_list.push(log)
        }    
        Ok(())    
    }
    pub fn get_connections(&mut self) -> std::io::Result<()> {
        let mut file = match File::open(DEV_NAME){
            Ok(file) => file,
            Err(why) => panic!("couldn't open file: {} in function: get_logs",why),
        };
        let op = Operation::GetConnection;
        file.write_all(&[(op as u8)])?;
        for line_result in BufReader::new(file).lines(){
            // self.log_list.push(line.parse::<Log>()?);
             let line_str =  match line_result {
                 Ok(line_str) => line_str,
                 Err(e)  => panic!("{}", e)
             };
             let conn = match line_str.parse::<Connection>() {
                 Ok(conn) => conn,
                 Err(e) => panic!("{}", e)
             };
             self.connection_list.push(conn)
         }    
         Ok(()) 
    }
}
