use std::ffi::{CStr, CString};
use std::io::Read;
use std::ptr;
use gl::types::*;
use crate::util::create_whitespace_cstring;
use crate::error::Error;
use crate::ResourceLoader;

pub struct Shader {
    id: GLuint
}

impl Shader {
    pub fn from_resource(loader: &ResourceLoader, name: &str) -> Result<Self, Error> {
        const POSSIBLE_KINDS: [(&str, GLenum); 3] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER),
            (".comp", gl::COMPUTE_SHADER),
        ];

        let kind = POSSIBLE_KINDS.iter()
            .find(|&(ext, _)| name.ends_with(ext))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::UnknownShaderType(name.to_owned()))?;

        let source = loader.load_cstring(name)?;
        Shader::from_source(&source, kind)
            .map_err(|message| Error::CompileError {name: name.to_owned(), message})
    }

    fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Self, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Self, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: GLenum) -> Result<GLuint, String> {
    let shader_id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(shader_id, 1, &source.as_ptr(), ptr::null());
        gl::CompileShader(shader_id);
    }

    let mut success: GLint = 1;
    unsafe { gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success); }

    if success == 0 {
        let mut info_log_len: GLint = 0;
        unsafe { gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_log_len); }

        let mut info_log = create_whitespace_cstring(info_log_len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                shader_id,
                info_log_len,
                ptr::null_mut(),
                info_log.as_ptr() as *mut GLchar
            );
        }

        Err(info_log.to_string_lossy().into_owned())
    } else {
        Ok(shader_id)
    }
}