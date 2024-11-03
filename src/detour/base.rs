use cxx::{type_id, ExternType};

use crate::error::{RNError, RNResult};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DtAABB {
    pub a: [f32; 3],
    pub b: [f32; 3],
}

#[allow(dead_code)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("recastnavigation-rs/src/detour/detour-ffi.h");

        type dtStatus = crate::detour::base::DtStatus;

        type c_void;
        unsafe fn dtFree(ptr: *mut c_void);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DtStatus(pub u32);

unsafe impl ExternType for DtStatus {
    type Id = type_id!("dtStatus");
    type Kind = cxx::kind::Trivial;
}

impl DtStatus {
    pub fn to_result(self) -> RNResult<()> {
        if self.0 & DT_SUCCESS != 0 {
            Ok(())
        } else if self.0 & DT_FAILURE != 0 {
            match self.0 & DT_STATUS_DETAIL_MASK {
                DT_WRONG_MAGIC => return Err(RNError::WrongMagic),
                DT_WRONG_VERSION => return Err(RNError::WrongVersion),
                DT_OUT_OF_MEMORY => return Err(RNError::OutOfMemory),
                DT_INVALID_PARAM => return Err(RNError::InvalidParam),
                DT_BUFFER_TOO_SMALL => return Err(RNError::BufferTooSmall),
                DT_OUT_OF_NODES => return Err(RNError::OutOfNodes),
                DT_PARTIAL_RESULT => return Err(RNError::PartialResult),
                DT_ALREADY_OCCUPIED => return Err(RNError::AlreadyOccupied),
                _ => return Err(RNError::Failed),
            }
        } else if self.0 & DT_IN_PROGRESS != 0 {
            return Err(RNError::InProgress);
        } else {
            return Err(RNError::Failed);
        }
    }
}

const DT_FAILURE: u32 = 1 << 31;
const DT_SUCCESS: u32 = 1 << 30;
const DT_IN_PROGRESS: u32 = 1 << 29;

const DT_STATUS_DETAIL_MASK: u32 = 0x0ffffff;
const DT_WRONG_MAGIC: u32 = 1 << 0;
const DT_WRONG_VERSION: u32 = 1 << 1;
const DT_OUT_OF_MEMORY: u32 = 1 << 2;
const DT_INVALID_PARAM: u32 = 1 << 3;
const DT_BUFFER_TOO_SMALL: u32 = 1 << 4;
const DT_OUT_OF_NODES: u32 = 1 << 5;
const DT_PARTIAL_RESULT: u32 = 1 << 6;
const DT_ALREADY_OCCUPIED: u32 = 1 << 7;

#[derive(Debug)]
pub struct DtBuf {
    pub(crate) data: *mut u8,
    pub(crate) size: i32,
}

impl Default for DtBuf {
    #[inline]
    fn default() -> Self {
        DtBuf::from_raw(std::ptr::null_mut(), 0)
    }
}

impl Drop for DtBuf {
    fn drop(&mut self) {
        unsafe { ffi::dtFree(self.data as *mut _) };
        self.data = std::ptr::null_mut();
    }
}

impl DtBuf {
    #[inline]
    pub(crate) fn from_raw(data: *mut u8, size: i32) -> DtBuf {
        DtBuf { data, size }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        return unsafe { std::slice::from_raw_parts(self.data, self.size as usize) };
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        return unsafe { std::slice::from_raw_parts_mut(self.data, self.size as usize) };
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[inline]
pub fn dt_vcross(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
pub fn dt_vdot(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
pub fn dt_vdot_2d(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    a[0] * b[0] + a[2] * b[2]
}

#[inline]
pub fn dt_vmad(a: &[f32; 3], b: &[f32; 3], s: f32) -> [f32; 3] {
    [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
}

#[inline]
pub fn dt_vlerp(a: &[f32; 3], b: &[f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

#[inline]
pub fn dt_vadd(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
pub fn dt_vsub(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
pub fn dt_vscale(a: &[f32; 3], b: f32) -> [f32; 3] {
    [a[0] * b, a[1] * b, a[2] * b]
}

#[inline]
pub fn dt_vmin(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])]
}

#[inline]
pub fn dt_vmax(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]
}

#[inline]
pub fn dt_vlen(a: &[f32; 3]) -> f32 {
    dt_vlen2(a).sqrt()
}

#[inline]
pub fn dt_vlen2(a: &[f32; 3]) -> f32 {
    a[0] * a[0] + a[1] * a[1] + a[2] * a[2]
}

#[inline]
pub fn dt_vdist(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    dt_vlen(&dt_vsub(a, b))
}

#[inline]
pub fn dt_vdist2(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    dt_vlen2(&dt_vsub(a, b))
}

#[inline]
pub fn dt_vdist_2d(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    dt_vdist2_2d(a, b).sqrt()
}

#[inline]
pub fn dt_vdist2_2d(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    let dx = a[0] - b[0];
    let dz = a[2] - b[2];
    dx * dx + dz * dz
}

#[inline]
pub fn dt_vnormalize(a: &[f32; 3]) -> [f32; 3] {
    let d = 1.0 / dt_vlen(a);
    [a[0] * d, a[1] * d, a[2] * d]
}

#[inline]
pub fn dt_visinf(a: &[f32; 3]) -> bool {
    a[0].is_infinite() && a[1].is_infinite() && a[2].is_infinite()
}

#[inline]
pub fn dt_visinf_2d(a: &[f32; 3]) -> bool {
    a[0].is_infinite() && a[2].is_infinite()
}

#[inline]
pub fn dt_vperp_2d(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    a[2] * b[0] - a[0] * b[2]
}

#[inline]
pub fn dt_tri_area_2d(a: &[f32; 3], b: &[f32; 3], c: &[f32; 3]) -> f32 {
    let abx = b[0] - a[0];
    let abz = b[2] - a[2];
    let acx = c[0] - a[0];
    let acz = c[2] - a[2];
    acx * abz - abx * acz
}

#[inline]
pub fn dt_overlap_bounds(amin: &[i32; 3], amax: &[i32; 3], bmin: &[i32; 3], bmax: &[i32; 3]) -> bool {
    let mut overlap = true;
    overlap = if amin[0] > bmax[0] || amax[0] < bmin[0] {
        false
    } else {
        overlap
    };
    overlap = if amin[1] > bmax[1] || amax[1] < bmin[1] {
        false
    } else {
        overlap
    };
    overlap = if amin[2] > bmax[2] || amax[2] < bmin[2] {
        false
    } else {
        overlap
    };
    overlap
}
