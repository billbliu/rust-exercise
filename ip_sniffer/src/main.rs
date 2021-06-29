use std::env;
/*
 * @Author: bill
 * @Date: 2021-06-27 22:38:10
 * @LastEditors: bill
 * @LastEditTime: 2021-06-29 15:27:44
 * @Description: 端口探嗅器
 * @FilePath: /rust-exercise/ip_sniffer/src/main.rs
 */
use std::net::{IpAddr, TcpStream};
use std::io::{self, Write};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

struct Arguments {
    flag: String, // 命令行标签
    ipaddr: IpAddr, // 目标机器ip地址
    threads: u16, // 多线程数
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments{flag: String::from(""), ipaddr, threads: 4});
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") {
                if args.len() == 2 {
                    println!("Usage: -j to select how many threads you want .example: target/debug/ip_sniffer -j 1000 10.3.63.2 \r\n
                        -h or -help to show this help message");
                        return Err("help");
                } else {
                    return Err("too many arguments");
                }
            }

            if flag.contains("-j") {
                f args.len() == 4 { 
                    let ipaddr = match IpAddr::from_str(&args[3]) {
                        Ok(s) => s,
                        Err(_) => return Err("not a valid IPADDR, must be IPv4 or IPv6"),
                    };
                    let threads = match args[2].parse::<u16>() {
                        Ok(s) => s,
                        Err(_) => return Err("failed to parse thread number"), 
                    };
                    return Ok(Arguments{threads, flag, ipaddr});
                } else {
                    return Err("not enough arguments. example: target/debug/ip_sniffer -j 1000 10.3.63.2"); 
                }
            }

            return Err("invalid flag");
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <=  num_threads {
            break;
        }
        port += num_threads;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} parsing arguments fail: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }

}
