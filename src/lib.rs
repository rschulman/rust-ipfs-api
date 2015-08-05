extern crate hyper;
extern crate serde;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use serde::json::{self, Value};

#[derive(Debug)]
pub enum IPFSError {
    ConnectionError,
    NoSuchHash
}

pub struct IPFS {
    host: String,
    port: u16,
    apistring: String
}

impl IPFS {
    pub fn new (host: String, port: u16) -> IPFS {
        IPFS { host: host, port: port, apistring: "/api/v0/".to_string() }
    }

    fn call(&self, cmd: &str, args: Vec<String>) -> Result<String, IPFSError> {
        let connection = Client::new();
        //let res = connection.get(&"http://" + self.host + ":" + self.port + "cat?arg=" + path);
        let mut res = connection.get(&format!("http://{}:{}{}{}?arg={}", 
                                         self.host, self.port, self.apistring, cmd, args[0]))
                                         .header(Connection::close())
                                         .send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        return Ok(body); // This is a hack, we should not be unwrapping above and instead doing
                         // something with errors.
    }
    
    pub fn cat (&self, path: String) -> Result<String, IPFSError> {
        return self.call("cat", vec![path]);
    }

    pub fn ls (&self, path: String) -> Result<Vec<(String, u64, String)>, IPFSError> {
        let result = self.call("ls", vec![path]);
        match result {
            Ok(raw_data) => {
                let data:Value = json::from_str(&raw_data).unwrap();
                let obj = data.as_object().unwrap();
                let obj_list = obj.get("Objects").unwrap().as_array().unwrap()[0].as_object().unwrap();
                 
                let ls_list = obj_list.get("Links").unwrap().as_array().unwrap();
                let mut links = Vec::new();
                for link in ls_list {
                    let link_obj = link.as_object().unwrap();
                    links.push((link_obj.get("Hash").unwrap().as_string().unwrap().to_string(), link_obj.get("Size").unwrap().as_u64().unwrap(), link_obj.get("Name").unwrap().as_string().unwrap().to_string()));
                }
                return Ok(links);
            }
            Err(error) => Err(error)
        }
    }
}

        
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cat_returns_correct_value() {
        let server = IPFS::new("localhost".to_string(), 5001);
        assert_eq!("This is a test", server.cat("/ipfs/QmR6XorNYAywK4q1dRiRN1gmvfLcx3ccBv68iGtAqon9tt".to_string()).unwrap().trim_right_matches('\n'));
    }

    #[test]
    fn ls_returns_vec_of_correct_values() {
        let server = IPFS::new("localhost".to_string(), 5001);
        // Use the ipfs.io website within IPFS as test
        let response = server.ls("/ipfs/QmeYYwD4y4DgVVdAzhT7wW5vrvmbKPQj8wcV2pAzjbj886".to_string());
        let expect = vec![("QmTkQCrspeQDEiFBvphH3ULHYNWr3aXpymGiyjMrrUimtZ".to_string(), 1717422, "blog".to_string()),
                          ("Qma4JRMJgwjBhsaBkEGXh682zphSoNi67k7pgGxkqTouiK".to_string(), 131161, "docs".to_string()),
                          ("QmSXujSW6xykhU5wECQRrSW83YRjz8M8t93mfKddKm9ncL".to_string(), 11894, "index.html".to_string()),
                          ("QmNXFHU3KzhzXN9pZ5Bdghk3LYGj58Xzi7WvMVVoykdLxA".to_string(), 136279, "media".to_string()),
                          ("QmSEBfiu7BmQkoHuBPAjs9tHqvM61NiKfFfRJR4UnTrorx".to_string(), 6223432, "styles".to_string())];
        let checked = match response {
            Ok(data) => data,
            Err(error) => panic!(error)
        };

        for (asked, gotten) in checked.iter().zip(&expect) {
            assert_eq!(asked, gotten);
        }
    }
}
