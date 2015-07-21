use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::process::Command;


pub fn create (from_email_address:&str, to_email_address: &str, subject_text: &str, body_text: &str){

    let from_email_address = from_email_address.to_string();
    let from = "\nFrom:".to_string() + &from_email_address;

    let to_email_address = to_email_address.to_string();
    let to = "\nTo:".to_string() + &to_email_address;

    let subject_text = subject_text.to_string();
    let subject = "\nSubject:".to_string() + &subject_text;

    let header = from + &to + &subject;
    let body_text = body_text.to_string();
    let body = "\n".to_string() + &body_text;
    let header_plus_body = header + &body;

    let path = Path::new("email.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file
    };

    // Write the 'text' string to 'file', returns 'io::Result<()>''
    match file.write_all(header_plus_body.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, Error::description(&why)),
        Ok(_) => println!("Successfully wrote to {}", display)
    }
}


pub fn send (to_email_address: &str) {

    // Send Email using sendmail -t
    let to_email_address = to_email_address.to_string();
    let sendmail_sh_frist_half = "sendmail ".to_string() + &to_email_address;
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
