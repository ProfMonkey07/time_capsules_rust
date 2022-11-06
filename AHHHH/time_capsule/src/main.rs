use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;
use std::io::Write;
use std::io::prelude::*;
use std::io;
use std::fs;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};
fn encrypt(key: &str){
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("you are making a new time capsule, please specify the directory in which to place the capsule");
    let mut directory:String = String::new();
    io::stdin().read_line(&mut directory).expect("failed to read user input");
    println!("what do you want to call your time capsule?");
    let mut name:String = String::new();
    io::stdin().read_line(&mut name).expect("failed to read user input");
    println!("now please type the message you want your time capsule to contain");
    let mut message:String = String::new();
    io::stdin().read_line(&mut message).expect("failed to read user input");
    message = format!("{} is time {} is message", time.to_string(), message.trim());
    message = new_magic_crypt!(key, 256).encrypt_str_to_base64(message.trim());
    println!("{}", message);
    let mut capsule = std::fs::File::create(format!("{}{}.capsule", directory.trim(), name.trim())).expect("ok that was the error");
    capsule.write_all(message.as_bytes()).expect("tough luck");
}
fn decrypt(key: &str){
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("please specify the location of the capsule to open, note: capsules may only be opened after 5 years");
    let mut path:String = String::new();
    io::stdin().read_line(&mut path).expect("failed to read user input");
    let file = fs::read_to_string(path.trim()).expect("Should have been able to read the file");
    let decrypter = new_magic_crypt!(key, 256);
    let decrypted = decrypter.decrypt_base64_to_string(&file).unwrap();
    let decvec: Vec<&str> = decrypted.split(' ').collect();
    let actualTime = decvec[0].parse::<u128>().unwrap();
    if(time - actualTime >= 31557600000) {
        println!("time created: {}", decrypted);
    } else{
        println!("sorry, you still have {} milliseconds before you can open this time capsule", 31557600000 - (time - actualTime))
    }

}
fn main() {
    println!("type create to create a new capsule, type open to open a capsule");
    let mut response:String = String::new();
    io::stdin().read_line(&mut response).expect("failed to read user input");
    match response.trim() {
        "create" => encrypt("PdSgVkYp3s6v8y/B?E(H+MbQeThWmZq4"),
        "open" => decrypt("PdSgVkYp3s6v8y/B?E(H+MbQeThWmZq4"),
        _ => println!("dude you didnt type the right thing did you read what it said?"),
    }
}
