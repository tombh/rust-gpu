use glam::Vec4;

use crate::ScalarOrVector;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Dimensionality {
    OneD = 0,
    TwoD = 1,
    ThreeD = 2,
    Cube = 3,
    Rect = 4,
    Buffer = 5,
    SubpassData = 6,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ImageDepth {
    No = 0,
    Yes = 1,
    Unknown = 2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Sampled {
    Unknown = 0,
    Yes = 1,
    No = 2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ImageFormat {
    Unknown = 0,
    Rgba32f = 1,
    Rgba16f = 2,
    R32f = 3,
    Rgba8 = 4,
    Rgba8Snorm = 5,
    Rg32f = 6,
    Rg16f = 7,
    R11fG11fB10f = 8,
    R16f = 9,
    Rgba16 = 10,
    Rgb10A2 = 11,
    Rg16 = 12,
    Rg8 = 13,
    R16 = 14,
    R8 = 15,
    Rgba16Snorm = 16,
    Rg16Snorm = 17,
    Rg8Snorm = 18,
    R16Snorm = 19,
    R8Snorm = 20,
    Rgba32i = 21,
    Rgba16i = 22,
    Rgba8i = 23,
    R32i = 24,
    Rg32i = 25,
    Rg16i = 26,
    Rg8i = 27,
    R16i = 28,
    R8i = 29,
    Rgba32ui = 30,
    Rgba16ui = 31,
    Rgba8ui = 32,
    R32ui = 33,
    Rgb10a2ui = 34,
    Rg32ui = 35,
    Rg16ui = 36,
    Rg8ui = 37,
    R16ui = 38,
    R8ui = 39,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AccessQualifier {
    ReadOnly = 0,
    WriteOnly = 1,
    ReadWrite = 2,
}

#[allow(unused_attributes)]
#[spirv(image(
    // sampled_type is hardcoded to f32 for now
    dim = "Dim2D",
    depth = 0,
    arrayed = 0,
    multisampled = 0,
    sampled = 1,
    image_format = "Unknown"
))]
#[derive(Copy, Clone)]
pub struct Image<
    const DIM: Dimensionality,
    const DEPTH: ImageDepth,
    const ARRAYED: bool,
    const MULTISAMPLED: bool,
    const SAMPLED: Sampled,
    const FORMAT: ImageFormat,
    const ACCESS_QUALIFIER: Option<AccessQualifier>,
> {
    _x: u32,
}

impl<
        const DIM: Dimensionality,
        const DEPTH: ImageDepth,
        const ARRAYED: bool,
        const MULTISAMPLED: bool,
        const SAMPLED: Sampled,
        const FORMAT: ImageFormat,
        const ACCESS_QUALIFIER: Option<AccessQualifier>,
    >
    Image<
        { DIM },
        { DEPTH },
        { ARRAYED },
        { MULTISAMPLED },
        { SAMPLED },
        { FORMAT },
        { ACCESS_QUALIFIER },
    >
{
    pub fn sample(&self, sampler: Sampler, coord: impl ScalarOrVector) -> Vec4 {
        #[cfg(not(target_arch = "spirv"))]
        {
            let _ = sampler;
            let _ = coord;
            panic!("Image sampling not supported on CPU");
        }

        #[cfg(target_arch = "spirv")]
        unsafe {
            let mut result = Default::default();
            asm!(
                "%typeSampledImage = OpTypeSampledImage typeof*{1}",
                "%image = OpLoad typeof*{1} {1}",
                "%sampler = OpLoad typeof*{2} {2}",
                "%coord = OpLoad typeof*{3} {3}",
                "%sampledImage = OpSampledImage %typeSampledImage %image %sampler",
                "%result = OpImageSampleImplicitLod typeof*{0} %sampledImage %coord",
                "OpStore {0} %result",
                in(reg) &mut result,
                in(reg) self,
                in(reg) &sampler,
                in(reg) &coord
            );
            result
        }
    }
}

#[allow(unused_attributes)]
#[spirv(sampler)]
#[derive(Copy, Clone)]
pub struct Sampler {
    _x: u32,
}

#[allow(unused_attributes)]
#[spirv(sampled_image)]
#[derive(Copy, Clone)]
pub struct SampledImage<I> {
    _image: I,
}

impl<
        const DIM: Dimensionality,
        const DEPTH: ImageDepth,
        const ARRAYED: bool,
        const MULTISAMPLED: bool,
        const SAMPLED: Sampled,
        const FORMAT: ImageFormat,
        const ACCESS_QUALIFIER: Option<AccessQualifier>,
    >
    SampledImage<
        Image<
            { DIM },
            { DEPTH },
            { ARRAYED },
            { MULTISAMPLED },
            { SAMPLED },
            { FORMAT },
            { ACCESS_QUALIFIER },
        >,
    >
{
    pub fn sample(&self, coord: impl ScalarOrVector) -> Vec4 {
        #[cfg(not(target_arch = "spirv"))]
        {
            let _ = coord;
            panic!("Image sampling not supported on CPU");
        }
        #[cfg(target_arch = "spirv")]
        unsafe {
            let mut result = Default::default();
            asm!(
                "%sampledImage = OpLoad typeof*{1} {1}",
                "%coord = OpLoad typeof*{2} {2}",
                "%result = OpImageSampleImplicitLod typeof*{0} %sampledImage %coord",
                "OpStore {0} %result",
                in(reg) &mut result,
                in(reg) self,
                in(reg) &coord
            );
            result
        }
    }
}
