pub mod jpeg;
pub mod png;

use std::path::PathBuf;

use anyhow::{bail, ensure, Result};
use bitvec::bits;
use bitvec::prelude::*;

use crate::options::ExtraArgs;

use self::jpeg::JpegDecoder;
use self::png::PngDecoder;

pub trait Decoder {
    fn read(&self, data: &mut BitSlice<u8>, seek: usize, max_step: usize) -> Result<()>;
    fn total_size(&self) -> usize;
    fn extra(&self) -> &ExtraArgs;

    fn read_data(&self) -> Result<Vec<u8>> {
        let size = bits![mut u8, Lsb0; 0u8; 32];
        self.read(size, 0, 0)?;
        let size: usize = size.load();

        if self.extra().max_step.is_none() {
            self.check_size(size)?;
        }

        let (data_size, max_step) = self.data_size(size)?;
        let mut data = vec![0u8; data_size];
        self.read(data.view_bits_mut(), 32, max_step)?;

        Ok(data)
    }

    fn check_size(&self, data_size: usize) -> Result<()> {
        let total_size = self.total_size();
        let data_size = data_size << 3;
        ensure!(data_size != 0, "no data found");
        ensure!(
            data_size <= self.total_size(),
            "invalid data size: data {data_size} vs image {total_size}",
        );
        Ok(())
    }

    fn data_size(&self, data_size: usize) -> Result<(usize, usize)> {
        match self.extra().max_step {
            Some(max_step) => {
                let data_size = (self.total_size() / max_step) >> 3;
                ensure!(data_size > 0, "too big step");
                Ok((data_size, max_step))
            }
            None => Ok((data_size, self.total_size() / (data_size << 3))),
        }
    }
}

pub fn new_decoder(input: PathBuf, extra_args: ExtraArgs) -> Result<Box<dyn Decoder>> {
    let image_buf = std::fs::read(input)?;
    match image::guess_format(&image_buf)? {
        image::ImageFormat::Png => Ok(Box::new(PngDecoder::new(
            image::load_from_memory(&image_buf)?,
            extra_args,
        )?)),
        image::ImageFormat::Jpeg => Ok(Box::new(JpegDecoder::new(&image_buf, extra_args)?)),
        _ => bail!("invalid image format"),
    }
}
