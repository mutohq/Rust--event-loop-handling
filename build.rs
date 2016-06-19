extern crate gcc;

fn main() {
    gcc::Config::new().file("src/createANDbind.c").compile("libcreateANDbind.a");    
    gcc::Config::new().file("src/makeSOCKETnonblocking.c").compile("libmakeSOCKETnonblocking.a");
    gcc::Config::new().file("src/createSocket.c").compile("libcreateSocket.a");


}
