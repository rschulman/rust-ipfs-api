extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

pub struct IPFS {
    host: String,
    port: u16,
    apistring: String
}

impl IPFS {
    pub fn new (host: String, port: u16) -> IPFS {
        IPFS { host: host, port: port, apistring: "/api/v0/".to_string() }
    }

    fn call(&self, cmd: &str, args: Vec<String>) -> String {
        let connection = Client::new();
        //let res = connection.get(&"http://" + self.host + ":" + self.port + "cat?arg=" + path);
        let mut res = connection.get(&format!("http://{}:{}{}{}?arg={}", 
                                         self.host, self.port, self.apistring, cmd, args[0]))
                                         .header(Connection::close())
                                         .send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        return body;
    }
    
    pub fn cat (&self, path: String) -> String {
        return self.call("cat", vec![path]);
    }
}

        
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cat_returns_correct_value() {
        let server = IPFS::new("localhost".to_string(), 5001);
        assert_eq!("This is a test", server.cat("/ipfs/QmR6XorNYAywK4q1dRiRN1gmvfLcx3ccBv68iGtAqon9tt\n".to_string()).trim_right_matches('\n'));
    }
}
