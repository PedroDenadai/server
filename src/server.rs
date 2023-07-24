use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::Read;


pub struct Server {
    addr: String,

}

fn arr(a: &[u8]){
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