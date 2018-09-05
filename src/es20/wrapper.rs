use std;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem::size_of;
use std::str::from_utf8;

use libc::{c_char};

use super::ffi;
use types::*;
use consts::*;
use enums::TextureUnit;
use enums::BufferTarget;
use enums::FrameBufferTarget;
use enums::RenderBufferTarget;
use enums::TextureBindTarget;
use enums::BlendEquationMode;
use enums::BlendFactor;
use enums::BufferUsage;
use enums::FrameBufferStatus;
use enums::TextureTarget;
use enums::ShaderType;
use enums::FaceMode;
use enums::FuncType;
use enums::FeatureType;
use enums::BeginMode;
use enums::FrameBufferAttachmentType;
use enums::FrontFaceDirection;
use enums::StateType;
use enums::BufferParamName;
use enums::ErrorType;
use enums::FrameBufferAttachmentParamType;
use enums::ProgramParamType;
use enums::RenderBufferParamType;
use enums::ShaderParamType;
use enums::ShaderPrecisionType;
use enums::ConstantType;
use enums::TextureParamType;
use enums::VertexAttributeParamType;
use enums::HintTargetType;
use enums::HintBehaviorType;
use enums::PackParamType;
use enums::PixelFormat;
use enums::PixelDataType;
use enums::ActionType;
use enums::DataType;

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

pub struct Active {
    pub name: String,
    pub size: i32,
    pub type_: DataType,
    pub length: i32,
}

pub struct ShaderPrecisionFormat {
    pub precision: i32,
    pub range: [i32; 2],
}

pub struct Error {

}

pub struct Wrapper {
}

trait Interceptor {
    fn intercept(&mut self, fun_name: &str) -> Result<(), Error>;
}

impl Wrapper {
    pub fn gl_active_texture(&mut self, texture_unit: TextureUnit) -> Result<(), Error> {
        unsafe {
            ffi::glActiveTexture(texture_unit as GLenum);
        }

        Ok(())
    }

    pub fn gl_attach_shader(&mut self, program: u32, shader: u32) -> Result<(), Error> {
        unsafe {
            ffi::glAttachShader(program as GLuint, shader as GLuint);
        }

        Ok(())
    }

    pub fn gl_bind_attrib_location(&mut self, program: u32, index: u32, name: &str) -> Result<(), Error> {
        unsafe {
            let c_str = CString::new(name).unwrap();

            ffi::glBindAttribLocation(program as GLuint, index as GLuint,
                                      c_str.as_ptr() as *const GLchar);
        }

        Ok(())
    }

    pub fn gl_bind_buffer(&mut self, target: BufferTarget, buffer: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBindBuffer(target as GLenum, buffer as GLuint);
        }

        Ok(())
    }

    pub fn gl_bind_framebuffer(&mut self, target: FrameBufferTarget, framebuffer: GLuint) -> Result<(), Error> {
        unsafe {
            ffi::glBindFramebuffer(target as GLenum, framebuffer as GLuint);
        }

        Ok(())
    }

    pub fn gl_bind_renderbuffer(&mut self, target: RenderBufferTarget, renderbuffer: u32) -> Result<(), Error> {
        unsafe {
            ffi::glBindRenderbuffer(target as GLenum, renderbuffer as GLuint);
        }

        Ok(())
    }

    pub fn gl_bind_texture(&mut self, target: TextureBindTarget, texture: u32) -> Result<(), Error> {
        unsafe {
            ffi::glBindTexture(target as GLenum, texture as GLuint)
        }

        Ok(())
    }

    pub fn gl_blend_color(&mut self, red: f32, green: f32, blue: f32,
                          alpha: f32) -> Result<(), Error> {
        unsafe {
            ffi::glBlendColor(red as GLclampf, green as GLclampf,
                              blue as GLclampf, alpha as GLclampf)
        }

        Ok(())
    }

    pub fn gl_blend_equation(&mut self, mode: BlendEquationMode) -> Result<(), Error> {
        unsafe {
            ffi::glBlendEquation(mode as GLenum)
        }

        Ok(())
    }

    pub fn gl_blend_equation_separate(&mut self, mode_rgb: BlendEquationMode, mode_alpha: BlendEquationMode)
                                      -> Result<(), Error> {
        unsafe {
            ffi::glBlendEquationSeparate(mode_rgb as GLenum, mode_alpha as GLenum)
        }

        Ok(())
    }

    pub fn gl_blend_func(&mut self, src_factor: BlendFactor, dst_factor: BlendFactor) -> Result<(), Error> {
        unsafe {
            ffi::glBlendFunc(src_factor as GLenum, dst_factor as GLenum)
        }

        Ok(())
    }

    pub fn gl_blend_func_separate(&mut self, src_rgb: BlendFactor, dst_rgb: BlendFactor,
                                  src_alpha: BlendFactor, dst_alpha: BlendFactor) -> Result<(), Error> {
        unsafe {
            ffi::glBlendFuncSeparate(src_rgb as GLenum, dst_rgb as GLenum,
                                     src_alpha as GLenum, dst_alpha as GLenum)
        }

        Ok(())
    }

    pub fn gl_buffer_data<T>(&mut self, target: BufferTarget,
                             buffer: &[T], usage: BufferUsage) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            let t_size = size_of::<T>();

            ffi::glBufferData(
                target as GLenum,
                (buffer.len() * t_size) as GLsizeiptr,
                buffer.as_ptr() as *const GLvoid,
                usage as GLenum,
            )
        }

        Ok(())
    }

    pub fn gl_buffer_sub_data<T>(&mut self, target: BufferTarget, offset: u32, buffer: &[T])
                                 -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            let t_size = size_of::<T>();

            ffi::glBufferSubData(
                target as GLenum,
                (offset * (t_size as u32)) as GLintptr,
                (buffer.len() * t_size) as GLsizeiptr,
                buffer.as_ptr() as *const GLvoid,
            )
        }

        Ok(())
    }

    pub fn gl_check_framebuffer_status(&mut self, target: FrameBufferTarget) -> Result<FrameBufferStatus, Error> {
        unsafe {
            let status = ffi::glCheckFramebufferStatus(target as GLenum);

            Ok(FrameBufferStatus::from(status))
        }
    }

    pub fn gl_clear(&mut self, mask: u32) -> Result<(), Error> {
        unsafe {
            ffi::glClear(mask as GLbitfield)
        }

        Ok(())
    }

    pub fn gl_clear_color(&mut self, red: f32, green: f32,
                          blue: f32, alpha: f32) -> Result<(), Error> {
        unsafe {
            ffi::glClearColor(red as GLclampf, green as GLclampf, blue as GLclampf,
                              alpha as GLclampf)
        }

        Ok(())
    }

    pub fn gl_clear_depthf(&mut self, depth: f32) -> Result<(), Error> {
        unsafe {
            ffi::glClearDepthf(depth as GLclampf)
        }

        Ok(())
    }

    pub fn gl_clear_stencil(&mut self, stencil: i32) -> Result<(), Error> {
        unsafe {
            ffi::glClearStencil(stencil as GLint)
        }

        Ok(())
    }


    pub fn gl_color_mask(&mut self, red: bool, green: bool, blue: bool, alpha: bool) -> Result<(), Error> {
        unsafe {
            ffi::glColorMask(
                red as GLboolean,
                green as GLboolean,
                blue as GLboolean,
                alpha as GLboolean,
            )
        }

        Ok(())
    }

    pub fn gl_compile_shader(&mut self, shader: u32) -> Result<(), Error> {
        unsafe {
            ffi::glCompileShader(shader as GLuint)
        }

        Ok(())
    }

    // TODO: internal_format 仍然是GLenum
    // OpenGL ES defines no specific compressed texture formats,
    // but does provide a mechanism to obtain symbolic constants
    // for such formats provided by extensions. The number of compressed
    // texture formats supported can be obtained by querying the value of
    // GL_NUM_COMPRESSED_TEXTURE_FORMATS. The list of specific compressed
    // texture formats supported can be obtained by querying the value of
    // GL_COMPRESSED_TEXTURE_FORMATS.

    pub fn gl_compressed_tex_image_2d<T>(
        &mut self,
        target: TextureTarget,
        level: i32,
        internal_format: GLenum,
        width: u32,
        height: u32,
        border: u32,
        image_size: u32,
        buffer: &[T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glCompressedTexImage2D(
                target as GLenum,
                level as GLint,
                internal_format,
                width as GLsizei,
                height as GLsizei,
                border as GLint,
                image_size as GLsizei,
                buffer.as_ptr() as *const GLvoid,
            )
        }

        Ok(())
    }

    pub fn gl_compressed_tex_sub_image_2d<T>(
        &mut self,
        target: TextureTarget,
        level: u32,
        x_offset: u32,
        y_offset: u32,
        width: u32,
        height: u32,
        format: GLenum,
        image_size: u32,
        buffer: &[T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glCompressedTexSubImage2D(
                target as GLenum,
                level as GLint,
                x_offset as GLint,
                y_offset as GLint,
                width as GLsizei,
                height as GLsizei,
                format,
                image_size as GLsizei,
                buffer.as_ptr() as *const GLvoid,
            )
        }

        Ok(())
    }

    pub fn gl_copy_tex_image_2d(
        &mut self,
        target: TextureTarget,
        level: u32,
        internal_format: GLenum,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        border: u32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCopyTexImage2D(target as GLenum,
                                  level as GLint,
                                  internal_format,
                                  x as GLint, y as GLint,
                                  width as GLsizei, height as GLsizei, border as GLint)
        }

        Ok(())
    }

    pub fn gl_copy_tex_sub_image_2d(
        &mut self,
        target: TextureTarget,
        level: u32,
        x_offset: u32,
        y_offset: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glCopyTexSubImage2D(target as GLenum,
                                     level as GLint,
                                     x_offset as GLint, y_offset as GLint,
                                     x as GLint, y as GLint,
                                     width as GLsizei, height as GLsizei)
        }

        Ok(())
    }

    pub fn gl_create_program(&mut self) -> Result<u32, Error> {
        unsafe {
            let program_id = ffi::glCreateProgram();

            Ok(program_id as u32)
        }
    }

    pub fn gl_create_shader(&mut self, type_: ShaderType) -> Result<u32, Error> {
        unsafe {
            let shader_id = ffi::glCreateShader(type_ as GLenum);

            Ok(shader_id as u32)
        }
    }

    pub fn gl_cull_face(&mut self, mode: FaceMode) -> Result<(), Error> {
        unsafe {
            ffi::glCullFace(mode as GLenum)
        }

        Ok(())
    }

    pub fn gl_delete_buffers(&mut self, buffers: &[u32]) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteBuffers(buffers.len() as GLsizei, buffers.as_ptr())
        }

        Ok(())
    }

    pub fn gl_delete_framebuffers(&mut self, framebuffers: &[u32]) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_ptr())
        }

        Ok(())
    }

    pub fn gl_delete_program(&mut self, program: u32) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteProgram(program as GLuint)
        }

        Ok(())
    }

    pub fn gl_delete_renderbuffers(&mut self, renderbuffers: &[u32]) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteRenderbuffers(renderbuffers.len() as GLsizei,
                                       renderbuffers.as_ptr())
        }

        Ok(())
    }

    pub fn gl_delete_shader(&mut self, shader: u32) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteShader(shader as GLuint)
        }

        Ok(())
    }

    pub fn gl_delete_textures(&mut self, textures: &[u32]) -> Result<(), Error> {
        unsafe {
            ffi::glDeleteTextures(textures.len() as GLsizei, textures.as_ptr())
        }

        Ok(())
    }

    pub fn gl_depth_func(&mut self, func: FuncType) -> Result<(), Error> {
        unsafe {
            ffi::glDepthFunc(func as GLenum)
        }

        Ok(())
    }

    pub fn gl_depth_mask(&mut self, flag: bool) -> Result<(), Error> {
        unsafe {
            ffi::glDepthMask(flag as GLboolean)
        }

        Ok(())
    }


    pub fn gl_depth_rangef(&mut self, z_near: f32, z_far: f32) -> Result<(), Error> {
        unsafe {
            ffi::glDepthRangef(z_near as GLclampf, z_far as GLclampf)
        }

        Ok(())
    }

    pub fn gl_detach_shader(&mut self, program: u32, shader: u32) -> Result<(), Error> {
        unsafe {
            ffi::glDetachShader(program as GLuint, shader as GLuint)
        }

        Ok(())
    }

    pub fn gl_disable(&mut self, feature: FeatureType) -> Result<(), Error> {
        unsafe {
            ffi::glDisable(feature as GLenum)
        }

        Ok(())
    }

    pub fn gl_disable_vertex_attrib_array(&mut self, index: u32) -> Result<(), Error> {
        unsafe {
            ffi::glDisableVertexAttribArray(index as GLuint)
        }

        Ok(())
    }

    pub fn gl_draw_arrays(&mut self, mode: BeginMode, first: i32, count: i32) -> Result<(), Error> {
        unsafe {
            ffi::glDrawArrays(mode as GLenum, first as GLint, count as GLsizei)
        }

        Ok(())
    }

    // TODO: type_ & T is reasonable ?
    pub fn gl_draw_elements<T>(&mut self, mode: BeginMode, count: i32, type_: GLenum, indices: &[T]) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glDrawElements(mode as GLenum, count as GLsizei,
                                type_, indices.as_ptr() as *const GLvoid)
        }

        Ok(())
    }

    pub fn gl_enable(&mut self, feature: FeatureType) -> Result<(), Error> {
        unsafe {
            ffi::glEnable(feature as GLenum)
        }

        Ok(())
    }

    pub fn gl_enable_vertex_attrib_array(&mut self, index: u32) -> Result<(), Error> {
        unsafe {
            ffi::glEnableVertexAttribArray(index as GLuint)
        }

        Ok(())
    }

    pub fn gl_finish(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glFinish()
        }

        Ok(())
    }

    pub fn gl_flush(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glFlush()
        }

        Ok(())
    }

    pub fn gl_framebuffer_renderbuffer(
        &mut self,
        target: FrameBufferTarget,
        attachment: FrameBufferAttachmentType,
        renderbuffer_target: RenderBufferTarget,
        renderbuffer: u32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glFramebufferRenderbuffer(target as GLenum,
                                           attachment as GLenum,
                                           renderbuffer_target as GLenum,
                                           renderbuffer as GLuint)
        }

        Ok(())
    }

    pub fn gl_framebuffer_texture_2d(
        &mut self,
        target: FrameBufferTarget,
        attachment: FrameBufferAttachmentType,
        texture_target: TextureTarget,
        texture: u32,
        level: i32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glFramebufferTexture2D(target as GLenum,
                                        attachment as GLenum,
                                        texture_target as GLenum,
                                        texture as GLuint,
                                        level as GLint)
        }

        Ok(())
    }

    pub fn gl_front_face(&mut self, mode: FrontFaceDirection) -> Result<(), Error> {
        unsafe {
            ffi::glFrontFace(mode as GLenum)
        }

        Ok(())
    }

    pub fn gl_gen_buffers(&mut self, count: u32) -> Result<Vec<u32>, Error> {
        unsafe {
            let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

            ffi::glGenBuffers(count as GLsizei, vec.as_mut_ptr());

            vec.set_len(count as usize);

            Ok(vec)
        }
    }

    pub fn gl_generate_mipmap(&mut self, target: TextureBindTarget) -> Result<(), Error> {
        unsafe {
            ffi::glGenerateMipmap(target as GLenum)
        }

        Ok(())
    }

    pub fn gl_gen_framebuffers(&mut self, count: u32) -> Result<Vec<u32>, Error> {
        unsafe {
            let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

            ffi::glGenFramebuffers(count as GLsizei, vec.as_mut_ptr());

            vec.set_len(count as usize);
            Ok(vec)
        }
    }

    pub fn gl_gen_renderbuffers(&mut self, count: u32) -> Result<Vec<u32>, Error> {
        unsafe {
            let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

            ffi::glGenRenderbuffers(count as GLsizei, vec.as_mut_ptr());

            vec.set_len(count as usize);
            Ok(vec)
        }
    }

    pub fn gl_gen_textures(&mut self, count: u32) -> Result<Vec<u32>, Error> {
        unsafe {
            let mut vec: Vec<u32> = Vec::with_capacity(count as usize);

            ffi::glGenTextures(count as GLsizei, vec.as_mut_ptr());

            vec.set_len(count as usize);
            Ok(vec)
        }
    }

    pub fn gl_get_active_attrib(&mut self, program: u32, index: u32) -> Result<Active, Error> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut size: GLint = 0;
            let mut attrib_type: GLenum = 0;

            let mut name = String::with_capacity(256);

            ffi::glGetActiveAttrib(
                program as GLuint,
                index as GLuint,
                256,
                &mut length,
                &mut size,
                &mut attrib_type,
                name.as_mut_vec().as_mut_ptr() as *mut GLchar,
            );

            if length > 0 {
                name.as_mut_vec().set_len(length as usize);
                name.truncate(length as usize);

                let type_ = DataType::from(attrib_type);

                Ok(Active {
                    name,
                    size,
                    type_,
                    length,
                })
            } else {
                // TODO: error desc
                Err(Error{})
            }
        }
    }

    pub fn gl_get_active_uniform(&mut self, program: u32, index: u32) -> Result<Active, Error> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut size: GLint = 0;
            let mut uniform_data_type: GLenum = 0;

            let mut name = String::with_capacity(256);

            ffi::glGetActiveUniform(
                program as GLuint,
                index as GLuint,
                256,
                &mut length,
                &mut size,
                &mut uniform_data_type,
                name.as_mut_vec().as_mut_ptr() as *mut GLchar,
            );

            if length > 0 {
                name.as_mut_vec().set_len(length as usize);
                name.truncate(length as usize);

                let type_ = DataType::from(uniform_data_type);

                Ok(Active {
                    name,
                    size,
                    type_,
                    length,
                })
            } else {
                // TODO: error desc
                Err(Error{})
            }
        }
    }

    pub fn gl_get_attached_shaders(&mut self, program: u32, max_count: i32) -> Result<Vec<u32>, Error> {
        unsafe {
            let mut count: GLsizei = 0;
            let mut vec: Vec<u32> = Vec::with_capacity(max_count as usize);

            ffi::glGetAttachedShaders(program as GLuint,
                                      max_count as GLsizei, &mut count,
                                      vec.as_mut_ptr());

            vec.set_len(count as usize);
            vec.truncate(count as usize);
            Ok(vec)
        }
    }

    pub fn gl_get_attrib_location(&mut self, program: u32, name: &str) -> Result<i32, Error> {
        unsafe {
            let c_str = CString::new(name).unwrap();

            let loc = ffi::glGetAttribLocation(program as GLuint, c_str.as_ptr() as *const GLchar);

            Ok(loc as i32)
        }
    }

    pub fn gl_get_booleanv(&mut self, name: StateType) -> Result<bool, Error> {
        let mut value: GLboolean = 0;

        unsafe {
            ffi::glGetBooleanv(name as GLenum, &mut value);
        }

        Ok(value == GL_TRUE)
    }

    pub fn gl_get_buffer_parameteriv(&mut self, target: BufferTarget, name: BufferParamName) -> Result<i32, Error> {
        let mut value: GLint = 0;

        unsafe {
            ffi::glGetBufferParameteriv(target as GLenum, name as GLenum,
                                        &mut value);
        }

        Ok(value as i32)
    }


    pub fn gl_get_error(&mut self) -> ErrorType {
        let mut error = GL_NO_ERROR;

        unsafe {
            error = ffi::glGetError();
        }

        ErrorType::from(error)
    }

    pub fn gl_get_floatv(&mut self, name: StateType) -> Result<f32, Error> {
        let mut value: GLfloat = 0.0;

        unsafe {
            ffi::glGetFloatv(name as GLenum, &mut value);
        }

        Ok(value as f32)
    }

    pub fn gl_get_framebuffer_attachment_parameteriv(
        &mut self,
        target: FrameBufferTarget,
        attachment: FrameBufferAttachmentType,
        name: FrameBufferAttachmentParamType,
    ) -> Result<i32, Error> {
        let mut value: GLint = 0;

        unsafe {
            ffi::glGetFramebufferAttachmentParameteriv(target as GLenum,
                                                       attachment as GLenum,
                                                       name as GLenum,
                                                       &mut value);
        }

        Ok(value as i32)
    }

    pub fn gl_get_integerv(&mut self, name: StateType) -> Result<i32, Error> {
        let mut value: GLint = 0;

        unsafe {
            ffi::glGetIntegerv(name as GLenum, &mut value);
        }

        Ok(value as i32)
    }

    pub fn gl_get_programiv(&mut self, program: u32, name: ProgramParamType) -> Result<i32, Error> {
        let mut value: GLint = 0;

        unsafe {
            ffi::glGetProgramiv(program as GLuint, name as GLenum, &mut value);
        }

        Ok(value as i32)
    }

    pub fn gl_get_program_info_log(&mut self, program: u32, max_length: i32) -> Result<String, Error> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut log = String::with_capacity(max_length as usize);

            #[cfg(target_os = "ios")]
                ffi::glGetProgramInfoLog(
                program as GLuint,
                max_length as GLsizei,
                &mut length,
                log.as_mut_vec().as_mut_ptr() as *mut i8,
            );

            #[cfg(target_os = "android")]
                ffi::glGetProgramInfoLog(
                program as GLuint,
                max_length as GLsizei,
                &mut length,
                log.as_mut_vec().as_mut_ptr() as *mut GLchar,
            );

            if length > 0 {
                log.as_mut_vec().set_len(length as usize);
                log.truncate(length as usize);

                Ok(log)
            } else {
                Ok("".to_string())
            }
        }
    }

    pub fn gl_get_renderbuffer_parameteriv(&mut self, target: RenderBufferTarget,
                                           name: RenderBufferParamType) -> Result<i32, Error> {
        let mut value: GLint = 0;

        unsafe {
            ffi::glGetRenderbufferParameteriv(target as GLenum,
                                              name as GLenum, &mut value);
        }

        Ok(value as i32)
    }

    pub fn gl_get_shaderiv(&mut self, shader: u32, name: ShaderParamType) -> Result<i32, Error> {
        let mut value: GLint = 0;

        unsafe {
            ffi::glGetShaderiv(shader as GLuint, name as GLenum, &mut value);
        }

        Ok(value as i32)
    }

    #[warn(unused_variables)]
    pub fn gl_get_shader_info_log(&mut self, shader: u32, max_length: i32) -> Result<String, Error> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut log = String::with_capacity(max_length as usize);

            #[cfg(target_os = "ios")]
                ffi::glGetShaderInfoLog(
                shader as GLuint,
                max_length as GLsizei,
                &mut length,
                log.as_mut_vec().as_mut_ptr() as *mut i8,
            );
            #[cfg(target_os = "android")]
                ffi::glGetShaderInfoLog(
                shader as GLuint,
                max_length as GLsizei,
                &mut length,
                log.as_mut_vec().as_mut_ptr() as *mut GLchar,
            );

            if length > 0 {
                log.as_mut_vec().set_len(length as usize);
                log.truncate(length as usize);

                Ok(log)
            } else {
                Ok("".to_string())
            }
        }
    }

    pub fn gl_get_shader_precision_format(
        &mut self,
        shader_type: ShaderType,
        precision_type: ShaderPrecisionType,
    ) -> Result<ShaderPrecisionFormat, Error> {
        let mut precision: GLint = 0;
        let mut range: [GLint; 2] = [0, 0];

        unsafe {
            ffi::glGetShaderPrecisionFormat(
                shader_type as GLenum,
                precision_type as GLenum,
                range.as_mut_ptr(),
                &mut precision,
            );
        }

        Ok(ShaderPrecisionFormat {
            precision: precision,
            range: range,
        })
    }

    pub fn gl_get_shader_source(&mut self, shader: u32, max_length: i32) -> Result<String, Error> {
        unsafe {
            let mut length: GLsizei = 0;
            let mut source = String::with_capacity(max_length as usize);

            ffi::glGetShaderSource(
                shader as GLuint,
                max_length as GLsizei,
                &mut length,
                source.as_mut_vec().as_mut_ptr() as *mut GLchar,
            );

            if length > 0 {
                source.as_mut_vec().set_len(length as usize);
                source.truncate(length as usize);

                Ok(source)
            } else {
                Ok("".to_string())
            }
        }
    }

    pub fn gl_get_string(&mut self, name: ConstantType) -> Result<String, Error> {
        unsafe {
            let c_str = ffi::glGetString(name as GLenum);
            //todo : can't guarantee the lifetime, because the memory is allocated by C
            if !c_str.is_null() {
                match from_utf8(CStr::from_ptr(c_str as *const c_char).to_bytes()) {
                    Ok(s) => Ok(s.to_string()),
                    // TODO: error desc
                    Err(_) => Err(Error{}),
                }
            } else {
                // TODO: Ok is not proper ?
                Ok("".to_string())
            }
        }
    }

    pub fn gl_get_tex_parameterfv(&mut self, target: TextureTarget, name: TextureParamType) -> Result<f32, Error> {
        let mut value: GLfloat = 0.0;

        unsafe {
            ffi::glGetTexParameterfv(target as GLenum, name as GLenum, &mut value);
        }

        Ok(value as f32)
    }

    pub fn gl_get_tex_parameteriv(&mut self, target: TextureTarget, name: TextureParamType) -> Result<i32, Error> {
        let mut value: GLint = 0;

        unsafe {
            ffi::glGetTexParameteriv(target as GLenum, name as GLenum, &mut value);
        }

        Ok(value as i32)
    }

    pub fn gl_get_uniformfv(&mut self, program: u32, location: i32) -> Result<f32, Error> {
        let mut value: GLfloat = 0.0;
        unsafe {
            ffi::glGetUniformfv(program as GLuint, location as GLint, &mut value);
        }

        Ok(value as f32)
    }

    pub fn gl_get_uniformiv(&mut self, program: u32, location: i32) -> Result<i32, Error> {
        let mut value: GLint = 0;

        unsafe {
            ffi::glGetUniformiv(program as GLuint, location as GLint, &mut value);
        }

        Ok(value as i32)
    }

    pub fn gl_get_uniform_location(&mut self, program: u32, name: &str) -> Result<i32, Error> {
        let mut loc: GLint = 0;

        unsafe {
            let name_c_str = CString::new(name).unwrap();

            loc = ffi::glGetUniformLocation(program as GLuint, name_c_str.as_ptr() as *const GLchar);
        }

        Ok(loc as i32)
    }

    pub fn gl_get_vertex_attribfv(&mut self, index: u32, name: VertexAttributeParamType) -> Result<f32, Error> {
        let mut value: GLfloat = 0.0;

        unsafe {
            ffi::glGetVertexAttribfv(index as GLuint, name as GLenum, &mut value);
        }

        Ok(value as f32)
    }

    pub fn gl_get_vertex_attribiv(&mut self, index: u32, name: VertexAttributeParamType) -> Result<i32, Error> {
        let mut value: GLint = 0;

        unsafe {
            ffi::glGetVertexAttribiv(index as GLuint, name as GLenum, &mut value);
        }

        Ok(value as i32)
    }

    pub fn gl_hint(&mut self, target: HintTargetType, mode: HintBehaviorType) -> Result<(), Error> {
        unsafe {
            ffi::glHint(target as GLenum, mode as GLenum)
        }

        Ok(())
    }

    pub fn gl_is_buffer(&mut self, buffer: u32) -> Result<bool, Error> {
        let mut res = false;

        unsafe {
            res = ffi::glIsBuffer(buffer as GLuint) == GL_TRUE;
        }

        Ok(res)
    }

    pub fn gl_is_enabled(&mut self, feature: FeatureType) -> Result<bool, Error> {
        let mut res = false;

        unsafe {
            res = ffi::glIsEnabled(feature as GLenum) == GL_TRUE;
        }

        Ok(res)
    }

    pub fn gl_is_framebuffer(&mut self, framebuffer: u32) -> Result<bool, Error> {
        let mut res = false;

        unsafe {
            res = ffi::glIsFramebuffer(framebuffer as GLuint) == GL_TRUE;
        }

        Ok(res)
    }

    pub fn gl_is_program(&mut self, program: u32) -> Result<bool, Error> {
        let mut res = false;

        unsafe {
            res = ffi::glIsProgram(program as GLuint) == GL_TRUE;
        }

        Ok(res)
    }

    pub fn gl_is_renderbuffer(&mut self, renderbuffer: u32) -> Result<bool, Error> {
        let mut res = false;

        unsafe {
            res = ffi::glIsRenderbuffer(renderbuffer as u32) == GL_TRUE;
        }

        Ok(res)
    }

    pub fn gl_is_shader(&mut self, shader: u32) -> Result<bool, Error> {
        let mut res = false;

        unsafe {
            res = ffi::glIsShader(shader as u32) == GL_TRUE;
        }

        Ok(res)
    }

    pub fn gl_is_texture(&mut self, texture: u32) -> Result<bool, Error> {
        let mut res = false;

        unsafe {
            res = ffi::glIsTexture(texture as u32) == GL_TRUE;
        }

        Ok(res)
    }

    pub fn gl_line_width(&mut self, width: f32) -> Result<(), Error>  {
        unsafe {
            ffi::glLineWidth(width as GLfloat);
        }

        Ok(())
    }

    pub fn gl_link_program(&mut self, program: u32) -> Result<(), Error> {
        unsafe {
            ffi::glLinkProgram(program as GLuint)
        }

        Ok(())
    }

    pub fn gl_pixel_storei(&mut self, name: PackParamType, param: i32) -> Result<(), Error> {
        unsafe {
            ffi::glPixelStorei(name as GLenum, param as GLint)
        }

        Ok(())
    }

    pub fn gl_polygon_offset(&mut self, factor: f32, units: f32) -> Result<(), Error> {
        unsafe {
            ffi::glPolygonOffset(factor as GLfloat, units as GLfloat)
        }

        Ok(())
    }

    // TODO: buffer size calculate automatically?
    pub fn gl_read_pixels<T>(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: PixelFormat,
        type_: PixelDataType,
        buffer: &mut [T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glReadPixels(
                x as GLint,
                y as GLint,
                width as GLsizei,
                height as GLsizei,
                format as GLenum,
                type_ as GLenum,
                buffer.as_mut_ptr() as *mut GLvoid,
            )
        }

        Ok(())
    }

    pub fn gl_release_shader_compiler(&mut self) -> Result<(), Error> {
        unsafe {
            ffi::glReleaseShaderCompiler()
        }

        Ok(())
    }

    pub fn gl_renderbuffer_storage(
        &mut self,
        target: RenderBufferTarget,
        internal_format: PixelFormat,
        width: i32,
        height: i32,
    ) -> Result<(), Error>  {
        unsafe {
            ffi::glRenderbufferStorage(target as GLenum, internal_format as GLenum,
                                       width as GLsizei, height as GLsizei)
        }

        Ok(())
    }

    pub fn gl_sample_coverage(
        &mut self,
        value: f32,
        invert: bool) -> Result<(), Error> {
        unsafe {
            ffi::glSampleCoverage(value as GLclampf, invert as GLboolean)
        }

        Ok(())
    }

    pub fn gl_scissor(&mut self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error> {
        unsafe {
            ffi::glScissor(x as GLint, y as GLint,
                           width as GLsizei, height as GLsizei)
        }

        Ok(())
    }

    // TODO: data_format
    // TODO: length's unit should be byte or T ?
    pub fn gl_shader_binary<T>(&mut self, shaders: &[u32], data_format: GLenum,
                               data: &[T], length: i32) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glShaderBinary(
                shaders.len() as GLsizei,
                shaders.as_ptr(),
                data_format,
                data.as_ptr() as *const GLvoid,
                length as GLsizei,
            )
        }

        Ok(())
    }

    pub fn gl_shader_source(&mut self, shader: u32, source: &str) -> Result<(), Error> {
        unsafe {
            let length: GLsizei = source.len() as GLsizei;

            ffi::glShaderSource(shader as GLuint, 1,
                                &(source.as_ptr() as *const GLchar), &length)
        }

        Ok(())
    }

    pub fn gl_stencil_func(&mut self, func: FuncType, ref_: i32, mask: u32) -> Result<(), Error> {
        unsafe {
            ffi::glStencilFunc(func as GLenum, ref_ as GLint, mask as GLuint)
        }

        Ok(())
    }

    pub fn gl_stencil_func_separate(&mut self, face: FaceMode, func: FuncType,
                                    ref_: i32, mask: u32) -> Result<(), Error> {
        unsafe {
            ffi::glStencilFuncSeparate(face as GLenum, func as GLenum,
                                       ref_ as GLint, mask as GLuint)
        }

        Ok(())
    }

    pub fn gl_stencil_mask(&mut self, mask: u32) -> Result<(), Error> {
        unsafe {
            ffi::glStencilMask(mask as GLuint)
        }

        Ok(())
    }

    pub fn gl_stencil_mask_separate(&mut self, face: FaceMode, mask: u32) -> Result<(), Error> {
        unsafe {
            ffi::glStencilMaskSeparate(face as GLenum, mask as GLuint)
        }

        Ok(())
    }

    pub fn gl_stencil_op(&mut self, s_fail: ActionType, dp_fail: ActionType, dp_pass: ActionType) -> Result<(), Error> {
        unsafe {
            ffi::glStencilOp(s_fail as GLenum, dp_fail as GLenum, dp_pass as GLenum)
        }

        Ok(())
    }

    pub fn gl_stencil_op_separate(&mut self, face: FaceMode, s_fail: ActionType,
                                  dp_fail: ActionType, dp_pass: ActionType) -> Result<(), Error> {
        unsafe {
            ffi::glStencilOpSeparate(face as GLenum, s_fail as GLenum, dp_fail as GLenum,
                                     dp_pass as GLenum)
        }

        Ok(())
    }

    // TODO: internal_format should be enum, but why GLint?
    pub fn gl_tex_image_2d<T>(
        &mut self,
        target: TextureTarget,
        level: i32,
        internal_format: GLint,
        width: i32,
        height: i32,
        border: i32,
        format: PixelFormat,
        type_: PixelDataType,
        buffer: &[T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glTexImage2D(
                target as GLenum,
                level as GLint,
                internal_format,
                width as GLsizei,
                height as GLsizei,
                border as GLint,
                format as GLenum,
                type_ as GLenum,
                buffer.as_ptr() as *const GLvoid,
            )
        }

        Ok(())
    }

    pub fn gl_tex_parameterf(&mut self, target: TextureBindTarget, name: TextureParamType,
                             value: f32) -> Result<(), Error> {
        unsafe {
            ffi::glTexParameterf(target as GLenum, name as GLenum, value as GLfloat)
        }

        Ok(())
    }

    pub fn gl_tex_parameterfv(&mut self, target: TextureBindTarget,
                              name: TextureParamType) -> Result<f32, Error> {

        let res: GLfloat = 0.0;
        unsafe {
            ffi::glTexParameterfv(target as GLenum, name as GLenum, &res)
        }

        Ok(res as f32)
    }

    pub fn gl_tex_parameteri(&mut self, target: TextureBindTarget, name: TextureParamType,
                             value: GLint) -> Result<(), Error> {
        unsafe {
            ffi::glTexParameteri(target as GLenum, name as GLenum, value)
        }

        Ok(())
    }

    pub fn gl_tex_parameteriv(&mut self, target: TextureBindTarget, name: TextureParamType) -> Result<i32, Error> {
        let res: GLint = 0;

        unsafe {
            ffi::glTexParameteriv(target as GLenum, name as GLenum, &res)
        }

        Ok(res as i32)
    }

    pub fn gl_tex_sub_image_2d<T>(
        &mut self,
        target: TextureTarget,
        level: i32,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        format: PixelFormat,
        type_: PixelDataType,
        buffer: &[T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            ffi::glTexSubImage2D(
                target as GLenum,
                level as GLint,
                x_offset as GLint,
                y_offset as GLint,
                width as GLsizei,
                height as GLsizei,
                format as GLenum,
                type_ as GLenum,
                buffer.as_ptr() as *const GLvoid,
            )
        }

        Ok(())
    }

    pub fn gl_uniform1f(&mut self, location: i32, x: f32) -> Result<(), Error> {
        unsafe {
            ffi::glUniform1f(location as GLint, x as GLfloat)
        }

        Ok(())
    }

    pub fn gl_uniform1fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform1fv(location as GLint, values.len() as GLsizei, values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_uniform1i(&mut self, location: i32, x: i32) -> Result<(), Error> {
        unsafe {
            ffi::glUniform1i(location as GLint, x as GLint)
        }

        Ok(())
    }

    pub fn gl_uniform1iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform1iv(location as GLint, values.len() as GLsizei, values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_uniform2f(&mut self, location: i32, x: f32, y: f32) -> Result<(), Error> {
        unsafe {
            ffi::glUniform2f(location as GLint, x as GLfloat, y as GLfloat)
        }

        Ok(())
    }

    pub fn gl_uniform2fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform2fv(location as GLint, (values.len() / 2) as GLsizei,
                              values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_uniform2i(&mut self, location: i32, x: i32, y: i32) -> Result<(), Error> {
        unsafe {
            ffi::glUniform2i(location as GLint, x as GLint, y as GLint)
        }

        Ok(())
    }

    pub fn gl_uniform2iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform2iv(location as GLint, (values.len() / 2) as GLsizei, values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_uniform3f(&mut self, location: i32, x: f32, y: f32, z: f32) -> Result<(), Error> {
        unsafe {
            ffi::glUniform3f(location as GLint, x as GLfloat, y as GLfloat, z as GLfloat)
        }

        Ok(())
    }

    pub fn gl_uniform3fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform3fv(location as GLint, (values.len() / 3) as GLsizei, values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_uniform3i(&mut self, location: i32, x: i32, y: i32, z: i32) -> Result<(), Error> {
        unsafe {
            ffi::glUniform3i(location as GLint, x as GLint, y as GLint, z as GLint)
        }

        Ok(())
    }

    pub fn gl_uniform3iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform3iv(location as GLint, (values.len() / 3) as GLsizei, values.as_ptr())
        }

        Ok(())
    }


    pub fn gl_uniform4f(&mut self, location: i32, x: f32, y: f32, z: f32,
                        w: f32) -> Result<(), Error> {
        unsafe {
            ffi::glUniform4f(location as GLint, x as GLfloat,
                             y as GLfloat, z as GLfloat, w as GLfloat)
        }

        Ok(())
    }

    pub fn gl_uniform4fv(&mut self, location: i32, values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform4fv(location as GLint, (values.len() / 4) as GLsizei, values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_uniform4i(&mut self, location: i32, x: i32, y: i32, z: i32, w: i32) -> Result<(), Error> {
        unsafe {
            ffi::glUniform4i(location as GLint, x as GLint, y as GLint, z as GLint, w as GLint)
        }

        Ok(())
    }

    pub fn gl_uniform4iv(&mut self, location: i32, values: &[i32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniform4iv(location as GLint, (values.len() / 4) as GLsizei, values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_uniform_matrix2fv(&mut self, location: i32, transpose: bool, values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix2fv(
                location as i32,
                (values.len() / 2*2) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }

        Ok(())
    }

    pub fn gl_uniform_matrix3fv(&mut self, location: i32, transpose: bool, values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix3fv(
                location as GLint,
                (values.len() / 3*3) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }

        Ok(())
    }

    pub fn gl_uniform_matrix4fv(&mut self, location: i32, transpose: bool,
                                values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glUniformMatrix4fv(
                location as GLint,
                (values.len() / 4*4) as GLsizei,
                transpose as GLboolean,
                values.as_ptr() as *const GLfloat,
            )
        }

        Ok(())
    }

    pub fn gl_use_program(&mut self, program: u32) -> Result<(), Error> {
        unsafe {
            ffi::glUseProgram(program as GLuint)
        }

        Ok(())
    }

    pub fn gl_validate_program(&mut self, program: u32) -> Result<(), Error> {
        unsafe {
            ffi::glValidateProgram(program as GLuint)
        }

        Ok(())
    }

    pub fn gl_vertex_attrib1f(&mut self, index: u32, x: f32) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttrib1f(index as GLuint, x as GLfloat)
        }

        Ok(())
    }

    pub fn gl_vertex_attrib1fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttrib1fv(index as GLuint, values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_vertex_attrib2f(&mut self, index: u32, x: f32, y: f32) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttrib2f(index as GLuint, x as GLfloat, y as GLfloat)
        }

        Ok(())
    }

    pub fn gl_vertex_attrib2fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttrib2fv(index as GLuint, values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_vertex_attrib3f(&mut self, index: u32, x: f32, y: f32, z: f32) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttrib3f(index as GLuint, x as GLfloat, y as GLfloat, z as GLfloat)
        }

        Ok(())
    }

    pub fn gl_vertex_attrib3fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttrib3fv(index as GLuint, values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_vertex_attrib4f(&mut self, index: u32, x: f32, y: f32, z: f32, w: f32) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttrib4f(index as GLuint, x as GLfloat, y as GLfloat, z as GLfloat,
                                  w as GLfloat)
        }

        Ok(())
    }

    pub fn gl_vertex_attrib4fv(&mut self, index: u32, values: &[f32]) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttrib4fv(index as GLuint, values.as_ptr())
        }

        Ok(())
    }

    pub fn gl_vertex_attrib_pointer<T>(
        &mut self,
        index: u32,
        size: i32,
        type_: DataType,
        normalized: bool,
        stride: i32,
        buffer: &[T],
    ) -> Result<(), Error> where T: std::fmt::Debug + Clone {
        unsafe {
            if buffer.len() == 0 {
                ffi::glVertexAttribPointer(
                    index as GLuint,
                    size as GLint,
                    type_ as GLenum,
                    normalized as GLboolean,
                    stride as GLsizei,
                    &0 as *const i32 as *const GLvoid,
                )
            } else {
                ffi::glVertexAttribPointer(
                    index as GLuint,
                    size as GLint,
                    type_ as GLenum,
                    normalized as GLboolean,
                    stride as GLsizei,
                    buffer.as_ptr() as *const GLvoid,
                )
            }
        }

        Ok(())
    }

    pub fn gl_vertex_attrib_pointer_offset(
        &mut self,
        index: u32,
        size: i32,
        type_: DataType,
        normalized: bool,
        stride: i32,
        offset: u32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::glVertexAttribPointer(
                index as GLuint,
                size as GLint,
                type_ as GLenum,
                normalized as GLboolean,
                stride as i32,
                offset as *const GLvoid)
        }

        Ok(())
    }

    pub fn gl_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) -> Result<(), Error> {
        unsafe {
            ffi::glViewport(x as GLint, y as GLint, width as GLsizei, height as GLsizei)
        }

        Ok(())
    }
}