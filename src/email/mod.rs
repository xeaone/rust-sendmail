use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::io::{Error,ErrorKind};
use std::io::BufReader;
use std::convert::AsRef;

const CONTENT_TYPE: &'static str = "Content-Type: text/html\n";

pub fn send<'a ,T: AsRef<[&'a str]>>(from_address:&str, to_address: T, subject_text: &str, body_text: &str) -> Result<(),Error> {

    let address: &[&str] = to_address.as_ref();

    let mut cmd = Command::new("sendmail");
    cmd.args(address)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    let mut process = cmd.spawn()?;
    
    { // required because of process.wait()
        let mut stdin = process.stdin.as_mut().ok_or(Error::new(ErrorKind::BrokenPipe, "no stdin"))?;
        
        writeln!(&mut stdin,"From: {}",from_address)?;
        writeln!(&mut stdin,"To: {}",address.join(","))?;
        writeln!(&mut stdin,"Subject: {}",subject_text)?;
        stdin.write_all(CONTENT_TYPE.as_bytes())?;
        stdin.write_all(body_text.as_bytes())?;
        stdin.flush()?;
    }
    
    process.wait()?;
    
    //let mut stdout_buffer = BufReader::new(process.stdout.take()?);
    let mut stderr_buffer = BufReader::new(process.stderr.ok_or(Error::new(ErrorKind::BrokenPipe, "no stderr"))?);
    
    let mut stderr: String = String::new();
    stderr_buffer.read_to_string(&mut stderr)?;
    
    if stderr.is_empty() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, format!("sendmail returned: {}",stderr)))
    }
}
