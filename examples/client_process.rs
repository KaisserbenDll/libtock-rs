#![no_std]

use libtock::result::TockResult;
//use core::fmt::Write;
use libtock::timer::Duration;
use libtock::ipc_client;
use libtock::ipc_client::ServerHandle;
use libtock::ipc_client::IpcClientCallback;

#[libtock::main]
async fn main() -> TockResult<()> {
    let mut drivers = libtock::retrieve_drivers()?;

    let mut timer_driver = drivers.timer.create_timer_driver();
    let timer_driver = timer_driver.activate()?;

    let mut console = drivers.console.create_console();
    console.write("Hello Tock World Client PROCESS\n")?;
    console.write( "Starting Client Process Client\n")?;
    timer_driver.sleep(Duration::from_ms(5000)).await?;
    Ok(())

    let mut server_buff = ipc_client::reserve_shared_buffer();
    let mut my_buf = ipc_client::reserve_shared_buffer();

    timer_driver.sleep(Duration::from_ms(1000)).await?;

    loop {
        let mut server = ServerHandle::ipc_discover_service(String::from("com_process")).unwrap();
        let payload: [u8; 32] = [5; 32];

        let mut handle = server.ipc_share(&mut server_buf).unwrap();
        handle.write_bytes(&payload);

        let mut callback =|_: usize, _: usize| {
            handle.read_bytes(&mut my_buf.buffer);
            console.write(String::from("Client: \"Payload: "));
            console.write(my_buf.buffer[0]);
            console.write(String::from("\"\n"));
        };

        let handle = server.ipc_register_client_cb(&mut callback);

        server.ipc_notify_svc().unwrap();
        timer_driver.sleep(Duration::from_ms(1000)).await?;
        handle.unwrap();
    }



}
