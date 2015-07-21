extern crate sendmail;

fn main() {

    // Configure email body and header
    sendmail::email::create(
        "noreply@example.com",
        "aemail@gmail.com",
        "This Is A Subject",
        "I am the body. Hello Wolrd!"
    );

    // Define the actual email address to recieve the email
    sendmail::email::send("aemail@gmail.com");

}
