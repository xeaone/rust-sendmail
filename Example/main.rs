extern crate sendmail;
use sendmail::email;

fn main() {

    // Configure email body and header
    email::create(
        // From Address
        "from.email@example.com",
        // To Address
        "to.email@example.com",
        // Subject
        "Subject - Hello World!",
        // Body
        "<html><body><h1>I am the body. Hello Wolrd!<br/><br/>And I accept html.</h1></body></html>"
    );

    // Define the actual email address to recieve the email
    email::send("your.email@gmail.com");
}
