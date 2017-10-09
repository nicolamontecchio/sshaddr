use std::env;
use std::path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

extern crate regex;

use regex::Regex;


fn get_ssh_config_path() -> Option<path::PathBuf> {
  // get the path to ~/.ssh/config for the current user
  env::home_dir().and_then (
    |h| {
      let mut sshpath = path::PathBuf::new();
      sshpath.push(h);
      sshpath.push(".ssh");
      sshpath.push("config");
      match sshpath.exists() {
        true => Some(sshpath),
        false => None
      }
    }
  )
}

fn get_hostname(fpath : path::PathBuf, host : String) -> Option<String> {
  let host_re = Regex::new(r"^(?i)host\W+(.*)").unwrap();
  let hostname_re = Regex::new(r"^(?i)hostname\W+(.*)").unwrap();
  let p : &path::Path = fpath.as_path();
  File::open(p)
    .ok()
    .and_then(|f|  {
      let mut host_found = false;
      let lines = BufReader::new(&f);
      for line in lines.lines() {
        let l = line.unwrap();
        if host_re.is_match(&l) {
          let line_host = host_re.captures(&l).unwrap().at(1).unwrap();
          if line_host == host.as_str() {
            host_found = true;
          }
        } else if hostname_re.is_match(&l) && host_found {
          let line_hostname = hostname_re.captures(&l).unwrap().at(1).unwrap();
          return Some(line_hostname.to_string())
        }
      }
      return None
    })
}


fn list_hostnames(fpath : path::PathBuf) -> Option<Vec<String>> {
  let host_re = Regex::new(r"^(?i)host\W+(.*)").unwrap();
  let p : &path::Path = fpath.as_path();
  File::open(p)
    .ok()
    .and_then(|f|  {
      let mut hosts : Vec<String> = Vec::new();
      let lines = BufReader::new(&f);
      for line in lines.lines() {
        let l = line.unwrap();
        if host_re.is_match(&l) {
          let line_host = host_re.captures(&l).unwrap().at(1).unwrap().trim();
          if line_host.len() > 0 {
            hosts.push(line_host.to_string());
          }
        }
      }
      Some(hosts)
    })
}


fn main() {
  let args : Vec<String> = env::args().skip(1).collect();
  match args.len() {
    0 => {
      get_ssh_config_path()
        .and_then( |p| { list_hostnames(p)} )
        .map_or( (), |h| {
          for n in h {
            println!("{}", n) }
        })
    },
    _ => {
      for arg in args {
        get_ssh_config_path()
          .and_then( |p| get_hostname(p, arg.to_string()))
          .map_or( (), |h| println!("{}", h) )
      }
    }
  }
}
