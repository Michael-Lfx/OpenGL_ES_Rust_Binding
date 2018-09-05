use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void, c_float};

// -------------------------------------------------------------------------------------------------
// TYPES
// -------------------------------------------------------------------------------------------------
pub type GLbitfield = c_uint;
pub type GLboolean = c_uchar;
pub type GLbyte = c_char;
pub type GLchar = c_char;
pub type GLclampf = c_float;
pub type GLenum = c_uint;
pub type GLfloat = c_float;
pub type GLint = c_int;
pub type GLshort = c_short;
pub type GLsizei = c_int;
pub type GLubyte = c_uchar;
pub type GLuint = c_uint;
pub type GLushort = c_ushort;
pub type GLvoid = c_void;
pub type GLcharARB = c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = c_uint;

pub type GLhalfARB = c_ushort;
pub type GLhalf = c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

pub type GLDEBUGPROC = extern "system" fn(
    source: GLenum,
    gl_type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    user_param: *mut c_void,
);
pub type GLDEBUGPROCARB = extern "system" fn(
    source: GLenum,
    gl_type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    user_param: *mut c_void,
);
pub type GLDEBUGPROCKHR = extern "system" fn(
    source: GLenum,
    gl_type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    user_param: *mut c_void,
);

// Vendor extension types
pub type GLDEBUGPROCAMD = extern "system" fn(
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    user_param: *mut c_void,
);
pub type GLhalfNV = c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;