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
  match File::open(p) {
    Ok(f) => {
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
          // println!("{}", line_hostname);
          return Some(line_hostname.to_string())
        }
      }
      None
    },
    Err(_) => (None)
  }

}

fn main() {
  for arg in env::args() {
    get_ssh_config_path()
      .and_then( |p| get_hostname(p, arg.to_string()))
      .map_or( (), |h| println!("{}", h) )
  }
}
