use crate::callback::CallbackSubscription;
//use crate::callback::SubscribableCallback;
use crate::result::SubscribeError;
use crate::syscalls;
use crate::callback::Identity3Consumer;
//use core::slice;

const DRIVER_NUMBER: usize = 0x10000;

mod ipc_commands {
    pub const REGISTER_SERVICE: usize = 0;
    pub const NOTIFY_CLIENT: usize = 1;
}

pub struct IpcServerCallback {
    callback: Identity3Consumer,
}

// impl<CB> IpcServerCallback<CB> {
//     pub fn new(callback: CB) -> Self {
//         IpcServerCallback { callback }
//     }
// }

// impl<CB: FnMut(usize, usize, &mut [u8])> SubscribableCallback for IpcServerCallback<CB> {
//     fn call_rust(&mut self, arg0: usize, arg1: usize, arg2: usize) {
//         // FIXME: This is unsafe because IpcServerCallback can be subscribed on any driver
//         let mut v = unsafe { slice::from_raw_parts_mut(arg2 as *mut u8, arg1) };
//         (self.callback)(arg0, arg1, &mut v);
//     }
// }
// Send a notify to the client at the given process id
pub fn ipc_notify_client(pid: usize) {
    unsafe { syscalls::command(DRIVER_NUMBER, pid, ipc_commands::NOTIFY_CLIENT, 0) };
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
    pub fn ipc_register_svc(callback: &mut IpcServerCallback) -> Result<CallbackSubscription, SubscribeError>    {
        syscalls::subscribe(DRIVER_NUMBER, ipc_commands::REGISTER_SERVICE, callback)
    }
}