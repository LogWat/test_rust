#![allow(non_snake_case)]
extern crate libc;
extern crate user32;
extern crate winapi;
extern crate kernel32;

use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::um::winnt::*;
use winapi::shared::minwindef::*;
use kernel32::*;

use std::mem::size_of;
use std::convert::TryInto;
use rand::Rng;

const DPATH: u32 = 0x4B5B4C;

#[no_mangle]
pub extern "stdcall" fn DllMain(
    _: winapi::shared::minwindef::HINSTANCE, 
    reason: winapi::shared::minwindef::DWORD,
    _: winapi::shared::minwindef::LPVOID
) -> i32 {

    match reason {
        winapi::um::winnt::DLL_PROCESS_ATTACH => {
            unsafe {
                if changedisplayname() == false {
                    let errtext = "Failed to change display name.\0".to_string();
                    err_msgbox(errtext);
                }
            }
            return 1;
        },
        _ => {}
    }

    TRUE
}

// 生ポインタの利用 *mut or *const
// この場合，アスタリスクは参照外しではなく型の一部である
unsafe extern "stdcall" fn changedisplayname() -> bool {

    let mut addr = *((*(DPATH as *mut i32) + 0x192C) as *mut i32); // [[0x4B5B4C] + 0x64B * 4]
    let num_of_characters = *(((*(DPATH as *mut i32)) + 0xCD4) as *mut i32); // [[0x4B5B4C] + 0x335 * 4]

    let mut last_page: DWORD = 0;
    let names: Vec<&[u8]> = vec![b"Hello, UnderWorld!\0", b"\\(^o^)/\0", b"OXOXOXOXOXOXOXOXOXOXOXOXO\0", b" \0", b"OMFG! Miko!!!\0"];

    for _ in 0..num_of_characters {
        if *((addr + 0x4) as *mut i32) == 0x7473694D && *((addr + 0x8) as *mut i32) == 0x6E656B61 {

            // 書き換えを行うために権限変更
            if VirtualProtect(
                addr as *mut _,
                ((size_of::<i32>() * 4) as u64).try_into().unwrap(),
                PAGE_READWRITE,
                &mut last_page
            ) == 0 {
                return false;
            }

            // Nameを書き換え
            let mut rng = rand::thread_rng();
            let name_index = rng.gen_range(0..(names.len() - 1));
            for i in 0..names[name_index].len() {
                *((addr + 0x4 + i as i32) as *mut u8) = names[name_index][i];
            }

            // 権限を元に戻す
            if VirtualProtect(
                addr as *mut _,
                ((size_of::<i32>() * 4) as u64).try_into().unwrap(),
                last_page,
                &mut last_page
            ) == 0 {
                return false;
            }
            
            return true;
        }
        addr += 0x438;
    }
    false
}

unsafe extern "stdcall" fn err_msgbox(text: String) {
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