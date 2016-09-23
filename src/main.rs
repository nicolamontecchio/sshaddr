use std::env;
use std::path;


fn get_ssh_config_path() -> Option<path::PathBuf> {
  // get the path to ~/.ssh/config for the current user
  env::home_dir().map(
    |h| {
      let mut sshpath = path::PathBuf::new();
      sshpath.push(h);
      sshpath.push(".ssh");
      sshpath.push("config");
      sshpath
    }
  )
}



fn main() {

  match get_ssh_config_path() {
    Some(path) => println!("path to ssh config is: {}", path.as_path().to_str().unwrap()),
    None => println!("found no path :(")
  }



}
