use std::env;

fn get_ssh_config_path() -> Option<String> {
  // get the path to ~/.ssh/config for the current user
  match env::home_dir() {
    Some(p) => {
      p.as_path().to_str().map(|s| {
        let mut q = s.to_string();
        q.push_str("/.ssh/config");
        q
      } )
    },
    None => None
  }
}


fn main() {
  match get_ssh_config_path() {
    Some(path) => println!("path to ssh config is: {}", path),
    None => println!("found no path :(")
  }



}
