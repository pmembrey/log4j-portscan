//! Simple little inverse port scanner to determine if you're blocking the default
//! outbound ports that the log4j exploit (commonly known as log4shell:
//! [CVE-2021-44228](https://nvd.nist.gov/vuln/detail/CVE-2021-44228))
//!
//! This tool tries to connect on the default ports this exploit uses, namely:
//! * 389
//! * 636
//! * 1099
//! * 1389
//! * 3268
//! * 3269
//!
//! Note: If an attacker uses a different port, blocking the default ports
//!       (and hence the results of this test) are worthless.
//!
//! The host the app tries to connect to these ports on an AWS instance I
//! created at http://log4j.the.engineer . nginx is binding to all of the
//! ports - there's nothing dangerous or malicious hosted.
//!
//! Regardless, please make sure you check and update your software! There
//! are vulnerabilities everywhere, and blocking these outbound ports can only
//! do so much...
//!

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

use paris::Logger;
use std::io;
use std::time::Duration;

fn main() {
    let mut logger = Logger::new();

    // The default ports used in the exploit
    let ports = vec![389, 636, 1099, 1389, 3268, 3269];

    logger.info("Starting scan...");

    // Counter to see if there were any failures
    let mut fail_counter = 0;

    for port in ports {
        // Kick off our loading animation
        logger.loading(format!("Trying port {}", port));

        // We need a SocketAddr in order to use connect_timeout
        // Using an IP to avoid DNS lookups - won't ever change anyway

        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(54, 218, 168, 32)), port);

        match TcpStream::connect_timeout(&socket, Duration::from_secs(3)) {
            Ok(_) => {
                // Normally this would be a good thing because we got connected
                // but in this case, we're hoping the ports are blocked and hence
                // this triggers a failure
                logger.warn(format!("Port {}\t is not <YELLOW>blocked</>", port));
                fail_counter += 1;
            }

            // We care about two kinds of error. It's not safe to consider other errors
            // as a sign that the port is reliably unreachable, so that will throw an
            // invalid test warning
            Err(error) => match error.kind() {
                // This one is a bit iffy - if you get connection refused against my server
                // though, then something is blocking your connection out, so that's good enough.
                io::ErrorKind::ConnectionRefused => {
                    logger.success(format!("Port {}\t is <GREEN>Blocked!</>", port));
                }

                // This is the one we're generally looking for as it's the effect most firewalls
                // will have on blocked traffic
                io::ErrorKind::TimedOut => {
                    logger.success(format!("Port {}\t is <GREEN>Blocked!</>", port));
                }

                // The catch all - no idea what these could be, so I'm just going to treat them
                // as transiently bad but they'll still cause the test to fail.
                _ => {
                    logger.warn("Unexpected error: test invalid!");
                    fail_counter += 1;
                }
            },
        }
    }
    if fail_counter > 0 {
        logger.warn("<RED>Failed:</> One or more ports were not blocked!");
    } else {
        logger.success("<GREEN>Success:</> Default ports are all blocked!");
    }
}
