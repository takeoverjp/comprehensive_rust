// TODO: 実装が完了したら、これを削除します。
#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

    // オペーク型。https://doc.rust-lang.org/nomicon/ffi.html をご覧ください。
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // readdir(3) の Linux マニュアル ページに沿ったレイアウト。ino_t と
    // off_t は
    // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h} の定義に従って解決されます。
    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    // macOSマニュアル ページのdir(5)に沿ったレイアウト。
    #[cfg(target_os = "macos")]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;

        #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        // https://github.com/rust-lang/libc/issues/414、および  macOS 版マニュアル ページのstat(2)における
        // _DARWIN_FEATURE_64_BIT_INODE に関するセクションをご覧ください。
        //
        // 「これらのアップデートが利用可能になる前に存在していたプラットフォーム("Platforms that existed before these updates were available")」とは、
        // Intel および PowerPC 上の macOS（iOS / wearOS などではない）を指します。
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // opendir を呼び出し、成功した場合は Ok 値を返し、
        // それ以外の場合はメッセージとともに Err を返します。
        let dir = ffi::opendir(path.into());
        if dir == std::ptr::null() {
            Err("".to_string())
        } else {
            Ok(path.into(), dir)
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // NULL ポインタが返されるまで readdir を呼び出し続けます。
        unimplemented!()
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // 必要に応じて closedir を呼び出します。
        unimplemented!()
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}
