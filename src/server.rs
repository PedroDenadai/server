use crate::http::Request;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;


pub struct Server {
    addr: String,

}

impl Server {
    pub fn new(addr: String ) -> Self{
        Self {
            addr
        }
    }

    pub fn run(self){
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop{
            match listener.accept(){
                Ok((mut stream, _)) =>{
                    
                    let mut buffer = [0; 1024]; // tamanho da requisicao 1Kbytes
                    
                    match stream.read(&mut buffer){

                        Ok(_)=>{
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer)); // pega a request e transforma de binary para texto

                            //let res: &Result<Request,  _> = &buffer[..].try_into(); outro jeito de implimentar o try_from

                            match Request::try_from(&buffer[..]) {// slice que tem todos os elementos do array (buffer)
                                Ok(request) => {

                                },
                                Err(e) => println!("Failed to parse: {}", e),

                            }
                            
                        }

                        Err(e) => println!("Failed to connect: {}", e), 
                    }
                }

                Err(e) => println!("Failed to connect: {}", e),
            }
            
            
            
            
            
            //let res = listener.accept();
            //if res.is_err(){continue;}
            //let (stream, addr) = res.unwrap();
        }
    }
}