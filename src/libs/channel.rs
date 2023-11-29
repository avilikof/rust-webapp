use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};

pub fn create_channel(set_buff_size: Option<usize>) -> (SyncSender<String>, Receiver<String>) {
    let mut buff_size: usize = 0;
    match set_buff_size {
        None => (),
        Some(size) => buff_size = size,
    }
    mpsc::sync_channel::<String>(buff_size)
}
