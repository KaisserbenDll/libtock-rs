use crate::callback::CallbackSubscription;
use crate::callback::Identity3Consumer;
use crate::result::*;
use crate::shared_memory::SharedMemory;
use crate::syscalls;
//use libtock_core::syscalls::allow;
use alloc::string;
//yield() is libtock_core::syscalls::raw::yieldk

const DRIVER_NUMBER: usize = 0x10000;

mod ipc_commands {
    pub const DISCOVER_SERVICE: usize = 0;
}

 pub struct IpcClientCallback {
    callback: Identity3Consumer,

 }
// impl<CB> IpcClientCallback<CB> {
//     pub fn new(callback: CB) -> Self {
//         IpcClientCallback { callback }
//     }
// }
//
// impl<CB: FnMut(usize, usize,usize)> Identity3Consumer for IpcClientCallback<CB> {
//     fn consume(&mut self, pid: usize, len: usize, _: usize) {
//         (self.callback)(pid, len);
//     }
// }

pub fn reserve_shared_buffer() -> IPCBuffer {
    IPCBuffer { buffer: [0; 32] }
}

#[repr(align(32))]
pub struct IPCBuffer {
    pub buffer: [u8; 32],
}

pub struct ServerHandle {
    pid: usize,
}
impl ServerHandle {
    // Performs service discovery
    // Returns the process identifier of the process with the given package name,
    // or a negative value on error.
    pub fn ipc_discover_service(mut name: string) -> Option<ServerHandle> {
        let len = name.len();
        let pid = unsafe {
            syscalls::allow( // Some ambiguities about the allow_ptr :(
                             DRIVER_NUMBER,
                             ipc_commands::DISCOVER_SERVICE,
                             name.as_bytes_mut(),//.as_mut_ptr(),
                             //len,
            )
        };
        if pid >= 0 {
            Some(ServerHandle { pid: pid as usize })
        } else {
            None
        }
    }
    // Registers a client callback for a particular service.
    //
    // `svc_id` is the (non-zero) process id of the service to subscribe to.
    //
    // Client callbacks are called in response to `notify`s from a particular
    // service and take the following arguments in order:
    //
    //   int pid   - the notifying service's process id
    //   int len   - the length of the shared buffer or zero if no buffer is shared
    //               from the service.
    //   char* buf - the base address of the shared buffer, or NULL if no buffer is
    //               shared from the service.
    //   void* ud  - `userdata`. same as the argument to this function.
    //              In our case its NULL
    pub fn ipc_register_client_cb<'a>(&self,callback: &'a mut IpcClientCallback,
    ) -> Result<CallbackSubscription<'a>, SubscribeError> {
        syscalls::subscribe(DRIVER_NUMBER, self.pid, callback)
    }
    // Send a notify to the service at the given process id
    pub fn ipc_notify_svc(&mut self) -> Result<usize, CommandError> {
        return unsafe { syscalls::command(DRIVER_NUMBER, self.pid as usize, 0, 0) };

    }
    // Share a buffer with the given process (either service or client)
    //
    // `pid` is the non-zero process id of the recipient.
    // `base` must be aligned to the value of `len`.
    // `len` must be a power-of-two larger than 16.
    pub fn ipc_share<'a>(&self, shared_buffer: &'a mut IPCBuffer) -> Result<SharedMemory<'a>, AllowError> {
        syscalls::allow(DRIVER_NUMBER, self.pid as usize, &mut shared_buffer.buffer)
    }
}
