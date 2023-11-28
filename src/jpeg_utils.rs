use anyhow::Result;
use mozjpeg_sys::{
  jpeg_c_set_int_param, jpeg_compress_struct, jpeg_create_compress, jpeg_create_decompress, jpeg_decompress_struct,
  jpeg_error_mgr, jpeg_mem_dest, jpeg_mem_src, jpeg_std_error, jvirt_barray_control, J_INT_PARAM,
};

use crate::options::JpegOptions;

pub unsafe fn decompress(buffer: &Vec<u8>) -> Result<jpeg_decompress_struct> {
  let mut err: jpeg_error_mgr = std::mem::zeroed();
  let mut cinfo: jpeg_decompress_struct = std::mem::zeroed();
  cinfo.common.err = jpeg_std_error(&mut err);
  jpeg_create_decompress(&mut cinfo);

  jpeg_mem_src(&mut cinfo, buffer.as_ptr(), buffer.len().try_into()?);

  Ok(cinfo)
}

pub unsafe fn compress(buffer_ptr: *mut *mut u8, buffer_size: *mut libc::c_ulong) -> Result<jpeg_compress_struct> {
  let mut err: jpeg_error_mgr = std::mem::zeroed();
  let mut cinfo: jpeg_compress_struct = std::mem::zeroed();
  cinfo.common.err = jpeg_std_error(&mut err);
  jpeg_create_compress(&mut cinfo);

  jpeg_mem_dest(&mut cinfo, buffer_ptr, buffer_size);

  Ok(cinfo)
}

pub unsafe fn get_blocks(
  cinfo: &mut jpeg_decompress_struct,
  coefs_ptr: *mut *mut jvirt_barray_control,
) -> (Vec<(*mut [i16; 64], u32)>, usize) {
  let mut result: Vec<(*mut [i16; 64], u32)> = Vec::new();
  let mut size: u32 = 0;
  let mut buffer;

  for comp in 0..cinfo.num_components as isize {
    let comp_info = cinfo.comp_info.offset(comp);
    size += (*comp_info).height_in_blocks * (*comp_info).width_in_blocks * 64;
    for blk_y in (0..(*comp_info).height_in_blocks).step_by((*cinfo.comp_info).v_samp_factor as usize) {
      buffer = (*cinfo.common.mem).access_virt_barray.unwrap()(
        &mut cinfo.common,
        *coefs_ptr.offset(comp),
        blk_y,
        (*comp_info).v_samp_factor as u32,
        1,
      );
      for offset_y in 0..(*comp_info).v_samp_factor {
        let block = *buffer.offset(offset_y as isize);
        result.push((block, (*comp_info).width_in_blocks));
      }
    }
  }
  (result, size as usize)
}

pub unsafe fn set_options(cinfo: &mut jpeg_compress_struct, jpeg_options: JpegOptions) {
  jpeg_c_set_int_param(
    cinfo,
    J_INT_PARAM::JINT_COMPRESS_PROFILE,
    jpeg_options.compress_profile as i32,
  );
}
