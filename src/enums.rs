use types::*;
use consts::*;

pub struct Error {

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureUnit {
    Texture0 = GL_TEXTURE0 as isize,
    Texture1 = GL_TEXTURE1 as isize,
    Texture2 = GL_TEXTURE2 as isize,
    Texture3 = GL_TEXTURE3 as isize,
    Texture4 = GL_TEXTURE4 as isize,
    Texture5 = GL_TEXTURE5 as isize,
    Texture6 = GL_TEXTURE6 as isize,
    Texture7 = GL_TEXTURE7 as isize,
    Texture8 = GL_TEXTURE8 as isize,
    Texture9 = GL_TEXTURE9 as isize,
    Texture10 = GL_TEXTURE10 as isize,
    Texture11 = GL_TEXTURE11 as isize,
    Texture12 = GL_TEXTURE12 as isize,
    Texture13 = GL_TEXTURE13 as isize,
    Texture14 = GL_TEXTURE14 as isize,
    Texture15 = GL_TEXTURE15 as isize,
    Texture16 = GL_TEXTURE16 as isize,
    Texture17 = GL_TEXTURE17 as isize,
    Texture18 = GL_TEXTURE18 as isize,
    Texture19 = GL_TEXTURE19 as isize,
    Texture20 = GL_TEXTURE20 as isize,
    Texture21 = GL_TEXTURE21 as isize,
    Texture22 = GL_TEXTURE22 as isize,
    Texture23 = GL_TEXTURE23 as isize,
    Texture24 = GL_TEXTURE24 as isize,
    Texture25 = GL_TEXTURE25 as isize,
    Texture26 = GL_TEXTURE26 as isize,
    Texture27 = GL_TEXTURE27 as isize,
    Texture28 = GL_TEXTURE28 as isize,
    Texture29 = GL_TEXTURE29 as isize,
    Texture30 = GL_TEXTURE30 as isize,
    Texture31 = GL_TEXTURE31 as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferTarget {
    ARRAY_BUFFER = GL_ARRAY_BUFFER_BINDING as isize,
    ELEMENT_ARRAY_BUFFER = GL_ELEMENT_ARRAY_BUFFER_BINDING as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrameBufferTarget {
    FrameBuffer = GL_FRAMEBUFFER as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RenderBufferTarget {
    RENDERBUFFER = GL_RENDERBUFFER as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureBindTarget {
    TEXTURE_2D = GL_TEXTURE_BINDING_2D as isize,
    TEXTURE_CUBE_MAP = GL_TEXTURE_BINDING_CUBE_MAP as isize

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureTarget {
    TEXTURE_2D = GL_TEXTURE_2D as isize,
    TEXTURE_CUBE_MAP_POSITIVE_X = GL_TEXTURE_CUBE_MAP_POSITIVE_X as isize,
    TEXTURE_CUBE_MAP_NEGATIVE_X = GL_TEXTURE_CUBE_MAP_NEGATIVE_X as isize,
    TEXTURE_CUBE_MAP_POSITIVE_Y = GL_TEXTURE_CUBE_MAP_POSITIVE_Y as isize,
    TEXTURE_CUBE_MAP_NEGATIVE_Y = GL_TEXTURE_CUBE_MAP_NEGATIVE_Y as isize,
    TEXTURE_CUBE_MAP_POSITIVE_Z = GL_TEXTURE_CUBE_MAP_POSITIVE_Z as isize,
    TEXTURE_CUBE_MAP_NEGATIVE_Z = GL_TEXTURE_CUBE_MAP_NEGATIVE_Z as isize,
    Texture3D = GL_TEXTURE_3D as isize,
    Texture2DArray = GL_TEXTURE_2D_ARRAY as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlendEquationMode {
    FUNC_ADD = GL_FUNC_ADD as isize,
    FUNC_SUBTRACT = GL_FUNC_SUBTRACT as isize,
    FUNC_REVERSE_SUBTRACT = GL_FUNC_REVERSE_SUBTRACT as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlendFactor {
    ZERO = GL_ZERO as isize,
    ONE = GL_ONE as isize,
    SRC_COLOR = GL_SRC_COLOR as isize,
    ONE_MINUS_SRC_COLOR = GL_ONE_MINUS_SRC_COLOR as isize,
    DST_COLOR = GL_DST_COLOR as isize,
    ONE_MINUS_DST_COLOR = GL_ONE_MINUS_DST_COLOR as isize,
    SRC_ALPHA = GL_SRC_ALPHA as isize,
    ONE_MINUS_SRC_ALPHA = GL_ONE_MINUS_SRC_ALPHA as isize,
    DST_ALPHA = GL_DST_ALPHA as isize,
    ONE_MINUS_DST_ALPHA = GL_ONE_MINUS_DST_ALPHA as isize,
    CONSTANT_COLOR = GL_CONSTANT_COLOR as isize,
    ONE_MINUS_CONSTANT_COLOR = GL_ONE_MINUS_CONSTANT_COLOR as isize,
    CONSTANT_ALPHA = GL_CONSTANT_ALPHA as isize,
    ONE_MINUS_CONSTANT_ALPHA = GL_ONE_MINUS_CONSTANT_ALPHA as isize,
    SRC_ALPHA_SATURATE = GL_SRC_ALPHA_SATURATE as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferUsage {
    STREAM_DRAW = GL_STREAM_DRAW as isize,
    STATIC_DRAW = GL_STATIC_DRAW as isize,
    DYNAMIC_DRAW = GL_DYNAMIC_DRAW as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrameBufferStatus {
    FRAMEBUFFER_COMPLETE = GL_FRAMEBUFFER_COMPLETE as isize,
    FRAMEBUFFER_INCOMPLETE_ATTACHMENT = GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT as isize,
    FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT = GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT as isize,
    FRAMEBUFFER_INCOMPLETE_DIMENSIONS = GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS as isize,
    FRAMEBUFFER_UNSUPPORTED = GL_FRAMEBUFFER_UNSUPPORTED as isize
}

impl From<GLuint> for FrameBufferStatus {
    fn from(id: GLuint) -> Self {
        match id {
            GL_FRAMEBUFFER_COMPLETE => FrameBufferStatus::FRAMEBUFFER_COMPLETE,
            GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT => FrameBufferStatus::FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
            GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => FrameBufferStatus::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
            GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS => FrameBufferStatus::FRAMEBUFFER_INCOMPLETE_DIMENSIONS,
            _ => FrameBufferStatus::FRAMEBUFFER_UNSUPPORTED
        }
    }
}

/// Buffer Objects
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorBufferMode {
    Back = GL_BACK as isize,
    None = GL_NONE as isize,
    ColorAttachment0 = GL_COLOR_ATTACHMENT0 as isize,
    ColorAttachment1 = GL_COLOR_ATTACHMENT1 as isize,
    ColorAttachment2 = GL_COLOR_ATTACHMENT2 as isize,
    ColorAttachment3 = GL_COLOR_ATTACHMENT3 as isize,
    ColorAttachment4 = GL_COLOR_ATTACHMENT4 as isize,
    ColorAttachment5 = GL_COLOR_ATTACHMENT5 as isize,
    ColorAttachment6 = GL_COLOR_ATTACHMENT6 as isize,
    ColorAttachment7 = GL_COLOR_ATTACHMENT7 as isize,
    ColorAttachment8 = GL_COLOR_ATTACHMENT8 as isize,
    ColorAttachment9 = GL_COLOR_ATTACHMENT9 as isize,
    ColorAttachment10 = GL_COLOR_ATTACHMENT10 as isize,
    ColorAttachment11 = GL_COLOR_ATTACHMENT11 as isize,
    ColorAttachment12 = GL_COLOR_ATTACHMENT12 as isize,
    ColorAttachment13 = GL_COLOR_ATTACHMENT13 as isize,
    ColorAttachment14 = GL_COLOR_ATTACHMENT14 as isize,
    ColorAttachment15 = GL_COLOR_ATTACHMENT15 as isize,
    MaxColorAttachments = GL_MAX_COLOR_ATTACHMENTS as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferObjectTarget {
    /// Vertex attributes
    ArrayBuffer = GL_ARRAY_BUFFER as isize,
    /// Atomic counter storage
    AtomicCounterBuffer = GL_ATOMIC_COUNTER_BUFFER as isize,
    /// Buffer copy source
    CopyReadBuffer = GL_COPY_READ_BUFFER  as isize,
    /// Buffer copy destination
    CopyWriteBuffer = GL_COPY_WRITE_BUFFER  as isize,
    /// Indirect compute dispatch commands
    DispatchIndirectBuffer = GL_DISPATCH_INDIRECT_BUFFER as isize,
    /// Indirect command arguments
    DrawIndirectBuffer = GL_DRAW_INDIRECT_BUFFER as isize,
    /// Vertex array indices
    ElementArrayBuffer = GL_ELEMENT_ARRAY_BUFFER  as isize,
    /// Pixel read target
    PixelPackBuffer = GL_PIXEL_PACK_BUFFER  as isize,
    /// Texture data source
    PixelUnpackBuffer = GL_PIXEL_UNPACK_BUFFER  as isize,
    /// Query result buffer
//    QueryBuffer = GL_QUERY_BUFFER as isize,
    /// Read-write storage for shaders
    ShaderStorageBuffer = GL_SHADER_STORAGE_BUFFER as isize,
    /// Texture data buffer
    TextureBuffer = GL_TEXTURE_BUFFER as isize,
    /// Transform feedback buffer
    TransformFeedbackBuffer = GL_TRANSFORM_FEEDBACK_BUFFER  as isize,
    /// Uniform block storage
    UniformBuffer = GL_UNIFORM_BUFFER  as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferMapTarget {
    BufferMapPointer = GL_BUFFER_MAP_POINTER as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShaderType {
    FRAGMENT_SHADER = GL_FRAGMENT_SHADER as isize,
    VERTEX_SHADER = GL_VERTEX_SHADER as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FaceMode {
    FRONT = GL_FRONT as isize,
    BACK = GL_BACK as isize,
    FRONT_AND_BACK= GL_FRONT_AND_BACK as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FuncType {
    NEVER = GL_NEVER as isize,
    LESS = GL_LESS as isize,
    EQUAL = GL_EQUAL as isize,
    LEQUAL = GL_LEQUAL as isize,
    GREATER = GL_GREATER as isize,
    NOTEQUAL = GL_NOTEQUAL as isize,
    GEQUAL = GL_GEQUAL as isize,
    ALWAYS = GL_ALWAYS as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FeatureType {
    BLEND = GL_BLEND as isize,
    CULL_FACE = GL_CULL_FACE as isize,
    DEPTH_TEST = GL_DEPTH_TEST as isize,
    DITHER = GL_DITHER as isize,
    POLYGON_OFFSET_FILL = GL_POLYGON_OFFSET_FILL as isize,
    SAMPLE_ALPHA_TO_COVERAGE = GL_SAMPLE_ALPHA_TO_COVERAGE as isize,
    SAMPLE_COVERAGE = GL_SAMPLE_COVERAGE as isize,
    SCISSOR_TEST = GL_SCISSOR_TEST as isize,
    STENCIL_TEST = GL_STENCIL_TEST as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BeginMode {
    POINTS = GL_POINTS as isize,
    LINES = GL_LINES as isize,
    LINE_LOOP = GL_LINE_LOOP as isize,
    LINE_STRIP = GL_LINE_STRIP as isize,
    TRIANGLES = GL_TRIANGLES as isize,
    TRIANGLE_STRIP = GL_TRIANGLE_STRIP as isize,
    TRIANGLE_FAN = GL_TRIANGLE_FAN as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MappingBit {
    InvalidateRange = GL_MAP_INVALIDATE_RANGE_BIT as isize,
    InvalidateBuffer = GL_MAP_INVALIDATE_BUFFER_BIT as isize,
    FlushExplicit = GL_MAP_FLUSH_EXPLICIT_BIT as isize,
    Unsynchronized = GL_MAP_UNSYNCHRONIZED_BIT as isize,
}

/// Samplers

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SamplerParameter {
    WrapS = GL_TEXTURE_WRAP_S as isize,
    WrapT = GL_TEXTURE_WRAP_T as isize,
    WrapR = GL_TEXTURE_WRAP_R as isize,
    MinFilter = GL_TEXTURE_MIN_FILTER as isize,
    MagFilter = GL_TEXTURE_MAG_FILTER as isize,
    BorderColor = GL_TEXTURE_BORDER_COLOR as isize,
    MinLod = GL_TEXTURE_MIN_LOD as isize,
    MaxLod = GL_TEXTURE_MAX_LOD as isize,
    //    LodBias = GL_TEXTURE_LOD_BIAS as isize,
    CompareMode = GL_TEXTURE_COMPARE_MODE as isize,
    CompareFunc = GL_TEXTURE_COMPARE_FUNC as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TransformFeedbackMode {
    InterleavedAttributes = GL_INTERLEAVED_ATTRIBS as isize,
    SeparatedAttributes = GL_SEPARATE_ATTRIBS as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TransformFeedbackObjectTarget {
    TransformFeedback = GL_TRANSFORM_FEEDBACK as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AttachmentTarget {
    Color_Attachment_0 = GL_COLOR_ATTACHMENT0 as isize,
    Color_Attachment_1 = GL_COLOR_ATTACHMENT1 as isize,
    Color_Attachment_2 = GL_COLOR_ATTACHMENT2 as isize,
    Color_Attachment_3 = GL_COLOR_ATTACHMENT3 as isize,
    Color_Attachment = GL_DEPTH_ATTACHMENT as isize,
    StencilAttachment = GL_STENCIL_ATTACHMENT as isize,
    DepthStencilAttachment = GL_DEPTH_STENCIL_ATTACHMENT as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FilterMode {
    Nearst = GL_NEAREST as isize,
    Linear = GL_LINEAR as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferMask {
    Color = GL_COLOR_BUFFER_BIT as isize,
    Depth = GL_DEPTH_BUFFER_BIT as isize,
    Stencil = GL_STENCIL_BUFFER_BIT as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FramebufferTarget {
    Draw = GL_DRAW_FRAMEBUFFER as isize,
    Read = GL_READ_FRAMEBUFFER as isize,
    /// GL_FRAMEBUFFER is equivalent to GL_DRAW_FRAMEBUFFER
    Framebuffer = GL_FRAMEBUFFER as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PixelDataFormat {
    Red = GL_RED as isize,
    RedInteger = GL_RED_INTEGER as isize,
    RedGreeen = GL_RG as isize,
    RedGreenInteger = GL_RG_INTEGER as isize,
    RedGreenBlue = GL_RGB as isize,
    RedGreenBlueInteget = GL_RGB_INTEGER as isize,
    RedGreenBlueAlpha = GL_RGBA as isize,
    RedGreenBlueAlphaInteger = GL_RGBA_INTEGER as isize,
    DepthComponent = GL_DEPTH_COMPONENT as isize,
    DepthStencil = GL_DEPTH_STENCIL as isize,
    LuminanceAlpha = GL_LUMINANCE_ALPHA as isize,
    Lumniance = GL_LUMINANCE as isize,
    Alpha = GL_ALPHA as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrameBufferAttachmentType {
    COLOR_ATTACHMENT0 = GL_COLOR_ATTACHMENT0 as isize,
    DEPTH_ATTACHMENT = GL_DEPTH_ATTACHMENT as isize,
    STENCIL_ATTACHMENT = GL_STENCIL_ATTACHMENT as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrontFaceDirection {
    CW = GL_CW as isize,
    CCW = GL_CCW as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StateType {
    ACTIVE_TEXTURE = GL_ACTIVE_TEXTURE as isize,
    ALIASED_LINE_WIDTH_RANGE = GL_ALIASED_LINE_WIDTH_RANGE as isize,
    ALIASED_POINT_SIZE_RANGE = GL_ALIASED_POINT_SIZE_RANGE as isize,
    ALPHA_BITS = GL_ALPHA_BITS as isize,
    ARRAY_BUFFER_BINDING = GL_ARRAY_BUFFER_BINDING as isize,
    BLEND = GL_BLEND as isize,
    BLEND_COLOR = GL_BLEND_COLOR as isize,
    BLEND_DST_ALPHA = GL_BLEND_DST_ALPHA as isize,
    BLEND_DST_RGB = GL_BLEND_DST_RGB as isize,
    BLEND_EQUATION_ALPHA = GL_BLEND_EQUATION_ALPHA as isize,
    BLEND_EQUATION_RGB = GL_BLEND_EQUATION_RGB as isize,
    BLEND_SRC_ALPHA = GL_BLEND_SRC_ALPHA as isize,
    BLEND_SRC_RGB = GL_BLEND_SRC_RGB as isize,
    BLUE_BITS = GL_BLUE_BITS as isize,
    COLOR_CLEAR_VALUE = GL_COLOR_CLEAR_VALUE as isize,
    COLOR_WRITEMASK = GL_COLOR_WRITEMASK as isize,
    COMPRESSED_TEXTURE_FORMATS = GL_COMPRESSED_TEXTURE_FORMATS as isize,
    CULL_FACE = GL_CULL_FACE as isize,
    CULL_FACE_MODE = GL_CULL_FACE_MODE as isize,
    CURRENT_PROGRAM = GL_CURRENT_PROGRAM as isize,
    DEPTH_BITS = GL_DEPTH_BITS as isize,
    DEPTH_CLEAR_VALUE = GL_DEPTH_CLEAR_VALUE as isize,
    DEPTH_FUNC = GL_DEPTH_FUNC as isize,
    DEPTH_RANGE = GL_DEPTH_RANGE as isize,
    DEPTH_TEST = GL_DEPTH_TEST as isize,
    DEPTH_WRITEMASK = GL_DEPTH_WRITEMASK as isize,
    DITHER = GL_DITHER as isize,
    ELEMENT_ARRAY_BUFFER_BINDING = GL_ELEMENT_ARRAY_BUFFER_BINDING as isize,
    FRAMEBUFFER_BINDING = GL_FRAMEBUFFER_BINDING as isize,
    FRONT_FACE = GL_FRONT_FACE as isize,
    GENERATE_MIPMAP_HINT = GL_GENERATE_MIPMAP_HINT as isize,
    GREEN_BITS = GL_GREEN_BITS as isize,
    IMPLEMENTATION_COLOR_READ_FORMAT = GL_IMPLEMENTATION_COLOR_READ_FORMAT as isize,
    IMPLEMENTATION_COLOR_READ_TYPE = GL_IMPLEMENTATION_COLOR_READ_TYPE as isize,
    LINE_WIDTH = GL_LINE_WIDTH as isize,
    MAX_COMBINED_TEXTURE_IMAGE_UNITS = GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS as isize,
    MAX_CUBE_MAP_TEXTURE_SIZE = GL_MAX_CUBE_MAP_TEXTURE_SIZE as isize,
    MAX_FRAGMENT_UNIFORM_VECTORS = GL_MAX_FRAGMENT_UNIFORM_VECTORS as isize,
    MAX_RENDERBUFFER_SIZE = GL_MAX_RENDERBUFFER_SIZE as isize,
    MAX_TEXTURE_IMAGE_UNITS = GL_MAX_TEXTURE_IMAGE_UNITS as isize,
    MAX_TEXTURE_SIZE = GL_MAX_TEXTURE_SIZE as isize,
    MAX_VARYING_VECTORS = GL_MAX_VARYING_VECTORS as isize,
    MAX_VERTEX_ATTRIBS = GL_MAX_VERTEX_ATTRIBS as isize,
    MAX_VERTEX_TEXTURE_IMAGE_UNITS = GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS as isize,
    MAX_VERTEX_UNIFORM_VECTORS = GL_MAX_VERTEX_UNIFORM_VECTORS as isize,
    MAX_VIEWPORT_DIMS = GL_MAX_VIEWPORT_DIMS as isize,
    NUM_COMPRESSED_TEXTURE_FORMATS = GL_NUM_COMPRESSED_TEXTURE_FORMATS as isize,
    NUM_SHADER_BINARY_FORMATS = GL_NUM_SHADER_BINARY_FORMATS as isize,
    PACK_ALIGNMENT = GL_PACK_ALIGNMENT as isize,
    POLYGON_OFFSET_FACTOR = GL_POLYGON_OFFSET_FACTOR as isize,
    POLYGON_OFFSET_FILL = GL_POLYGON_OFFSET_FILL as isize,
    POLYGON_OFFSET_UNITS = GL_POLYGON_OFFSET_UNITS as isize,
    RED_BITS = GL_RED_BITS as isize,
    RENDERBUFFER_BINDING = GL_RENDERBUFFER_BINDING as isize,
    SAMPLE_ALPHA_TO_COVERAGE = GL_SAMPLE_ALPHA_TO_COVERAGE as isize,
    SAMPLE_BUFFERS = GL_SAMPLE_BUFFERS as isize,
    SAMPLE_COVERAGE = GL_SAMPLE_COVERAGE as isize,
    SAMPLE_COVERAGE_INVERT = GL_SAMPLE_COVERAGE_INVERT as isize,
    SAMPLE_COVERAGE_VALUE = GL_SAMPLE_COVERAGE_VALUE as isize,
    SAMPLES = GL_SAMPLES as isize,
    SCISSOR_BOX = GL_SCISSOR_BOX as isize,
    SCISSOR_TEST = GL_SCISSOR_TEST as isize,
    SHADER_BINARY_FORMATS = GL_SHADER_BINARY_FORMATS as isize,
    SHADER_COMPILER = GL_SHADER_COMPILER as isize,
    STENCIL_BACK_FAIL = GL_STENCIL_BACK_FAIL as isize,
    STENCIL_BACK_FUNC = GL_STENCIL_BACK_FUNC as isize,
    STENCIL_BACK_PASS_DEPTH_FAIL = GL_STENCIL_BACK_PASS_DEPTH_FAIL as isize,
    STENCIL_BACK_PASS_DEPTH_PASS = GL_STENCIL_BACK_PASS_DEPTH_PASS as isize,
    STENCIL_BACK_REF = GL_STENCIL_BACK_REF as isize,
    STENCIL_BACK_VALUE_MASK = GL_STENCIL_BACK_VALUE_MASK as isize,
    STENCIL_BACK_WRITEMASK = GL_STENCIL_BACK_WRITEMASK as isize,
    STENCIL_BITS = GL_STENCIL_BITS as isize,
    STENCIL_CLEAR_VALUE = GL_STENCIL_CLEAR_VALUE as isize,
    STENCIL_FAIL = GL_STENCIL_FAIL as isize,
    STENCIL_FUNC = GL_STENCIL_FUNC as isize,
    STENCIL_PASS_DEPTH_FAIL  = GL_STENCIL_PASS_DEPTH_FAIL as isize,
    STENCIL_PASS_DEPTH_PASS = GL_STENCIL_PASS_DEPTH_PASS as isize,
    STENCIL_REF = GL_STENCIL_REF as isize,
    STENCIL_TEST = GL_STENCIL_TEST as isize,
    STENCIL_VALUE_MASK = GL_STENCIL_VALUE_MASK as isize,
    STENCIL_WRITEMASK = GL_STENCIL_WRITEMASK as isize,
    SUBPIXEL_BITS = GL_SUBPIXEL_BITS as isize,
    TEXTURE_BINDING_2D = GL_TEXTURE_BINDING_2D as isize,
    TEXTURE_BINDING_CUBE_MAP = GL_TEXTURE_BINDING_CUBE_MAP as isize,
    UNPACK_ALIGNMENT = GL_UNPACK_ALIGNMENT as isize,
    VIEWPORT = GL_VIEWPORT as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferParamName {
    BUFFER_SIZE = GL_BUFFER_SIZE as isize,
    BUFFER_USAGE = GL_BUFFER_USAGE as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorType {
    NO_ERROR = GL_NO_ERROR as isize,
    INVALID_ENUM = GL_INVALID_ENUM as isize,
    INVALID_VALUE = GL_INVALID_VALUE as isize,
    INVALID_OPERATION = GL_INVALID_OPERATION as isize,
    INVALID_FRAMEBUFFER_OPERATION = GL_INVALID_FRAMEBUFFER_OPERATION as isize,
    OUT_OF_MEMORY = GL_OUT_OF_MEMORY as isize,
}

impl From<GLenum> for ErrorType {
    fn from(error: GLenum) -> Self {
        match error {
            GL_NO_ERROR => ErrorType::NO_ERROR,
            GL_INVALID_ENUM => ErrorType::INVALID_ENUM,
            GL_INVALID_VALUE => ErrorType::INVALID_VALUE,
            GL_INVALID_OPERATION => ErrorType::INVALID_OPERATION,
            GL_INVALID_FRAMEBUFFER_OPERATION => ErrorType::INVALID_FRAMEBUFFER_OPERATION,
            GL_OUT_OF_MEMORY => ErrorType::OUT_OF_MEMORY,
            _ => ErrorType::NO_ERROR,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FrameBufferAttachmentParamType {
    FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE = GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE as isize,
    FRAMEBUFFER_ATTACHMENT_OBJECT_NAME = GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME as isize,
    FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL = GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL as isize,
    FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE = GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProgramParamType {
    DELETE_STATUS = GL_DELETE_STATUS as isize,
    LINK_STATUS = GL_LINK_STATUS as isize,
    VALIDATE_STATUS = GL_VALIDATE_STATUS as isize,
    INFO_LOG_LENGTH = GL_INFO_LOG_LENGTH as isize,
    ATTACHED_SHADERS = GL_ATTACHED_SHADERS as isize,
    ACTIVE_ATTRIBUTES = GL_ACTIVE_ATTRIBUTES as isize,
    ACTIVE_ATTRIBUTE_MAX_LENGTH = GL_ACTIVE_ATTRIBUTE_MAX_LENGTH as isize,
    ACTIVE_UNIFORMS = GL_ACTIVE_UNIFORMS as isize,
    ACTIVE_UNIFORM_MAX_LENGTH = GL_ACTIVE_UNIFORM_MAX_LENGTH as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RenderBufferParamType {
    RENDERBUFFER_WIDTH = GL_RENDERBUFFER_WIDTH as isize,
    RENDERBUFFER_HEIGHT = GL_RENDERBUFFER_HEIGHT as isize,
    RENDERBUFFER_INTERNAL_FORMAT = GL_RENDERBUFFER_INTERNAL_FORMAT as isize,
    RENDERBUFFER_RED_SIZE = GL_RENDERBUFFER_RED_SIZE as isize,
    RENDERBUFFER_GREEN_SIZE = GL_RENDERBUFFER_GREEN_SIZE as isize,
    RENDERBUFFER_BLUE_SIZE = GL_RENDERBUFFER_BLUE_SIZE as isize,
    RENDERBUFFER_ALPHA_SIZE = GL_RENDERBUFFER_ALPHA_SIZE as isize,
    RENDERBUFFER_DEPTH_SIZE = GL_RENDERBUFFER_DEPTH_SIZE as isize,
    RENDERBUFFER_STENCIL_SIZE = GL_RENDERBUFFER_STENCIL_SIZE as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShaderParamType {
    SHADER_TYPE = GL_SHADER_TYPE as isize,
    DELETE_STATUS = GL_DELETE_STATUS as isize,
    COMPILE_STATUS = GL_COMPILE_STATUS as isize,
    INFO_LOG_LENGTH = GL_INFO_LOG_LENGTH as isize,
    SHADER_SOURCE_LENGTH = GL_SHADER_SOURCE_LENGTH as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShaderPrecisionType {
    LOW_FLOAT = GL_LOW_FLOAT as isize,
    MEDIUM_FLOAT = GL_MEDIUM_FLOAT as isize,
    HIGH_FLOAT = GL_HIGH_FLOAT as isize,
    LOW_INT = GL_LOW_INT as isize,
    MEDIUM_INT = GL_MEDIUM_INT as isize,
    HIGH_INT = GL_HIGH_INT as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ConstantType {
    VENDOR = GL_VENDOR as isize,
    RENDERER = GL_RENDERER as isize,
    VERSION = GL_VERSION as isize,
    SHADING_LANGUAGE_VERSION = GL_SHADING_LANGUAGE_VERSION as isize,
    EXTENSIONS = GL_EXTENSIONS as isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureParamType {
    TEXTURE_MAG_FILTER = GL_TEXTURE_MAG_FILTER as isize,
    TEXTURE_MIN_FILTER = GL_TEXTURE_MIN_FILTER as isize,
    TEXTURE_WRAP_S = GL_TEXTURE_WRAP_S as isize,
    TEXTURE_WRAP_T = GL_TEXTURE_WRAP_T as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VertexAttributeParamType {
    VERTEX_ATTRIB_ARRAY_BUFFER_BINDING = GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING as isize,
    VERTEX_ATTRIB_ARRAY_ENABLED = GL_VERTEX_ATTRIB_ARRAY_ENABLED as isize,
    VERTEX_ATTRIB_ARRAY_SIZE = GL_VERTEX_ATTRIB_ARRAY_SIZE as isize,
    VERTEX_ATTRIB_ARRAY_STRIDE = GL_VERTEX_ATTRIB_ARRAY_STRIDE as isize,
    VERTEX_ATTRIB_ARRAY_TYPE = GL_VERTEX_ATTRIB_ARRAY_TYPE as isize,
    VERTEX_ATTRIB_ARRAY_NORMALIZED = GL_VERTEX_ATTRIB_ARRAY_NORMALIZED as isize,
    CURRENT_VERTEX_ATTRIB = GL_CURRENT_VERTEX_ATTRIB as isize,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HintTargetType {
    GENERATE_MIPMAP_HINT = GL_GENERATE_MIPMAP_HINT as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HintBehaviorType {
    FASTEST = GL_FASTEST as isize,
    NICEST = GL_NICEST as isize,
    DONT_CARE = GL_DONT_CARE as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PackParamType {
    PACK_ALIGNMENT = GL_PACK_ALIGNMENT as isize,
    UNPACK_ALIGNMENT = GL_UNPACK_ALIGNMENT as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PixelFormat {
    ALPHA = GL_ALPHA as isize,
    RGB = GL_RGB as isize,
    RGBA = GL_RGBA as isize,
    RGBA4 = GL_RGBA4 as isize,
    RGB565 = GL_RGB565 as isize,
    RGB5_A1 = GL_RGB5_A1 as isize,
    DEPTH_COMPONENT16 = GL_DEPTH_COMPONENT16 as isize,
    STENCIL_INDEX8 = GL_STENCIL_INDEX8 as isize,
    LUMINANCE = GL_LUMINANCE as isize,
    LUMINANCE_ALPHA = GL_LUMINANCE_ALPHA as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PixelDataType {
    UNSIGNED_BYTE = GL_UNSIGNED_BYTE as isize,
    UNSIGNED_SHORT_5_6_5 = GL_UNSIGNED_SHORT_5_6_5 as isize,
    UNSIGNED_SHORT_4_4_4_4 = GL_UNSIGNED_SHORT_4_4_4_4 as isize,
    UNSIGNED_SHORT_5_5_5_1 = GL_UNSIGNED_SHORT_5_5_5_1 as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ActionType {
    KEEP = GL_KEEP as isize,
    ZERO = GL_ZERO as isize,
    REPLACE = GL_REPLACE as isize,
    INCR = GL_INCR as isize,
    INCR_WRAP = GL_INCR_WRAP as isize,
    DECR = GL_DECR as isize,
    DECR_WRAP = GL_DECR_WRAP as isize,
    INVERT = GL_INVERT as isize
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DataType {
    BYTE = GL_BYTE as isize,
    UNSIGNED_BYTE = GL_UNSIGNED_BYTE as isize,
    SHORT = GL_SHORT as isize,
    UNSIGNED_SHORT = GL_UNSIGNED_SHORT as isize,
    FIXED = GL_FIXED as isize,
    FLOAT = GL_FLOAT as isize,
    FLOAT_VEC2 = GL_FLOAT_VEC2 as isize,
    FLOAT_VEC3 = GL_FLOAT_VEC3 as isize,
    FLOAT_VEC4 = GL_FLOAT_VEC4 as isize,
    FLOAT_MAT2 = GL_FLOAT_MAT2 as isize,
    FLOAT_MAT3 = GL_FLOAT_MAT3 as isize,
    FLOAT_MAT4 = GL_FLOAT_MAT4 as isize,
    INT = GL_INT as isize,
    INT_VEC2 = GL_INT_VEC2 as isize,
    INT_VEC3 = GL_INT_VEC3 as isize,
    INT_VEC4 = GL_INT_VEC4 as isize,
    BOOL = GL_BOOL as isize,
    BOOL_VEC2 = GL_BOOL_VEC2 as isize,
    BOOL_VEC3 = GL_BOOL_VEC3 as isize,
    BOOL_VEC4 = GL_BOOL_VEC4 as isize,
    SAMPLER_2D = GL_SAMPLER_2D as isize,
    SAMPLER_CUBE = GL_SAMPLER_CUBE as isize
}

impl From<GLenum> for DataType {
    fn from(data_type: GLenum) -> Self {
        match data_type {
            GL_BYTE => DataType::BYTE,
            GL_UNSIGNED_BYTE => DataType::UNSIGNED_BYTE,
            GL_SHORT => DataType::SHORT,
            GL_UNSIGNED_SHORT => DataType::UNSIGNED_SHORT,
            GL_FIXED => DataType::FIXED,
            GL_FLOAT => DataType::FLOAT,
            GL_FLOAT_VEC2 => DataType::FLOAT_VEC2,
            GL_FLOAT_VEC3 => DataType::FLOAT_VEC3,
            GL_FLOAT_VEC4 => DataType::FLOAT_VEC4,
            GL_FLOAT_MAT2 => DataType::FLOAT_MAT2,
            GL_FLOAT_MAT3 => DataType::FLOAT_MAT3,
            GL_FLOAT_MAT4 => DataType::FLOAT_MAT4,
            GL_INT => DataType::INT,
            GL_INT_VEC2 => DataType::INT_VEC2,
            GL_INT_VEC3 => DataType::INT_VEC3,
            GL_INT_VEC4 => DataType::INT_VEC4,
            GL_BOOL => DataType::BOOL,
            GL_BOOL_VEC2 => DataType::BOOL_VEC2,
            GL_BOOL_VEC3 => DataType::BOOL_VEC3,
            GL_BOOL_VEC4 => DataType::BOOL_VEC4,
            GL_SAMPLER_2D => DataType::SAMPLER_2D,
            GL_SAMPLER_CUBE => DataType::SAMPLER_CUBE,
            _ => DataType::BYTE
        }
    }
}