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
			let ptr = ffi::FreeImage_Load( fif, CString::new(filename.as_bytes()).unwrap().as_ptr(), 0 );
			if ptr.is_null(){
			    Err( "FreeImage_Load returned null" )
			}else{
				Ok(Bitmap { ptr: transmute(ptr) })
			}
		}
	}
    
    pub fn load_from_memory(buffer: Vec<u8>) -> Result<Bitmap,&'static str> {
		unsafe {
            let hmem = ffi::FreeImage_OpenMemory( transmute(buffer.as_ptr()), buffer.len() as u32 );
            if hmem.is_null(){
                Err( "FreeImage_OpenMemory returned null" )
            }else{
                let fif = ffi::FreeImage_GetFileTypeFromMemory(transmute(hmem), 0);
                if fif!=Format::UNKNOWN{
                    let ptr = ffi::FreeImage_LoadFromMemory(fif,transmute(hmem),0);
                    if ptr.is_null(){
                        Err( "FreeImage_LoadFromMemory returned null" )
                    }else{
						Ok(Bitmap { ptr: transmute(ptr) })
                    }
                }else{
                    Err( "FreeImage_GetFileTypeFromMemory returned UNKOWN" )
                }
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


	fn scanline_unsafe(&self, scanline: usize) -> *const u8 {
		unsafe { ffi::FreeImage_GetScanLine( self.ptr, scanline as libc::c_int ) as *const u8}
	}

	pub fn bits(&self) -> &[u8] {
		let pitch = self.pitch();
		let height = self.height();
		
		unsafe{ slice::from_raw_parts( self.bits_unsafe(), pitch * height) }
	}
	
	pub fn scanlines(&self) -> ScanLines{
	    ScanLines{ ptr: self.ptr, line: 0 }
	}
}

pub struct ScanLines{
    ptr: *const ffi::FIBITMAP,
    line: usize
}

impl<'a> Iterator for ScanLines{
    type Item = &'a [u8];
    fn next(&mut self) -> Option<&[u8]>{
		let pitch = unsafe { ffi::FreeImage_GetPitch( self.ptr ) as usize };
		let height = unsafe { ffi::FreeImage_GetHeight( self.ptr ) as usize };

		if self.line == height {
			None
		} else {
			let bits = unsafe { ffi::FreeImage_GetScanLine( self.ptr, self.line as libc::c_int ) as *const u8};

			if bits.is_null() {
				None
			} else {
			    self.line += 1;
				unsafe{ Some(slice::from_raw_parts( bits, pitch )) }
			}
		}
    }
}