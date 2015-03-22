#![crate_name = "freeimage"]
#![crate_type = "lib"]
#![allow(non_camel_case_types)]

extern crate libc;
extern crate core;


// TODO: Document differences between GLFW and glfw-rs

use libc::*;
use std::slice;
use core::intrinsics::transmute;
use std::ffi::CString;

// re-export constants
pub use freeimage_consts::*;

pub mod ffi;
pub mod freeimage_consts;
//mod private;


pub struct Bitmap {
    ptr: *const ffi::FIBITMAP,
}

impl Drop for Bitmap {

	fn drop( &mut self ) {
		unsafe { ffi::FreeImage_Unload( self.ptr ); }
	}
}

pub fn init() {
	unsafe { ffi::FreeImage_Initialise( 0 ) }
}

pub fn get_file_type(filename: &str) -> FormatIdentifier {
	unsafe {
		ffi::FreeImage_GetFileType( CString::from_slice(filename.as_bytes()).as_ptr(), 0 )
	}
}

pub fn supports_reading(fif: FormatIdentifier) -> bool {
	unsafe {
		return 1 == ffi::FreeImage_FIFSupportsReading( fif );
	}
}

impl Bitmap {

	pub fn load(fif: FormatIdentifier, filename: &str) -> Result<Bitmap,&'static str> {
		unsafe {
			match ffi::FreeImage_Load( fif, CString::from_slice(filename.as_bytes()).as_ptr(), 0 ).as_ref(){
				Some(ptr) => Ok(Bitmap { ptr: transmute(ptr) }),
				None      => Err( "FreeImage_Load returned null" )
			}
		}
	}
    
    pub fn new_from_buffer(buffer: Vec<u8>) -> Result<Bitmap,&'static str> {
		unsafe {
            match ffi::FreeImage_OpenMemory( transmute(buffer.as_ptr()), buffer.len() as u32 ).as_ref(){
                Some(hmem) => {
                    let fif = ffi::FreeImage_GetFileTypeFromMemory(transmute(hmem), 0);
                    if fif!=FormatIdentifier::UNKNOWN{
                        match ffi::FreeImage_LoadFromMemory(fif,transmute(hmem),0).as_ref(){
                            Some(ptr)   => Ok(Bitmap { ptr: transmute(ptr) }),
                            None        => Err( "FreeImage_LoadFromMemory returned null" )
                        }
                    }else{
                        Err( "FreeImage_GetFileTypeFromMemory returned UNKOWN" )
                    }
                }
                None      => Err( "FreeImage_OpenMemory returned null" )
            }
		}
    }


	pub fn get_width(&self) -> uint {
		unsafe { return ffi::FreeImage_GetWidth( self.ptr ) as uint; }
	}


	pub fn get_height(&self) -> uint {
		unsafe { return ffi::FreeImage_GetHeight( self.ptr ) as uint; }
	}


	pub fn get_bpp(&self) -> uint {
		unsafe { return ffi::FreeImage_GetBPP( self.ptr ) as uint; }
	}


	pub fn get_pitch(&self) -> uint {
		unsafe { return ffi::FreeImage_GetPitch( self.ptr ) as uint; }
	}


	fn get_bits_unsafe(&self) -> *const u8 {
		unsafe { ffi::FreeImage_GetBits( self.ptr ) as *const u8}
	}


	fn get_scanline_unsafe(&self, scanline: uint) -> *const u8 {
		unsafe { ffi::FreeImage_GetScanLine( self.ptr, scanline as c_int ) as *const u8}
	}

	pub fn get_bits<U,T: Fn(&[u8]) -> U>(&self, f: T) -> U {
		let pitch = self.get_pitch();
		let height = self.get_height();
		
		unsafe{ f(slice::from_raw_buf( &self.get_bits_unsafe(), pitch * height)) }
	}

	pub fn get_scanline<U, T: Fn(&[u8]) -> U>(&self, scanline: uint, f: T) -> U {
		let pitch = self.get_pitch();
		let height = self.get_height();

		if scanline >= height {
			panic!( "Scanline index out of bounds: {:?} out of {:?}", scanline, height )
		} else {
			let bits = self.get_scanline_unsafe( scanline );

			if bits.is_null() {
				panic!( "FreeImage_GetScanLine returned null" )
			} else {
				unsafe{ f(slice::from_raw_buf( &bits, pitch )) }
			}
		}
	}
}
