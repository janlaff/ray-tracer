use std::env;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::error::Error;

pub struct ResourceLoader {
    root_path: PathBuf
}

impl ResourceLoader {
    pub fn from_absolute_path(abs_path: &Path) -> Self {
        unimplemented!()
    }

    pub fn from_relative_path(rel_path: &Path) -> Result<Self, Error> {
        let exe_file = env::current_exe()
            .map_err(|_| Error::FailedToGetExePath)?;

        let exe_path = exe_file.parent()
            .ok_or(Error::FailedToGetExePath)?;

        Ok(ResourceLoader {
            root_path: exe_path.join(rel_path)
        })
    }

    pub fn load_cstring(&self, name: &str) -> Result<CString, Error> {
        let mut file = File::open(
            resolve_resource_name(&self.root_path, name)
        )?;

        let mut buf = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut buf)?;

        Ok(unsafe { CString::from_vec_unchecked(buf) })
    }
}

fn resolve_resource_name(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}