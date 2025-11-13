mod bindings;
use bindings::*;

use std::{
    ffi::{CString, c_void},
    ptr,
};

struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
        }
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }

    fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    fn hex_to(&mut self, hex_str: &str) {
        unsafe {
            let c_str = CString::new(hex_str).unwrap();
            hexTo(self.as_mut_ptr() as *mut c_void, c_str.as_ptr());
        }
    }

    fn hex_from(&mut self, src: *const u8, len: usize) -> String {
        unsafe {
            hexFrom(self.as_mut_ptr() as *mut i8, src as *const c_void, len);
            self.data
                .iter()
                .take_while(|&&b| b != 0)
                .map(|&b| b as char)
                .collect()
        }
    }
}

fn main() {
    unsafe {
        println!("beltHash_keep: {}", beltHash_keep());

        let mut privkey = Buffer::new(64);
        let mut pubkey = Buffer::new(128);
        let mut hash = Buffer::new(64);
        let mut der = Buffer::new(512);
        let mut sig = Buffer::new(96);
        let mut buf = Buffer::new(128);

        let mut params = std::mem::zeroed::<bign_params>();

        let oid_std = CString::new("1.2.112.0.2.0.34.101.45.3.1").unwrap();
        bignParamsStd(&mut params, oid_std.as_ptr());

        println!("l: {}", params.l);
        println!("q: {}", buf.hex_from(params.q.as_ptr(), 32));

        privkey.hex_to("1F66B5B84B7339674533F0329C74F21834281FED0732429E0C79235FC273E269");

        let err = bignPubkeyCalc(pubkey.as_mut_ptr(), &params, privkey.as_ptr());
        println!("err: {}", err);
        println!("pubkey: {}", buf.hex_from(pubkey.as_ptr(), 64));

        let src = beltH();
        let err = beltHash(hash.as_mut_ptr(), src as *const c_void, 13);
        println!("err: {}", err);
        println!("hash: {}", buf.hex_from(hash.as_ptr(), 32));

        let mut count: usize = 512;
        let oid_der = CString::new("1.2.112.0.2.0.34.101.31.81").unwrap();
        bignOidToDER(der.as_mut_ptr(), &mut count, oid_der.as_ptr());
        println!("count: {}", count);

        let err = bignSign2(
            sig.as_mut_ptr(),
            &params,
            der.as_ptr(),
            count,
            hash.as_ptr(),
            privkey.as_ptr(),
            ptr::null(),
            0,
        );
        println!("err: {}", err);
        println!("sig: {}", buf.hex_from(sig.as_ptr(), 48));

        let err = bignVerify(
            &params,
            der.as_ptr(),
            count,
            hash.as_ptr(),
            sig.as_ptr(),
            pubkey.as_ptr(),
        );
        println!("verify err: {}", err);
    }
}
