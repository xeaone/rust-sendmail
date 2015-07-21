extern crate sendmail;

fn main() {

    // Configure email body and header
    sendmail::email::create(
        "noreply@verge.website",
        "alex.steven.elias@gmail.com",
        "This Is Subject",
        "I am the body. hello wolrd!"
    );

    // Define the actual email address to recieve the email
    sendmail::email::send("alex.steven.elias@gmail.com");

}
