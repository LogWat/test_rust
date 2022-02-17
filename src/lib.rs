#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;

mod processlib;
mod overwrite;
mod dbg;
mod threadpool;
mod ffi_helpers;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::um::{winnt::*, libloaderapi, processthreadsapi};
use winapi::shared::minwindef::*;

use processlib::Process;
use overwrite::{OverWrite, AddrSize};
use dbg::{Debugger};

use rand::Rng;
use std::process::{Command};

const DPATH: u32 = 0x4B5B4C;

#[no_mangle]
pub extern "stdcall" fn DllMain(
    hinst_dll: HINSTANCE, 
    reason: DWORD,
    _: LPVOID
) -> i32 {

    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                libloaderapi::DisableThreadLibraryCalls(hinst_dll);

                let process = Process::current_process();

                processthreadsapi::CreateThread(
                    0 as *mut _,
                    0,
                    Some(threadpool::Thread_Checker),
                    0 as *mut _,
                    0,
                    0 as *mut _
                );
                match overwrite::OverWrite(&process) {
                    Ok(_) => {},
                    Err(e) => {
                        let msg = format!("Failed to overwrite.\n{}", e);
                        err_msgbox(msg);
                    }
                };
                if changedisplayname(&process) == false {
                    let errtext = "Failed to change display name.\0".to_string();
                    err_msgbox(errtext);
                }
            }
            return true as i32;
        },
        DLL_PROCESS_DETACH => {
            return true as i32;
        },
        _ => true as i32,
    }
}

// 生ポインタの利用 *mut or *const
unsafe extern "stdcall" fn changedisplayname(process: &Process) -> bool {

    let mut addr = *((*(DPATH as *mut i32) + 0x192C) as *mut i32);
    let num_of_characters = *(((*(DPATH as *mut i32)) + 0xCD4) as *mut i32);

    let names: Vec<&[u8]> = vec![
        b"]pyyz95Bzgyq4\0", 
        b"I=KzK<:\0", 
        b"GQwrrpg\0", 
        b"ZXSR45X|~z444\0",
        b"/Q\0",
        b"/E\0",
        b"\0",
        b"!'!'!'!'!'!'!'!'!'!'!'!'!'\0",
        b")4+5X\\FAT^P[5)4+\0",
        b"A}pl2gp5tyy5vgtol95t{q5lz`5a}|{~5lz`2gp5a}p5z{yl5z{p5b}z2f5{za*\0",
        b"XF5qzv`xp{ata|z{5|f5gptyyl5f}|aal95qz{2a5lz`5a}|{~*\0",
    ];

    for _ in 0..num_of_characters {
        if *((addr + 0x4) as *mut i32) == 0x7473694D && *((addr + 0x8) as *mut i32) == 0x6E656B61 {

            // Nameを書き換え
            let mut rng = rand::thread_rng();
            let name_index = rng.gen_range(0..(names.len()));
            let mut byte_list: Vec<OverWrite> = Vec::new();

            for i in 0..names[name_index].len() {
                let mut d_byte: u8 = '\0' as u8;
                if names[name_index][i] != '\0' as u8 {
                    d_byte = names[name_index][i] ^ 21;
                }
                byte_list.push(
                    OverWrite {
                        addr: (addr + 0x4 + i as i32) as u32,
                        byte: AddrSize::Byte(d_byte),
                    }
                );
            }

            match overwrite::overwrite_process_list(&byte_list, process) {
                Ok(_) => {},
                Err(_e) => { return false; }
            };

            return true;
        }
        addr += 0x438;
    }
    false
}

unsafe fn err_msgbox(text: String) {
    let lp_text: Vec<u16> = text.encode_utf16().collect();
    let caption = "⚠Error⚠\0".to_string();
    let lp_caption: Vec<u16> = caption.encode_utf16().collect();

    MessageBoxW(
        std::ptr::null_mut(),
        lp_text.as_ptr(),
        lp_caption.as_ptr(),
        MB_OK
    );
}