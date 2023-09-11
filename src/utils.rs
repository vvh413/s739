use anyhow::Result;
use mozjpeg_sys::{
  jpeg_compress_struct, jpeg_create_compress, jpeg_create_decompress, jpeg_decompress_struct, jpeg_error_mgr,
  jpeg_mem_dest, jpeg_mem_src, jpeg_std_error,
};

pub unsafe fn decompress(buffer: &Vec<u8>) -> Result<jpeg_decompress_struct> {
  let mut err: jpeg_error_mgr = std::mem::zeroed();
  let mut cinfo: jpeg_decompress_struct = std::mem::zeroed();
  cinfo.common.err = jpeg_std_error(&mut err);
  jpeg_create_decompress(&mut cinfo);

  jpeg_mem_src(&mut cinfo, buffer.as_ptr(), buffer.len() as u64);

  Ok(cinfo)
}

pub unsafe fn compress(buffer_ptr: *mut *mut u8, buffer_size: *mut u64) -> Result<jpeg_compress_struct> {
  let mut err: jpeg_error_mgr = std::mem::zeroed();
  let mut cinfo: jpeg_compress_struct = std::mem::zeroed();
  cinfo.common.err = jpeg_std_error(&mut err);
  jpeg_create_compress(&mut cinfo);

  jpeg_mem_dest(&mut cinfo, buffer_ptr, buffer_size);

  Ok(cinfo)
}

pub unsafe fn get_total_size(cinfo: &jpeg_decompress_struct) -> usize {
  let mut size = 0;
  for comp in 0..cinfo.num_components as isize {
    let comp_info = cinfo.comp_info.offset(comp);
    size += (*comp_info).height_in_blocks * (*comp_info).width_in_blocks * 64;
  }
  size as usize
}
