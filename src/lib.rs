#![feature(libc,core)]
#![crate_name = "freeimage"]
#![crate_type = "lib"]
#![allow(non_camel_case_types,dead_code,non_snake_case,non_upper_case_globals)]

extern crate libc;

use std::slice;
use std::mem::transmute;
use std::ffi::CString;

// re-export constants
pub use consts::*;

pub mod ffi;
pub mod consts;


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

pub fn file_type(filename: &str) -> Format {
	unsafe {
		ffi::FreeImage_GetFileType( CString::new(filename.as_bytes()).unwrap().as_ptr(), 0 )
	}
}

pub fn supports_reading(fif: Format) -> bool {
	unsafe {
		1 == ffi::FreeImage_FIFSupportsReading( fif )
	}
}

pub fn supports_writting(fif: Format) -> bool{
    unsafe{
        1 == ffi::FreeImage_FIFSupportsWriting( fif )
    }
}

impl Bitmap {

	pub fn load(fif: Format, filename: &str) -> Result<Bitmap,&'static str> {
		unsafe {
			match ffi::FreeImage_Load( fif, CString::new(filename.as_bytes()).unwrap().as_ptr(), 0 ).as_ref(){
				Some(ptr) => Ok(Bitmap { ptr: transmute(ptr) }),
				None      => Err( "FreeImage_Load returned null" )
			}
		}
	}
    
    pub fn load_from_memory(buffer: Vec<u8>) -> Result<Bitmap,&'static str> {
		unsafe {
            match ffi::FreeImage_OpenMemory( transmute(buffer.as_ptr()), buffer.len() as u32 ).as_ref(){
                Some(hmem) => {
                    let fif = ffi::FreeImage_GetFileTypeFromMemory(transmute(hmem), 0);
                    if fif!=Format::UNKNOWN{
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

	pub fn save(&self, filename: &str, flags: i32){
	    unsafe{
	        let format = file_type(filename);
	        if supports_writting(format){
				ffi::FreeImage_Save(format, self.ptr, CString::new(filename.as_bytes()).unwrap().as_ptr(),flags);
	        }
	    }
	}

	pub fn width(&self) -> usize {
		unsafe { ffi::FreeImage_GetWidth( self.ptr ) as usize }
	}


	pub fn height(&self) -> usize {
		unsafe { ffi::FreeImage_GetHeight( self.ptr ) as usize }
	}


	pub fn bpp(&self) -> usize {
		unsafe { ffi::FreeImage_GetBPP( self.ptr ) as usize }
	}


	pub fn pitch(&self) -> usize {
		unsafe { ffi::FreeImage_GetPitch( self.ptr ) as usize }
	}


	fn bits_unsafe(&self) -> *const u8 {
		unsafe { ffi::FreeImage_GetBits( self.ptr ) as *const u8}
	}


	unsafe fn scanline_unsafe(&self, scanline: usize) -> *const u8 {
		ffi::FreeImage_GetScanLine( self.ptr, scanline as libc::c_int ) as *const u8
	}

	pub fn bits(&self) -> &[u8] {
		let pitch = self.pitch();
		let height = self.height();
		
		unsafe{ slice::from_raw_parts( self.bits_unsafe(), pitch * height) }
	}
	
	pub fn scanlines<'a>(&'a self) -> ScanLines<'a>{
	    ScanLines{ bitmap: self, line: 0 }
	}
}

pub struct ScanLines<'a>{
    bitmap: &'a Bitmap,
    line: usize
}

impl<'a> Iterator for ScanLines<'a>{
    type Item = &'a [u8];
    fn next(&mut self) -> Option<&'a [u8]>{
		let pitch = self.bitmap.pitch();
		let height = self.bitmap.height();

		if self.line == height {
			None
		} else {
			let bits = unsafe { self.bitmap.scanline_unsafe(self.line) };

			if bits.is_null() {
				None
			} else {
			    self.line += 1;
				unsafe{ Some(slice::from_raw_parts( bits, pitch )) }
			}
		}
    }
}
