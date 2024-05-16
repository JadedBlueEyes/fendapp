#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DirMode {
    Create,
    DontCreate,
}
use std::{env, error, ffi, fmt, fs, io, path};

#[derive(Debug)]
pub struct HomeDirError;

impl fmt::Display for HomeDirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unable to find home directory")
    }
}

impl error::Error for HomeDirError {}

impl From<HomeDirError> for io::Error {
    fn from(e: HomeDirError) -> Self {
        Self::new(io::ErrorKind::Other, e)
    }
}

fn get_home_dir() -> Result<path::PathBuf, HomeDirError> {
    let Some(home_dir) = home::home_dir() else {
        return Err(HomeDirError);
    };
    Ok(home_dir)
}

pub fn get_cache_dir(mode: DirMode) -> Result<path::PathBuf, io::Error> {
    // first try $FEND_CACHE_DIR
    if let Some(env_var_cache_dir) = env::var_os("FEND_CACHE_DIR") {
        let res = path::PathBuf::from(env_var_cache_dir);
        if mode == DirMode::Create {
            fs::create_dir_all(&res)?;
        }
        return Ok(res);
    }

    // otherwise try $XDG_CACHE_HOME/fend/
    if let Some(env_var_xdg_cache_dir) = env::var_os("XDG_CACHE_HOME") {
        let mut res = path::PathBuf::from(env_var_xdg_cache_dir);
        if mode == DirMode::Create {
            fs::create_dir_all(&res)?;
            mark_dir_as_hidden(&res);
        }
        res.push("fend");
        if mode == DirMode::Create {
            fs::create_dir_all(&res)?;
        }
        return Ok(res);
    }

    // otherwise use $HOME/.cache/fend/
    let mut res = get_home_dir()?;
    res.push(".cache");
    if mode == DirMode::Create {
        fs::create_dir_all(&res)?;
        mark_dir_as_hidden(&res);
    }
    res.push("fend");
    if mode == DirMode::Create {
        fs::create_dir_all(&res)?;
    }
    Ok(res)
}

fn mark_dir_as_hidden(path: &path::Path) {
    let Ok(metadata) = fs::metadata(path) else {
        return;
    };

    if !metadata.is_dir() {
        return;
    }

    let path = {
        let mut p = ffi::OsString::from("\\\\?\\");
        p.push(path.as_os_str());
        p
    };

    match mark_dir_as_hidden_impl(path.as_os_str()) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("failed to set cache directory as hidden: error code: {e}");
        }
    }
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn mark_dir_as_hidden_impl(path: &ffi::OsStr) -> Result<(), u32> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::{
        Foundation::GetLastError,
        Storage::FileSystem::{SetFileAttributesW, FILE_ATTRIBUTE_HIDDEN},
    };

    let path = path.encode_wide().chain(Some(0)).collect::<Vec<u16>>();

    unsafe {
        // https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-setfileattributesw
        let return_code = SetFileAttributesW(path.as_slice().as_ptr(), FILE_ATTRIBUTE_HIDDEN);
        if return_code == 0 {
            return Err(GetLastError());
        }
    }
    Ok(())
}

#[cfg(not(windows))]
#[allow(clippy::unnecessary_wraps)]
fn mark_dir_as_hidden_impl(_path: &ffi::OsStr) -> Result<(), u32> {
    Ok(())
}
