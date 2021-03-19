use std;
use std::io;
use std::io::Write;
use crate::vm::VM;

// core structure for the REPL for the Assembler
pub struct REPL {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM
}

impl REPL{
    // Creates and returns new assembly REPL
    pub fn new() -> REPL{
        REPL{
            vm: VM::new(),
            command_buffer: vec![]
        }
    }

    pub fn run(&mut self){
        println!("Welcome to HLVM! Let's be productive!");
        loop {
            // allocates a new string to store whatever the user types each iteration
            let mut buffer = String::new();

            // blocking call until the user types in a command
            let stdin = io::stdin();
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Farewell! Have a good day!");
                    std::process::exit(0);
                },
                ".history" =>{
                    for command in &self.command_buffer{
                        println!("{}",command);
                    }
                },
                ".program" =>{
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program{
                        println!("{}",instruction);
                    }
                    println!("End of Program Listing");
                },
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}",self.vm.registers);
                    println!("End of Register Listing");
                },
                _ => {
                    // let results = self.parse_hex(buffer);
                    // match results {
                    //     Ok(bytes)=>{
                    //         for byte in bytes {
                    //             self.vm.add_byte(byte)
                    //         }
                    //     },
                    //     Err(_e)=>{
                    //         println!("Unable to decode hex string.Please enter 4 group of 2 hex characters.")
                    //     }
                    // };
                    // self.vm.run_once();
                    println!("Input Invalied!");
                }
            }
        }
    }

    // fn parse_hex(&mut self,i: &str) {
    //     let split = i.split(" ").collect::<Vec<&str>>();
    //     let mut results: Vec<u8> = vec![];
    //     for hex_string in split {
    //         let byte = u8::from_str_radix(&hex_string, 16);
    //         match byte {
    //             OK(result) => {
    //                 results.push(result);
    //             },
    //             Err(e) => {
    //                 return Err(e);
    //             }
    //         }
    //     }
    //     Ok(results);
    // }
}