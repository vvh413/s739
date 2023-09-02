use std::ffi::CString;

use anyhow::Result;
use mozjpeg_sys::{
  jpeg_compress_struct, jpeg_create_compress, jpeg_create_decompress, jpeg_decompress_struct, jpeg_error_mgr,
  jpeg_std_error, jpeg_stdio_dest, jpeg_stdio_src,
};

pub unsafe fn open(file: &str, mode: &str) -> Result<*mut libc::FILE> {
  let file = CString::new(file)?;
  let mode = CString::new(mode)?;
  Ok(libc::fopen(file.as_ptr(), mode.as_ptr()))
}

pub unsafe fn decompress(file: &str) -> Result<(jpeg_decompress_struct, *mut libc::FILE)> {
  let mut err: jpeg_error_mgr = std::mem::zeroed();
  let mut cinfo: jpeg_decompress_struct = std::mem::zeroed();
  cinfo.common.err = jpeg_std_error(&mut err);
  jpeg_create_decompress(&mut cinfo);

  let file = open(file, "rb")?;
  jpeg_stdio_src(&mut cinfo, file);

  Ok((cinfo, file))
}

pub unsafe fn compress(file: &str) -> Result<(jpeg_compress_struct, *mut libc::FILE)> {
  let mut err: jpeg_error_mgr = std::mem::zeroed();
  let mut cinfo: jpeg_compress_struct = std::mem::zeroed();
  cinfo.common.err = jpeg_std_error(&mut err);
  jpeg_create_compress(&mut cinfo);

  let file = open(file, "wb")?;
  jpeg_stdio_dest(&mut cinfo, file);

  Ok((cinfo, file))
}

pub unsafe fn get_total_size(cinfo: &jpeg_decompress_struct) -> usize {
  let mut size = 0;
  for comp in 0..cinfo.num_components as isize {
    let comp_info = cinfo.comp_info.offset(comp);
    size += (*comp_info).height_in_blocks * (*comp_info).width_in_blocks * 64;
  }
  size as usize
}
