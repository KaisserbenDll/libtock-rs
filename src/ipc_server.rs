//use crate::callback::CallbackSubscription;
//use crate::callback::SubscribableCallback;
use crate::result::SubscribeError;
use crate::result::CommandError;
use crate::syscalls;
//use crate::callback::SubscribableCallback;
//use core::slice;

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
    pub fn ipc_register_svc<'a>(
        callback: extern "C" fn(usize, usize, usize, usize),
        userdata: usize)
        -> Result<(), SubscribeError> {
        syscalls::subscribe_fn(DRIVER_NUMBER, ipc_commands::REGISTER_SERVICE, callback,userdata)
    }
}