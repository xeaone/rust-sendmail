extern crate sendmail;
use sendmail::email;

fn main() {

    // Configure email body and header
    email::send(
        // From Address
        "test@localhost",
        // To Address
        &["root@localhost"],
        // Subject
        "Subject - Hello World!",
        // Body
        "<html><body><h1>I am the body. Hello Wolrd!<br/><br/>And I accept html.</h1></body></html>"
    ).unwrap();
}
