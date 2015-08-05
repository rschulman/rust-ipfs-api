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

    pub fn ls (&self, path: String) -> Vec<(String, u32, String)> {
        return vec![("".to_string(), 0, "".to_string())];
    }
}

        
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cat_returns_correct_value() {
        let server = IPFS::new("localhost".to_string(), 5001);
        assert_eq!("This is a test", server.cat("/ipfs/QmR6XorNYAywK4q1dRiRN1gmvfLcx3ccBv68iGtAqon9tt".to_string()).trim_right_matches('\n'));
    }

    #[test]
    fn ls_returns_vec_of_correct_values() {
        let server = IPFS::new("localhost".to_string(), 5001);
        // Use the ipfs.io website within IPFS as test
        let response = server.ls("/ipfs/QmeYYwD4y4DgVVdAzhT7wW5vrvmbKPQj8wcV2pAzjbj886".to_string());
        let expect = vec![("QmTkQCrspeQDEiFBvphH3ULHYNWr3aXpymGiyjMrrUimtZ".to_string(), 1717422, "/blog".to_string()),
                          ("Qma4JRMJgwjBhsaBkEGXh682zphSoNi67k7pgGxkqTouiK".to_string(), 131161, "/docs".to_string()),
                          ("QmSXujSW6xykhU5wECQRrSW83YRjz8M8t93mfKddKm9ncL".to_string(), 11894, "index.html".to_string()),
                          ("QmNXFHU3KzhzXN9pZ5Bdghk3LYGj58Xzi7WvMVVoykdLxA".to_string(), 136279, "/media".to_string()),
                          ("QmSEBfiu7BmQkoHuBPAjs9tHqvM61NiKfFfRJR4UnTrorx".to_string(), 6223432, "/styles".to_string())];
        for (asked, gotten) in response.iter().zip(&expect) {
            assert_eq!(asked, gotten);
        }
    }
}
