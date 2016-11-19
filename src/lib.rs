extern crate libc;

struct IOSCore {
    internal: String
}

impl Core for IOSCore {
    fn create() -> IOSCore {
        IOSCore { internal: "IOS!!".into() }
    }

    fn inspect(&self) {
        println!("Hello from IOS!");
    }
}

struct AndroidCore {
    internal: String
}

impl Core for AndroidCore {
    fn create() -> AndroidCore {
        AndroidCore { internal: "Android!!".into() }
    }

    fn inspect(&self) {
        println!("Hello from Android!");
    }
}

pub trait Core {
    fn create() -> Self where Self: Sized;
    fn inspect(&self);
}

#[repr(C)]
pub struct CoreClient {
    destroy: extern "C" fn(*mut std::os::raw::c_void) -> (),
    inspect: extern "C" fn(*mut std::os::raw::c_void) -> (),
    core: *mut std::os::raw::c_void,
}

impl<C: Core> From<Box<C>> for CoreClient {
    fn from(i: Box<C>) -> CoreClient {
        CoreClient {
            destroy: destroy_core::<C>,
            inspect: inspect_core::<C>,
            core: Box::into_raw(i) as *mut std::os::raw::c_void,
        }
    }
}

extern "C" fn destroy_core<C: Core>(c: *mut std::os::raw::c_void) {
    unsafe {
        let client = c as *mut CoreClient;
        Box::from_raw((*client).core as *mut C);
    }
}

extern "C" fn inspect_core<C: Core>(c: *mut std::os::raw::c_void) {
    unsafe {
        let client = c as *mut CoreClient;
        let core: Box<C> = Box::from_raw((*client).core as *mut C);
        core.inspect();
        std::mem::forget(client);
        std::mem::forget(core);
    }
}


#[no_mangle]
pub extern "C" fn create_ios_client() -> *mut CoreClient {
    let core = Box::new(Box::new(IOSCore::create()).into());
    Box::into_raw(core)
}

#[no_mangle]
pub extern "C" fn create_android_client() -> *mut CoreClient {
    let core = Box::new(Box::new(AndroidCore::create()).into());
    Box::into_raw(core)
}

#[no_mangle]
pub extern "C" fn destroy(c: *mut CoreClient) {
    let client = unsafe { Box::from_raw(c) };
    (client.destroy)(c as *mut std::os::raw::c_void);
}

#[no_mangle]
pub extern "C" fn inspect(c: *mut CoreClient) {
    unsafe { ((*c).inspect)(c as *mut std::os::raw::c_void) };
}