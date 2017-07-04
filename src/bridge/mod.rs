use tokio_core::reactor::{Core, Handle};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_io::{AsyncRead,AsyncWrite};
use tokio_io::io;
use futures::{Stream, Sink};
use futures::Future;
use result;

pub fn run() -> result::Result<()> {

    let mut core = Core::new()?;
    let handle = core.handle();

    let socket_addr = "127.0.0.1:1024".parse()?;
    
    let listener = TcpListener::bind(&socket_addr, &handle)?;

    let bridge = listener.incoming().for_each(|(stream, addr)|{
        let (reader, writer) = stream.split(); 
        
        let bytes_copied = io::copy(reader, writer);

        let handle_conn = bytes_copied.map(|(n, _, _)| {
            println!("wrote {} bytes", n)
        }).map_err(|err| {
            println!("IO error {:?}", err)
        });

        // Spawn the future as a concurrent task
        handle.spawn(handle_conn);

        Ok(())
    });

    core.run(bridge)?;

    Ok(())

}
