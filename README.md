<h1>Rust Sendmail</h1>

Rust sending emails via sendmail.


Required:
<ol>
  <li>linux/unix machine</li>
  <li>sendmail compatible software installed (postfix/exim4..)</li>
</ol>


\* Note these instructions assume a Ubuntu machine


<h2>Step One</h2>
<h3>Install sendmail and configure</h3>

Run Commands:
```
apt-get install sendmail
nano /etc/hosts
```

Edit: ```127.0.0.1 localhost localhost.localdomain HOSTNAME_IF_YOU_HAVE ONE```


Run Commands:
```
/etc/init.d/networking stop
/etc/init.d/networking start
```

Run Command:
```
sendmailconfig
```
\* Y to everything


<h2>Step Two</h2>
<h3>Create Main.rs File</h3>

```
extern crate sendmail;
use sendmail::email;

fn main() {

    // Configure email body and header
    email::send_new(
        // From Address
        "from.email@example.com",
        // To Address
        &["to.email@example.com"],
        // Subject
        "Subject - Hello World!",
        // Body
        "<html><body><h1>I am the body. Hello Wolrd!<br/><br/>And I accept html.</h1></body></html>"
    ).unwrap();
}
```
