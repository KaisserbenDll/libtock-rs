use crate::callback::CallbackSubscription;
use crate::callback::Consumer;
use crate::result::*;
use crate::syscalls;
use core::slice;

const DRIVER_NUMBER: usize = 0x10000;

mod ipc_commands {
    pub const REGISTER_SERVICE: usize = 0;
    pub const NOTIFY_CLIENT: usize = 1;
}

// Send a notify to the client at the given process id
pub fn ipc_notify_client(pid: usize) -> Result<usize, CommandError> {
      let res =  syscalls::command(DRIVER_NUMBER, pid, ipc_commands::NOTIFY_CLIENT, 0) ;
      return res;
}
#[allow(dead_code)]
pub struct IpcServerCallback<CB>{
    callback: CB,
}

impl<CB> IpcServerCallback<CB> {
    pub fn new(callback: CB) -> Self {
        IpcServerCallback { callback }
    }
}
impl<CB: FnMut(usize, usize, &mut [u8])> Consumer<CB> for IpcServerCallback<CB> {
    fn consume(callback:&mut CB, pid: usize, len: usize, message: usize) {
        let mut v = unsafe {slice::from_raw_parts_mut(message as *mut u8,len )} ;
        (callback)(pid, len, &mut v );
    }
}
pub struct IpcServerDriver;

impl IpcServerDriver {
    // Registers a service callback for this process.
    //
    // Service callbacks are called in response to `notify`s from clients and take
    // the following arguments in order:
    //
    //   int pid   - the notifying client's process id
    //   int len   - the length of the shared buffer or zero if no buffer is shared
    //               from the client.
    //   char* buf - the base address of the shared buffer, or NULL if no buffer is
    //               shared from the client.
    //   void* ud  - `userdata`. same as the argument to this function.
    pub fn ipc_register_svc<CB: FnMut(usize, usize, &mut [u8])>(
        callback: &mut CB
        ) -> TockResult<CallbackSubscription> {
        syscalls::subscribe::<IpcServerCallback<CB>,_>(
            DRIVER_NUMBER,
            ipc_commands::REGISTER_SERVICE,
            callback,
        )
        .map_err(Into::into)
    }
}