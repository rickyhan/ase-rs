use std::io::{self, Read};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::helpers::read_string;

bitflags! {
	pub struct Flags: u32 {
		const IsNinePatchesSlice = 1;
		const HasPivotInformation = 2;
	}
}

pub struct NinePatchesInfo {
	pub x_position: i32,
	pub y_position: i32,
	pub width: u32,
	pub height: u32,
}

pub struct PivotInfo {
	pub x_position: i32,
	pub y_position: i32,
}

pub struct SliceKey {
	pub frame_number: u32,
	pub x_origin: i32,
	pub y_origin: i32,
	pub width: u32,
	pub height: u32,
	pub nine_patches_info: Option<NinePatchesInfo>,
	pub pivot_info: Option<PivotInfo>,
}

pub struct SliceChunk {
	pub number_of_slice_keys: u32,
	pub flags: Flags,
	pub name: String,
	pub keys: Vec<SliceKey>,
}

impl SliceChunk {
	pub fn from_read<R>(read: &mut R) -> io::Result<Self>
	where
		R: Read,
	{
		let number_of_slice_keys = read.read_u32::<LittleEndian>()?;
		let flags = Flags::from_bits_truncate(read.read_u32::<LittleEndian>()?);
		let name = read_string(read)?;
		let mut keys = Vec::with_capacity(number_of_slice_keys as usize);
		for _ in 0..number_of_slice_keys {
			let frame_number = read.read_u32::<LittleEndian>()?;
			let x_origin = read.read_i32::<LittleEndian>()?;
			let y_origin = read.read_i32::<LittleEndian>()?;
			let width = read.read_u32::<LittleEndian>()?;
			let height = read.read_u32::<LittleEndian>()?;
			let nine_patches_info = if flags.contains(Flags::IsNinePatchesSlice) {
				Some(NinePatchesInfo {
					x_position: read.read_i32::<LittleEndian>()?,
					y_position: read.read_i32::<LittleEndian>()?,
					width: read.read_u32::<LittleEndian>()?,
					height: read.read_u32::<LittleEndian>()?,
				})
			} else {
				None
			};
			let pivot_info = if flags.contains(Flags::HasPivotInformation) {
				Some(PivotInfo {
					x_position: read.read_i32::<LittleEndian>()?,
					y_position: read.read_i32::<LittleEndian>()?,
				})
			} else {
				None
			};

			keys.push(SliceKey {
				frame_number,
				x_origin,
				y_origin,
				width,
				height,
				nine_patches_info,
				pivot_info,
			});
		}

		Ok(Self {
			number_of_slice_keys,
			flags,
			name,
			keys,
		})
	}
}