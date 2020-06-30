use crate::callback::CallbackSubscription;
use crate::callback::Consumer;
//use crate::callback::SubscribableCallback;
use crate::result::*;
use crate::shared_memory::SharedMemory;
use crate::syscalls;
use crate::syscalls::platform;
//use libtock_core::syscalls::allow;
use alloc::string::String;
//yield() is libtock_core::syscalls::raw::yieldk
const DRIVER_NUMBER: usize = 0x10000;


mod ipc_commands {
    pub const DISCOVER_SERVICE: usize = 0;
}

#[allow(dead_code)]
pub struct IpcClientCallback<CB> {
    callback: CB,
}
impl<CB> IpcClientCallback<CB> {
    pub fn new(callback: CB) -> Self {
        IpcClientCallback { callback }
    }
}
impl<CB: FnMut(usize, usize)> Consumer<CB> for IpcClientCallback<CB> {
    fn consume(callback:&mut CB, pid: usize, len: usize, _: usize) {
        (callback)(pid, len);
    }
}

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
    pub fn ipc_discover_service(mut name: String) -> Option<ServerHandle> {
        let len = name.len();
        let pid = unsafe {
            platform::allow( // Some ambiguities about the allow_ptr :(
                             DRIVER_NUMBER,
                             ipc_commands::DISCOVER_SERVICE,
                             name.as_bytes_mut().as_mut_ptr(),
                             len,
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
    // pub fn ipc_register_client_cb<'a>(&self,callback: &'a mut IpcClientCallback,
    // ) -> Result<CallbackSubscription<'a>, SubscribeError> {
    //     syscalls::subscribe_fn(DRIVER_NUMBER, self.pid, callback,0)
    // }
    // pub fn ipc_register_client_cb<'a>(
    //     &self,
    //     callback: extern "C" fn(usize, usize, usize, usize),
    //     userdata: usize)
    //     -> Result<(), SubscribeError> {
    //     syscalls::subscribe_fn(DRIVER_NUMBER, self.pid, callback,userdata)
    // }
    pub fn ipc_register_client_cb<'a, CB: FnMut(usize, usize)>(
        &self,
        callback: &'a mut CB,
        ) -> TockResult<CallbackSubscription<'a>> {
        syscalls::subscribe::<IpcClientCallback<CB>, _>(
            DRIVER_NUMBER,
            self.pid,
            callback,
        )

        .map_err(Into::into)
    }
    // Send a notify to the service at the given process id
    pub fn ipc_notify_svc(&mut self) -> Result<usize, CommandError> {
        let res =
        syscalls::command(DRIVER_NUMBER, self.pid as usize, 0, 0);
        return res ;
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
