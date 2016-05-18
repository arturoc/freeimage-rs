#![crate_name = "freeimage"]
#![crate_type = "lib"]
#![allow(non_camel_case_types,dead_code,non_snake_case,non_upper_case_globals)]

extern crate libc;
#[macro_use] extern crate enum_primitive;
extern crate num;


use std::slice;
use std::mem;
use std::ptr;
use std::ffi::CString;
use num::FromPrimitive;

// re-export constants
pub use consts::*;
pub mod ffi;
pub mod consts;

pub struct Bitmap {
    ptr: *const ffi::FIBITMAP,
}

unsafe impl Send for Bitmap{}

impl Clone for Bitmap{
    fn clone(&self) -> Bitmap{
        Bitmap{ ptr: unsafe{ ffi::FreeImage_Clone(self.ptr) } }
    }
}

enum_from_primitive! {
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type{
    UNKNOWN = 0,
    BITMAP = 1,
    UINT16 = 2,
    INT16 = 3,
    UINT32 = 4,
    INT32 = 5,
    FLOAT = 6,
    DOUBLE = 7,
    COMPLEX = 8,
    RGB16 = 9,
    RGBA16 = 10,
    RGBF = 11,
    RGBAF = 12,
}
}


enum_from_primitive! {
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Filter{
    BOX = 0,
    BILINEAR = 1,
    BSPLINE = 2,
    BICUBIC = 3,
    CATMULLROM = 4,
    LANCZOS3 = 5,
}
}

impl Drop for Bitmap {

	fn drop( &mut self ) {
		unsafe { ffi::FreeImage_Unload( self.ptr ); }
	}
}

pub fn init() {
	unsafe { ffi::FreeImage_Initialise( 0 ) }
}

unsafe fn file_type(filename: &str) -> Format {
	ffi::FreeImage_GetFileType( CString::new(filename.as_bytes()).unwrap().as_ptr(), 0 )
}

unsafe fn file_type_from_name(filename: &str) -> Format {
    ffi::FreeImage_GetFIFFromFilename(CString::new(filename.as_bytes()).unwrap().as_ptr())
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
				Ok(Bitmap { ptr: mem::transmute(ptr) })
			}
		}
	}

    pub fn load_from_memory(buffer: Vec<u8>) -> Result<Bitmap,&'static str> {
		unsafe {
            let hmem = ffi::FreeImage_OpenMemory( mem::transmute(buffer.as_ptr()), buffer.len() as u32 );
            if hmem.is_null(){
                Err( "FreeImage_OpenMemory returned null" )
            }else{
                let fif = ffi::FreeImage_GetFileTypeFromMemory(mem::transmute(hmem), 0);
                if fif!=Format::UNKNOWN{
                    let ptr = ffi::FreeImage_LoadFromMemory(fif,mem::transmute(hmem),0);
                    if ptr.is_null(){
                        Err( "FreeImage_LoadFromMemory returned null" )
                    }else{
						Ok(Bitmap { ptr: mem::transmute(ptr) })
                    }
                }else{
                    Err( "FreeImage_GetFileTypeFromMemory returned UNKOWN" )
                }
            }
		}
    }

    pub fn new<T: Clone>(ty: Type, width: usize, height: usize, bpp: usize, data: Option<&[T]>) -> Bitmap{
        unsafe{
            let ptr = ffi::FreeImage_AllocateT(ty as i32, width as i32, height as i32, bpp as i32, 0, 0, 0);
            let mut bitmap = Bitmap{ ptr: ptr };
            if let Some(data) = data{
                let bytes_per_channel = mem::size_of::<T>();
                let bytes_pp =  bpp / 8;
                let channels = bytes_pp / bytes_per_channel;
                if bitmap.pitch() != width * bytes_pp{
                    for (line_src, line_dst) in data.chunks(width * channels).zip(bitmap.scanlines_mut()){
                        ptr::copy(line_src.as_ptr(), mem::transmute(line_dst.as_mut_ptr()), line_src.len() * bytes_per_channel);
                    }
                }else{
                    bitmap.pixels_mut().clone_from_slice(data);
                }
            }
            bitmap
        }
    }

	pub fn save(&self, filename: &str, flags: i32) -> Result<(),String>{
	    unsafe{
	        let format = file_type_from_name(filename);
	        if supports_writting(format){
				ffi::FreeImage_Save(format, self.ptr, CString::new(filename.as_bytes()).unwrap().as_ptr(),flags);
                Ok(())
	        }else{
                Err(format!("Format {:?} doesn't support writing", format))
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

    pub fn ty(&self) -> Type{
        unsafe{ Type::from_i32(ffi::FreeImage_GetImageType( self.ptr )).unwrap() }
    }


	pub fn pitch(&self) -> usize {
		unsafe { ffi::FreeImage_GetPitch( self.ptr ) as usize }
	}


	fn bits_unsafe(&self) -> *const u8 {
		unsafe { ffi::FreeImage_GetBits( self.ptr ) }
	}

	fn bits_unsafe_mut(&mut self) -> *mut u8 {
		unsafe { ffi::FreeImage_GetBits( self.ptr ) as *mut u8}
	}


	unsafe fn scanline_unsafe(&self, scanline: usize) -> *const u8 {
		ffi::FreeImage_GetScanLine( self.ptr, scanline as libc::c_int ) as *const u8
	}

	unsafe fn scanline_unsafe_mut(&self, scanline: usize) -> *mut u8 {
		ffi::FreeImage_GetScanLine( self.ptr, scanline as libc::c_int ) as *mut u8
	}

	pub fn bits(&self) -> &[u8] {
		let pitch = self.pitch();
		let height = self.height();

		unsafe{ slice::from_raw_parts( self.bits_unsafe(), pitch * height) }
	}

	pub fn pixels<T>(&self) -> &[T] {
		let pitch = self.pitch();
		let height = self.height();

		unsafe{ slice::from_raw_parts( mem::transmute(self.bits_unsafe()), pitch / mem::size_of::<T>() * height) }
	}

	pub fn bits_mut(&mut self) -> &mut [u8] {
		let pitch = self.pitch();
		let height = self.height();

		unsafe{ slice::from_raw_parts_mut( self.bits_unsafe_mut(), pitch * height) }
	}

	pub fn pixels_mut<T>(&mut self) -> &mut [T] {
		let pitch = self.pitch();
		let height = self.height();

		unsafe{ slice::from_raw_parts_mut( mem::transmute(self.bits_unsafe_mut()), pitch / mem::size_of::<T>() * height) }
	}

	pub fn scanlines<'a>(&'a self) -> ScanLines<'a>{
	    ScanLines{ bitmap: self, line: 0 }
	}

	pub fn scanlines_mut(&mut self) -> ScanLinesMut{
	    ScanLinesMut{ bitmap: self, line: 0 }
	}

    pub fn flip_vertical(self) -> Result<Bitmap,String>{
        unsafe{
            if ffi::FreeImage_FlipVertical(self.ptr) == 0{
                Err("Couldn't flip vertically".to_string())
            }else{
                Ok(self)
            }
        }
    }

    pub fn rescale(&self, w: usize, h: usize, filter: Filter) -> Result<Bitmap, String>{
        unsafe{
            let scaled_ptr = ffi::FreeImage_Rescale(self.ptr, w as i32, h as i32, filter as ffi::FREE_IMAGE_FILTER);
            if scaled_ptr == ptr::null(){
                Err("Couldn't scale image".to_string())
            }else{
                Ok(Bitmap{ptr: scaled_ptr})
            }
        }
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


pub struct ScanLinesMut<'a>{
    bitmap: &'a mut Bitmap,
    line: usize
}

impl<'a> Iterator for ScanLinesMut<'a>{
    type Item = &'a mut [u8];
    fn next(&mut self) -> Option<&'a mut [u8]>{
		let pitch = self.bitmap.pitch();
		let height = self.bitmap.height();

		if self.line == height {
			None
		} else {
			let bits = unsafe { self.bitmap.scanline_unsafe_mut(self.line) };

			if bits.is_null() {
				None
			} else {
			    self.line += 1;
				unsafe{ Some(slice::from_raw_parts_mut( bits, pitch )) }
			}
		}
    }
}
