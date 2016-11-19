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

#[no_mangle]
pub extern "C" fn create_ios_client() -> *mut Core {
    let core : Box<Core> = Box::new(IOSCore::create());
    Box::into_raw(core) as *mut Core
}

#[no_mangle]
pub extern "C" fn create_android_client() -> *mut Core {
    let core : Box<Core> = Box::new(AndroidCore::create());
    Box::into_raw(core)
}

#[no_mangle]
pub extern "C" fn destroy(c: *mut Core) {
    let _: Box<Core> = unsafe { Box::from_raw(c) };
}

#[no_mangle]
pub extern "C" fn inspect(c: *mut Core) {
    unsafe {
        let core: Box<Core> = Box::from_raw(c);
        core.inspect();
    };
}