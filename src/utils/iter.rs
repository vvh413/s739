use std::ops::ShrAssign;
use std::slice::{Iter, IterMut};

use anyhow::{bail, Result};
use bitvec::prelude::*;
use rand::Rng;

use crate::options::ExtraArgs;

use super::jpeg::selective_check;
use std::ops::BitAnd;

pub fn get_n_bits<T: From<u8>>(
    data_iter: &mut bitvec::slice::Iter<'_, u8, Lsb0>,
    n_bits: usize,
) -> Result<T> {
    let mut bits: u8 = 0;
    for i in (0..n_bits).rev() {
        let bit = match data_iter.next() {
            Some(bit) => bit,
            None if i == n_bits => bail!("no more data"),
            None => return Ok(bits.into()),
        };
        bits |= (if *bit { 1 } else { 0 }) << i;
    }
    Ok(bits.into())
}

pub fn set_n_bits<T>(
    mut value: T,
    data_iter: &mut bitvec::slice::IterMut<'_, u8, Lsb0>,
    n_bits: usize,
) -> Result<()>
where
    T: Copy + From<u8> + BitAnd<Output = T> + ShrAssign + PartialEq<T>,
{
    for _ in 0..n_bits {
        let mut bit = match data_iter.next() {
            Some(bit) => bit,
            None => bail!("no more data"),
        };
        *bit = (value & T::from(1)) == T::from(1);
        value >>= T::from(1);
    }
    Ok(())
}

pub fn rand_step<R: Rng>(rng: &mut R, max_step: usize) -> usize {
    if max_step > 1 {
        rng.random_range(0..max_step)
    } else {
        0
    }
}

pub struct JpegBlockIter<'a> {
    row_iter: Iter<'a, (*mut [i16; 64], usize)>,
    current_row: Option<&'a (*mut [i16; 64], usize)>,
    block_offset: usize,
}

impl<'a> JpegBlockIter<'a> {
    fn new(blocks: &'a super::jpeg::Blocks) -> Self {
        Self {
            row_iter: blocks.inner().iter(),
            current_row: None,
            block_offset: 0,
        }
    }
}

impl Iterator for JpegBlockIter<'_> {
    type Item = *mut [i16; 64];

    fn next(&mut self) -> Option<Self::Item> {
        if self.block_offset == 0 {
            self.current_row = self.row_iter.next();
        }
        match self.current_row {
            Some((row, width)) => {
                let result = Some(unsafe { row.add(self.block_offset) });
                self.block_offset = (self.block_offset + 1) % width;
                result
            }
            None => None,
        }
    }
}

pub struct JpegCoefIter<'a> {
    block_iter: JpegBlockIter<'a>,
    current_block: Option<Iter<'a, i16>>,
    coef_idx: usize,
    extra: ExtraArgs,
}

impl<'a> JpegCoefIter<'a> {
    fn new(blocks: &'a super::jpeg::Blocks, extra: ExtraArgs) -> Self {
        Self {
            block_iter: JpegBlockIter::new(blocks),
            current_block: None,
            coef_idx: 0,
            extra,
        }
    }
}

impl<'a> Iterator for JpegCoefIter<'a> {
    type Item = &'a i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coef_idx == 0 {
            self.current_block = self
                .block_iter
                .next()
                .map(|block| unsafe { (*block).iter() });
        }
        match self.current_block {
            Some(ref mut block) => {
                let curr_idx = self.coef_idx;
                self.coef_idx = (self.coef_idx + 1) % 64;
                match block.next() {
                    Some(coef) if selective_check(&self.extra, curr_idx, *coef) => self.next(),
                    Some(coef) => Some(coef),
                    None => None,
                }
            }
            None => None,
        }
    }
}

pub struct JpegCoefIterMut<'a> {
    block_iter: JpegBlockIter<'a>,
    current_block: Option<IterMut<'a, i16>>,
    coef_idx: usize,
    extra: ExtraArgs,
}

impl<'a> JpegCoefIterMut<'a> {
    fn new(blocks: &'a super::jpeg::Blocks, extra: ExtraArgs) -> Self {
        Self {
            block_iter: JpegBlockIter::new(blocks),
            current_block: None,
            coef_idx: 0,
            extra,
        }
    }
}

impl<'a> Iterator for JpegCoefIterMut<'a> {
    type Item = &'a mut i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coef_idx == 0 {
            self.current_block = self
                .block_iter
                .next()
                .map(|block| unsafe { (*block).iter_mut() });
        }
        match self.current_block {
            Some(ref mut block) => {
                let curr_idx = self.coef_idx;
                self.coef_idx = (self.coef_idx + 1) % 64;
                match block.next() {
                    Some(coef) if selective_check(&self.extra, curr_idx, *coef) => self.next(),
                    Some(coef) => Some(coef),
                    None => None,
                }
            }
            None => None,
        }
    }
}

impl super::jpeg::Blocks {
    pub fn iter(&self, extra: ExtraArgs) -> JpegCoefIter {
        JpegCoefIter::new(self, extra)
    }

    pub fn iter_mut(&self, extra: ExtraArgs) -> JpegCoefIterMut {
        JpegCoefIterMut::new(self, extra)
    }
}
