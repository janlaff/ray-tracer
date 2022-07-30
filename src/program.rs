use std::ptr;
use gl::types::*;
use crate::{ResourceLoader, Shader};
use crate::util::create_whitespace_cstring;
use crate::error::Error;

pub struct Program {
    id: GLuint
}

impl Program {
    pub fn from_resource(loader: &ResourceLoader, name: &str) -> Result<Self, Error> {
        const POSSIBLE_KINDS: [&str; 3] = [".vert", ".frag", ".comp"];

        let shaders = POSSIBLE_KINDS.iter()
            .map(|ext| Shader::from_resource(loader, &format!("{}{}", name, ext)))
            .filter(|result| {
                result.as_ref().inspect_err(|error| println!("{:?}", error));
                result.is_ok()
            })
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(&shaders[..])
            .map_err(|message| Error::LinkError {name: name.to_owned(), message})
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = program_from_shaders(shaders)?;
        Ok(Program { id })
    }

    pub fn set_used(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn id(&mut self) -> GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

fn program_from_shaders(shaders: &[Shader]) -> Result<GLuint, String> {
    let program_id = unsafe { gl::CreateProgram() };

    for shader in shaders {
        unsafe { gl::AttachShader(program_id, shader.id()); }
    }

    unsafe { gl::LinkProgram(program_id); }

    let mut success: GLint = 1;
    unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success); }

    if success == 0 {
        let mut info_log_len: GLint = 0;
        unsafe { gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut info_log_len); }

        let mut info_log = create_whitespace_cstring(info_log_len as usize);
        unsafe {
            gl::GetProgramInfoLog(
                program_id,
                info_log_len,
                ptr::null_mut(),
                info_log.as_ptr() as *mut GLchar
            );
        }

        return Err(info_log.to_string_lossy().into_owned())
    }

    for shader in shaders {
        unsafe { gl::DetachShader(program_id, shader.id()); }
    }

    Ok(program_id)
}