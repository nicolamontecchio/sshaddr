use std::env;
use std::path;


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



fn main() {
  get_ssh_config_path()
    .map( |path| println!("path to ssh config is: {}", path.as_path().to_str().unwrap()) );
}
