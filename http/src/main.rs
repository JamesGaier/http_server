use nix::sys::epoll::*;
use nix::sys::eventfd::{EventFd};
use std::time::Instant;
use nix::unistd::{read, write, pipe};
use std::os::unix::io::AsRawFd;

fn main() -> nix::Result<()> {
    const DATA: u64 = 17;
    const MILLIS: u8 = 100;
    
    // Create epoll
    let epoll = Epoll::new(EpollCreateFlags::empty())?;
    
    // Create eventfd & Add event
    let eventfd = EventFd::new()?;
    epoll.add(&eventfd, EpollEvent::new(EpollFlags::EPOLLIN,DATA))?;
    
    // Arm eventfd & Time wait
    eventfd.write(1)?;
    let now = Instant::now();
    
    // Wait on event
    let mut events = [EpollEvent::empty()];
    epoll.wait(&mut events, MILLIS)?;
    
    Ok(())
}
