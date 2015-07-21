<h1>Rust Sendmail</h1>

Rust sending emails via sendmail.

This is the begning stages of this repo and I am a Rust newbie. Plese provide suggestions or corrections. Currently there is no working mehtod (that I am aware of) to send emails with rust. 


The requirments for Rust Sendmail:
<ol>
  <li>linux/unix machine</li>
  <li>sendmail installed</li>
</ol>


\* Not these instructions assume a Ubuntu machine


<h2>Step One</h2>
<h3>Install sendmail and configure<h3>
  
Run Commands:

    apt-get install sendmail
    nano /etc/hosts


  Edit: <b>127.0.0.1 localhost localhost.localdomain HOSTNAME_IF_YOU_HAVE ONE</b>
  

Run Commands:

    /etc/init.d/networking stop
    /etc/init.d/networking start
    
  Y to everything:
  
    sendmailconfig

  
  
  
