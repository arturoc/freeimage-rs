#[link(name = "freeimage",
	vers = "0.1",
	uuid = "57e00cd0-19ab-11e3-8ffd-0800200c9a66",
	author = "Tomasz Stachowiak",
	url = "https://github.com/h3r2tic/freeimage-rs")];

#[comment = "Bindings and wrapper functions for FreeImage."];
#[crate_type = "lib"];

// TODO: Document differences between GLFW and glfw-rs

use std::libc::*;
use std::ptr;
//use std::str;
//use std::vec;

// re-export constants
pub use consts::*;

pub mod ffi;
pub mod consts;
//mod private;


#[deriving(Eq, IterBytes)]
pub struct Bitmap {
    ptr: *ffi::Bitmap,
}

impl Drop for Bitmap {
	#[fixed_stack_segment] #[inline(never)]
	fn drop( &mut self ) {
		unsafe { ffi::FreeImage_Unload( self.ptr ); }
	}
}

#[fixed_stack_segment] #[inline(never)]
pub fn init() {
	unsafe { ffi::FreeImage_Initialise( 0 ) }
}

#[fixed_stack_segment] #[inline(never)]
pub fn get_file_type(filename: &str) -> FormatIdentifier {
	unsafe {
		do filename.with_c_str |fname| {
			ffi::FreeImage_GetFileType( fname, 0 )
		}
	}
}

#[fixed_stack_segment] #[inline(never)]
pub fn supports_reading(fif: FormatIdentifier) -> bool {
	unsafe {
		return 1 == ffi::FreeImage_FIFSupportsReading( fif );
	}
}

impl Bitmap {
	#[fixed_stack_segment] #[inline(never)]
	pub fn load(fif: FormatIdentifier, filename: &str) -> Result<Bitmap,~str> {
		unsafe {
			do filename.with_c_str |fname| {
				ffi::FreeImage_Load( fif, fname, 0 )
					.to_option().map_default(Err( ~"FreeImage_Load returned null" ),
						| &ptr | Ok(
							Bitmap { ptr: ptr::to_unsafe_ptr( ptr ) }
						)
					)
			}
		}
	}

	#[fixed_stack_segment] #[inline(never)]
	pub fn get_width(&self) -> uint {
		unsafe { return ffi::FreeImage_GetWidth( self.ptr ) as uint; }
	}

	#[fixed_stack_segment] #[inline(never)]
	pub fn get_height(&self) -> uint {
		unsafe { return ffi::FreeImage_GetHeight( self.ptr ) as uint; }
	}

	#[fixed_stack_segment] #[inline(never)]
	pub fn get_bpp(&self) -> uint {
		unsafe { return ffi::FreeImage_GetBPP( self.ptr ) as uint; }
	}

	#[fixed_stack_segment] #[inline(never)]
	pub fn get_pitch(&self) -> uint {
		unsafe { return ffi::FreeImage_GetPitch( self.ptr ) as uint; }
	}

	#[fixed_stack_segment] #[inline(never)]
	fn get_bits_unsafe(&self) -> *u8 {
		unsafe { ffi::FreeImage_GetBits( self.ptr ) }
	}

	#[fixed_stack_segment] #[inline(never)]
	fn get_scanline_unsafe(&self, scanline: uint) -> *u8 {
		unsafe { ffi::FreeImage_GetScanLine( self.ptr, scanline as c_int ) }
	}

	pub fn get_bits<U>(&self, f: &fn(v: &[u8]) -> U) -> U {
		let pitch = self.get_pitch();
		let height = self.get_height();

		unsafe { std::vec::raw::buf_as_slice( self.get_bits_unsafe(), pitch * height, f ) }
	}

	pub fn get_scanline<U>(&self, scanline: uint, f: &fn(v: &[u8]) -> U) -> U {
		let pitch = self.get_pitch();
		let height = self.get_height();

		if scanline >= height {
			fail!( "Scanline index out of bounds: %? out of %?", scanline, height )
		} else {
			let bits = self.get_scanline_unsafe( scanline );

			if std::ptr::is_null( bits ) {
				fail!( "FreeImage_GetScanLine returned null" )
			} else {
				unsafe { std::vec::raw::buf_as_slice( bits, pitch, f ) }
			}
		}
	}
}
