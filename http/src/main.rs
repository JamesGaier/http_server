use nix::sys::epoll::*;
use nix::sys::eventfd::{EventFd};
use std::time::Instant;
use nix::unistd::{read, write, pipe};
use std::net::{TcpListener};
use std::os::unix::io::AsRawFd;
use std::io;

fn main() -> nix::Result<()> {
    const DATA: u64 = 17;
    const MILLIS: u16 = 100;
    
    // Create epoll
    let epoll = Epoll::new(EpollCreateFlags::empty())?;

    let listener = TcpListener::bind("127.0.0.1:3000")
                        .unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");
    
    // Create eventfd & Add event
    let eventfd = EventFd::new()?;
    epoll.add(&eventfd, EpollEvent::new(EpollFlags::EPOLLIN | EpollFlags::EPOLLOUT | EpollFlags::EPOLLET,listener.as_raw_fd().try_into().unwrap()))?;
    
    //// Arm eventfd & Time wait
    //eventfd.write(1)?;
    //let now = Instant::now();
    
    loop {
        // Wait on listen
        let mut events = [EpollEvent::empty()];
        epoll.wait(&mut events, MILLIS)?;

        for i in 0..events.len() {
            println!("{}", events[i].data());
            if events[i].data() == listener.as_raw_fd().try_into().unwrap() {
                println!("Before");
                for stream in listener.incoming() {
                    match stream {
                        Ok(s) => {
                            println!("Connected!");
                        },
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            epoll.wait(&mut events, MILLIS)?;
                            continue;
                        },
                        Err(e) => panic!("encountered IO error: {e}"),
                    }
                }
            }
        }
    }
    
    Ok(())
}
