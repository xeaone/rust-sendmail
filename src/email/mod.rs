use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::process::Command;


pub fn create (from_address:&str, to_address: &str, subject_text: &str, body_text: &str){

    let from_address = from_address.to_string();
    let from = "From:".to_string() + &from_address + "\n";

    let to_address = to_address.to_string();
    let to = "To:".to_string() + &to_address + "\n";

    let subject_text = subject_text.to_string();
    let subject = "Subject:".to_string() + &subject_text + "\n";

    let content_type = "Content-Type: text/html".to_string() + "\n";
    let header = from + &to + &subject + &content_type;

    let body = body_text.to_string();

    let header_body = header + &body;

    let path = Path::new("email.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file
    };

    // Write the 'text' string to 'file', returns 'io::Result<()>''
    match file.write_all(header_body.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, Error::description(&why)),
        Ok(_) => println!("Successfully wrote to {}", display)
    }
}


pub fn send (to_address: &str) {

    // Send Email using sendmail -t
    let to_address = to_address.to_string();
    let sendmail_sh_frist_half = "sendmail ".to_string() + &to_address;
    let sendmail_sh_second_half = " < email.txt".to_string();
    let sendmail_sh = sendmail_sh_frist_half + & sendmail_sh_second_half;

    let output = Command::new("sh")
                    .arg("-c")
                    .arg(sendmail_sh)
                    .output()
                    .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
