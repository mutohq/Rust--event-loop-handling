extern crate gcc;

fn main() {
    gcc::Config::new().file("src/createANDbind.c").compile("libcreateANDbind..a");    
}
