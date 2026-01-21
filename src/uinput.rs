#![expect(non_snake_case)]
#![allow(unused)]

use libc;

const _IOC_NRBITS: u32 = 8;
const _IOC_TYPEBITS: u32 = 8;
const _IOC_SIZEBITS: u32 = 14;
const _IOC_DIRBITS: u32 = 2;

const _IOC_NRSHIFT: u32 = 0;
const _IOC_TYPESHIFT: u32 = (_IOC_NRSHIFT + _IOC_NRBITS);
const _IOC_SIZESHIFT: u32 = (_IOC_TYPESHIFT + _IOC_TYPEBITS);
const _IOC_DIRSHIFT: u32 = (_IOC_SIZESHIFT + _IOC_SIZEBITS);

const _IOC_NONE: u32 = 0;
const _IOC_WRITE: u32 = 1;
const _IOC_READ: u32 = 2;

const UINPUT_IOCTL_BASE: u32 = b'U' as u32;

const UI_SET_EVBIT: u32 = _IOW::<u32>(UINPUT_IOCTL_BASE, 100);
const UI_SET_KEYBIT: u32 = _IOW::<u32>(UINPUT_IOCTL_BASE, 101);

const EV_KEY: u16 = 0x01;
const EV_SYN: u16 = 0x00;

const KEY_SPACE: u16 = 0x39;
const KEY_F: u16 = 33;
const KEY_D: u16 = 32;
const KEY_DOWN: u16 = 108;
const KEY_ENTER: u16 = 28;
const KEY_ESC: u16 = 1;
const KEY_Q: u16 = 16;
const KEY_RIGHT: u16 = 106;

const UINPUT_MAX_NAME_SIZE: usize = 80;

const BUS_USB: u16 = 0x03;
const UINPUT_PATH: &str = "/dev/uinput";

const UI_DEV_CREATE: u32 = _IO(UINPUT_IOCTL_BASE, 1);
const UI_DEV_DESTROY: u32 = _IO(UINPUT_IOCTL_BASE, 2);

const SYN_REPORT: u16 = 0;

const fn _IO(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_NONE, ty, nr, 0)
}

const fn _IOC(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << _IOC_DIRSHIFT)
        | (ty << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | (size << _IOC_SIZESHIFT)
}

const fn _IOW<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_WRITE, ty, nr, core::mem::size_of::<T>() as u32)
}

#[repr(C)]
struct InputEvent {
    timeval: Timeval,
    ty: u16,
    code: u16,
    value: i32,
}

#[repr(C)]
struct Timeval {
    tv_sec: i64,
    tv_usec: i64,
}

fn emit(fd: i32, ty: u16, code: u16, value: i32) {
    let ie = InputEvent {
        timeval: Timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ty,
        code,
        value,
    };

    unsafe {
        dbg!(libc::write(
            fd,
            &ie as *const InputEvent as *const libc::c_void,
            std::mem::size_of::<libc::input_event>(),
        ));
    }
}

#[repr(C)]
struct UinputSetup {
    id: InputId,
    name: [u8; UINPUT_MAX_NAME_SIZE],
    ff_effects_max: u32,
}

#[repr(C)]
struct InputId {
    bustype: u16,
    vendor: u16,
    product: u16,
    version: u16,
}

use std::time::Duration;

pub fn run() {
    let mut usetup: UinputSetup = unsafe { std::mem::zeroed() };
    let dev = std::ffi::CString::new("/dev/uinput").unwrap();
    let path = dev.as_ptr() as *const i8;

    let fd = unsafe { libc::open(path, libc::O_WRONLY | libc::O_NONBLOCK) };
    dbg!(fd);

    assert!(fd > 0, "File Descriptor returned -1");

    unsafe {
        dbg!(libc::ioctl(fd, UI_SET_EVBIT as u64, EV_KEY as u32));
        dbg!(libc::ioctl(fd, UI_SET_KEYBIT as u64, KEY_SPACE as u32));

        dbg!(libc::ioctl(fd, UI_SET_EVBIT as u64, EV_KEY as u32));
        dbg!(libc::ioctl(fd, UI_SET_KEYBIT as u64, KEY_F as u32));

        dbg!(libc::ioctl(fd, UI_SET_EVBIT as u64, EV_KEY as u32));
        dbg!(libc::ioctl(fd, UI_SET_KEYBIT as u64, KEY_D as u32));

        dbg!(libc::ioctl(fd, UI_SET_EVBIT as u64, EV_KEY as u32));
        dbg!(libc::ioctl(fd, UI_SET_KEYBIT as u64, KEY_DOWN as u32));

        dbg!(libc::ioctl(fd, UI_SET_EVBIT as u64, EV_KEY as u32));
        dbg!(libc::ioctl(fd, UI_SET_KEYBIT as u64, KEY_ENTER as u32));

        dbg!(libc::ioctl(fd, UI_SET_EVBIT as u64, EV_KEY as u32));
        dbg!(libc::ioctl(fd, UI_SET_KEYBIT as u64, KEY_ESC as u32));

        dbg!(libc::ioctl(fd, UI_SET_EVBIT as u64, EV_KEY as u32));
        dbg!(libc::ioctl(fd, UI_SET_KEYBIT as u64, KEY_Q as u32));

        dbg!(libc::ioctl(fd, UI_SET_EVBIT as u64, EV_KEY as u32));
        dbg!(libc::ioctl(fd, UI_SET_KEYBIT as u64, KEY_RIGHT as u32));
    }

    let mut buffer = [0; 80];
    let name = b"Swift";
    buffer[..name.len()].copy_from_slice(name);
    println!("{:?}", buffer);

    usetup = UinputSetup {
        id: InputId {
            bustype: BUS_USB,
            vendor: 0x1234,
            product: 0x5678,
            version: 1,
        },
        name: buffer,
        ff_effects_max: 0,
    };
    println!("{:?}", buffer);

    const UI_DEV_SETUP: u32 = _IOW::<libc::uinput_setup>(UINPUT_IOCTL_BASE, 3);

    println!("InputId: {}", size_of::<InputId>());
    println!("UinputSetup: {}", size_of::<UinputSetup>());

    unsafe {
        dbg!(libc::ioctl(
            fd,
            UI_DEV_SETUP as u64,
            &usetup as *const _ as *const libc::c_void
        ));
        dbg!(libc::ioctl(fd, UI_DEV_CREATE as u64));
    }

    sleep(3000);

    loop {
        press_key(fd, KEY_F);
        press_key(fd, KEY_D);
        press_key(fd, KEY_D);
        press_key(fd, KEY_DOWN);
        press_key(fd, KEY_DOWN);
        press_key(fd, KEY_DOWN);
        press_key(fd, KEY_ENTER);
        press_key(fd, KEY_RIGHT);
        press_key(fd, KEY_ENTER);
        press_key(fd, KEY_ESC);
        sleep(1000);
        press_key(fd, KEY_Q);
        sleep(1500);
        press_key(fd, KEY_Q);
        sleep(1500);
    }

    sleep(3000);

    unsafe {
        dbg!(libc::ioctl(fd, UI_DEV_DESTROY as u64));
        dbg!(libc::close(fd));
    }
}

fn sleep(milis: u64) {
    std::thread::sleep(Duration::from_millis(milis));
}

fn press_key(fd: i32, key: u16) {
    dbg!(emit(fd, EV_KEY, key, 1));
    dbg!(emit(fd, EV_SYN, SYN_REPORT, 0));
    dbg!(emit(fd, EV_KEY, key, 0));
    dbg!(emit(fd, EV_SYN, SYN_REPORT, 0));
    sleep(100);
}
