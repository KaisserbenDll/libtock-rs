#![no_std]
//extern crate alloc ;
//use alloc::string::String;
//use core::fmt::Write;

use libtock::syscalls;
use libtock::result::TockResult;
use libtock::ipc_server::*;

#[libtock::main]
async fn main()  -> TockResult<()>{
    let  drivers = libtock::retrieve_drivers()?;
    let mut console = drivers.console.create_console();

    // writeln!(console, "Starting COM Process Service")?;
    console.write("Hello Tock World Server PROCESS\n")?;
    //console.write( "Starting COM Process Service\n")?;
    console.write( "Starting Server Process Service\n")?;

    let mut callback = |pid: usize, _: usize, message: &mut [u8]| {
        console.write ("Server: \"Payload:");
        console.write(message);
        console.write("\"\n");
        // console.write(String::from("Server: \"Payload: "))?;
        // // console.write(message[0] as u32);
        // console.write(message)?;
        // console.write(String::from("\"\n"))?;
        // //message[0] += 1 ;
       ipc_notify_client(pid);
    };
    let _server = IpcServerDriver::ipc_register_svc(&mut callback)?;

    loop {
           unsafe { syscalls::raw::yieldk() };
    }
    _server.unwrap();
}
