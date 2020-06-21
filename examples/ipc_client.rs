#![feature(alloc)]
#![no_std]

extern crate alloc;

use alloc::string::String;
use libtock::console::Console;
use libtock::fmt;
use libtock::ipc_client;
use libtock::ipc_client::IpcClientCallback;
use libtock::ipc_client::ServerHandle;
use libtock::timer;

// Calls the ipc_server and prints result
fn main() {
    let mut server_buf = ipc::reserve_shared_buffer();
    let mut my_buf = ipc::reserve_shared_buffer();
    timer::sleep(timer::Duration::from_ms(1000));

    loop {
        let mut server = ServerHandle::discover_service(String::from("ipcserver")).unwrap();
        let payload: [u8; 32] = [5; 32];

        let mut handle = server.share(&mut server_buf).unwrap();
        handle.write_bytes(&payload);

        let mut callback = IpcClientCallback::new(|_: usize, _: usize| {
            let mut console = Console::new();
            handle.read_bytes(&mut my_buf.buffer);
            console.write(String::from("Client: \"Payload: "));
            console.write(fmt::u32_as_decimal(my_buf.buffer[0] as u32));
            console.write(String::from("\"\n"));
        });

        let handle = server.subscribe_callback(&mut callback);

        server.notify().unwrap();
        timer::sleep(timer::Duration::from_ms(1000));
        handle.unwrap();
    }
}
