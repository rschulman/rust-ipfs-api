extern crate hyper;
extern crate protobuf;
extern crate rust_base58;

mod merkledag;
mod unixfs;

use hyper::Client;
use hyper::header::Connection;
use protobuf::core::Message;
use rust_base58::ToBase58;
use std::str::from_utf8;

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

    fn call(&self, cmd: &str, args: Vec<String>) -> Result<merkledag::PBNode, IPFSError> {
        let connection = Client::new();
        //let res = connection.get(&"http://" + self.host + ":" + self.port + "cat?arg=" + path);
        let mut res = connection.get(&format!("http://{}:{}{}{}?arg={}&encoding=protobuf", 
                                         self.host, self.port, self.apistring, cmd, args[0]))
                                         .header(Connection::close())
                                         .send().unwrap();
        let mut object = merkledag::PBNode::new();
        object.merge_from(&mut protobuf::CodedInputStream::new(&mut res));

        return Ok(object); // This is a hack, we should not be unwrapping above and instead doing
                         // something with errors.
    }
    
    pub fn cat (&self, path: String) -> Result<String, IPFSError> {
        let result = self.call("/object/get", vec![path]);
        println!("{:?}", result);
        match result {
            Ok(node) => {
                let mut content = unixfs::Data::new();
                content.merge_from_bytes(&mut node.get_Data());
                return Ok(from_utf8(content.get_Data()).unwrap().to_string());
            }
            Err(error) => Err(error)
        }
    }

    pub fn ls (&self, path: String) -> Result<Vec<(String, u64, String)>, IPFSError> {
        let result = self.call("/object/get", vec![path]);
        match result {
            Ok(node) => {
                let links = node.get_Links();
                let mut link_vec = Vec::new();
                for link in links {
                    link_vec.push((link.get_Hash().to_base58(), link.get_Tsize(), link.get_Name().to_string()));
                }
                return Ok(link_vec);
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
        
        if checked.len() == 0 {
            panic!("No results returned.");
        }

        for (asked, gotten) in checked.iter().zip(&expect) {
            assert_eq!(asked, gotten);
        }
    }
}
