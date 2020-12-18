#![allow(non_camel_case_types,dead_code,non_snake_case,non_upper_case_globals)]

use std::slice;
use std::mem;
use std::ptr;
use std::ffi::CString;
use std::path::Path;
use std::os::raw::c_int;

// re-export constants
pub use consts::*;
pub mod consts;
pub mod ffi;

pub struct Bitmap {
    ptr: *mut ffi::FIBITMAP,
}

unsafe impl Send for Bitmap{}
unsafe impl Sync for Bitmap{}

impl Clone for Bitmap{
    fn clone(&self) -> Bitmap{
        Bitmap{ ptr: unsafe{ ffi::FreeImage_Clone(self.ptr) } }
    }
}
#[repr(C)]
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

#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Filter{
    BOX = 0,
    BILINEAR = 1,
    BSPLINE = 2,
    BICUBIC = 3,
    CATMULLROM = 4,
    LANCZOS3 = 5,
}

#[derive(Debug)]
pub struct Error{
	msg: &'static str,
}

impl ::std::fmt::Display for Error{
	fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result{
		fmt.write_str(self.msg)
	}
}

impl ::std::error::Error for Error {}

impl Drop for Bitmap {

	fn drop( &mut self ) {
		unsafe { ffi::FreeImage_Unload( self.ptr ); }
	}
}

pub fn init() {
	unsafe { ffi::FreeImage_Initialise( 0 ) }
}

unsafe fn file_type(filename: &str) -> Format {
	let cstr = CString::new(filename.as_bytes()).unwrap();
	ffi::FreeImage_GetFileType( cstr.as_ptr(), 0 ).into()
}

unsafe fn file_type_from_name(filename: &str) -> Format {
	let cstr = CString::new(filename.as_bytes()).unwrap();
    ffi::FreeImage_GetFIFFromFilename(cstr.as_ptr()).into()
}

pub fn supports_reading(fif: Format) -> bool {
	unsafe {
		1 == ffi::FreeImage_FIFSupportsReading( fif  as i32 )
	}
}

pub fn supports_writting(fif: Format) -> bool{
    unsafe{
        1 == ffi::FreeImage_FIFSupportsWriting( fif as i32 )
    }
}

impl Bitmap {
	pub fn load<P: AsRef<Path>>(filename: P) -> Result<Bitmap,Error> {
		unsafe {
			let filename = filename
				.as_ref()
				.to_str()
				.ok_or(Error{ msg: "Couldn't parse filename" })?;
            let cname = CString::new(filename.as_bytes()).unwrap();
            let fif = ffi::FreeImage_GetFIFFromFilename(cname.as_ptr());
			let ptr = ffi::FreeImage_Load(fif, cname.as_ptr(), 0);
			if ptr.is_null(){
			    Err( Error{msg:"FreeImage_Load returned null"} )
			}else{
				Ok(Bitmap { ptr: mem::transmute(ptr) })
			}
		}
	}

	pub fn load_with_format<P: AsRef<Path>>(fif: Format, filename: P) -> Result<Bitmap,Error> {
		unsafe {
			let filename = filename
				.as_ref()
				.to_str()
				.ok_or(Error{ msg: "Couldn't parse filename" })?;
            let cname = CString::new(filename.as_bytes()).unwrap();
			let ptr = ffi::FreeImage_Load( fif  as i32, cname.as_ptr(), 0 );
			if ptr.is_null(){
			    Err( Error{msg:"FreeImage_Load returned null"} )
			}else{
				Ok(Bitmap { ptr: mem::transmute(ptr) })
			}
		}
	}

    pub fn load_from_memory(buffer: &[u8]) -> Result<Bitmap,Error> {
		unsafe {
            let hmem = ffi::FreeImage_OpenMemory(
				buffer.as_ptr() as *mut u8,
				buffer.len() as u32
			);
            if hmem.is_null(){
                Err( Error{msg:"FreeImage_OpenMemory returned null"} )
            }else{
                let fif = ffi::FreeImage_GetFileTypeFromMemory(hmem, 0);
                if fif != Format::UNKNOWN  as i32{
                    let ptr = ffi::FreeImage_LoadFromMemory(fif,hmem,0);
                    if ptr.is_null(){
                        Err( Error{msg:"FreeImage_LoadFromMemory returned null"} )
                    }else{
						Ok(Bitmap { ptr: mem::transmute(ptr) })
                    }
                }else{
                    Err( Error{msg:"FreeImage_GetFileTypeFromMemory returned UNKOWN"} )
                }
            }
		}
    }

    pub fn new<T: Copy>(ty: Type, width: usize, height: usize, bpp: usize, data: Option<&[T]>) -> Bitmap{
        unsafe{
            let ptr = ffi::FreeImage_AllocateT(ty as u32, width as i32, height as i32, bpp as i32, 0, 0, 0);
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
                    bitmap.pixels_mut().copy_from_slice(data);
                }
            }
            bitmap
        }
    }

	pub fn save<P: AsRef<Path>>(&self, filename: P, flags: i32) -> Result<(),Error>{
	    unsafe{
			let filename = filename
				.as_ref()
				.to_str()
				.ok_or(Error{ msg: "Couldn't parse filename" })?;
	        let format = file_type_from_name(filename);
	        if supports_writting(format){
				let cstr = CString::new(filename.as_bytes()).unwrap();
				ffi::FreeImage_Save(format as i32, self.ptr, cstr.as_ptr(),flags);
                Ok(())
	        }else{
                Err(Error{msg: "Format doesn't support writing"})
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
        unsafe{ mem::transmute(ffi::FreeImage_GetImageType( self.ptr )) }
    }


	pub fn pitch(&self) -> usize {
		unsafe { ffi::FreeImage_GetPitch( self.ptr ) as usize }
	}


	unsafe fn bits_unsafe(&self) -> *const u8 {
		ffi::FreeImage_GetBits( self.ptr )
	}

	unsafe fn bits_unsafe_mut(&mut self) -> *mut u8 {
		ffi::FreeImage_GetBits( self.ptr ) as *mut u8
	}

	unsafe fn scanline_unsafe(&self, scanline: usize) -> *const u8 {
		ffi::FreeImage_GetScanLine( self.ptr, scanline as c_int ) as *const u8
	}

	unsafe fn scanline_unsafe_mut(&self, scanline: usize) -> *mut u8 {
		ffi::FreeImage_GetScanLine( self.ptr, scanline as c_int ) as *mut u8
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

    pub fn flip_vertical(self) -> Result<Bitmap,Error>{
        unsafe{
            if ffi::FreeImage_FlipVertical(self.ptr) == 0{
                Err(Error{msg:"Couldn't flip vertically"})
            }else{
                Ok(self)
            }
        }
    }

    pub fn flip_horizontal(self) -> Result<Bitmap,Error>{
        unsafe{
            if ffi::FreeImage_FlipHorizontal(self.ptr) == 0{
                Err(Error{msg:"Couldn't flip horizontally"})
            }else{
                Ok(self)
            }
        }
    }

    pub fn rescale(&self, w: usize, h: usize, filter: Filter) -> Result<Bitmap, Error>{
        unsafe{
            let scaled_ptr = ffi::FreeImage_Rescale(self.ptr, w as i32, h as i32, filter as ffi::FREE_IMAGE_FILTER);
            if scaled_ptr == ptr::null_mut(){
                Err(Error{msg:"Couldn't scale image"})
            }else{
                Ok(Bitmap{ptr: scaled_ptr})
            }
        }
	}

	pub fn is_transparent(&self) -> bool {
		unsafe{ ffi::FreeImage_IsTransparent(self.ptr) != 0 }
	}

	pub fn to_4bits(&self) -> Result<Bitmap, Error>{
		unsafe{
			let ptr = ffi::FreeImage_ConvertTo4Bits(self.ptr);
			if !ptr.is_null() {
				Ok(Bitmap{ptr})
			}else{
				Err(Error{msg: "Couldn't convert bitmap to 4 bits"})
			}
		}
	}

	pub fn to_8bits(&self) -> Result<Bitmap, Error>{
		unsafe{
			let ptr = ffi::FreeImage_ConvertTo8Bits(self.ptr);
			if !ptr.is_null() {
				Ok(Bitmap{ptr})
			}else{
				Err(Error{msg: "Couldn't convert bitmap to 8 bits"})
			}
		}
	}

	pub fn to_greyscale(&self) -> Result<Bitmap, Error>{
		unsafe{
			let ptr = ffi::FreeImage_ConvertToGreyscale(self.ptr);
			if !ptr.is_null() {
				Ok(Bitmap{ptr})
			}else{
				Err(Error{msg: "Couldn't convert bitmap to greyscale bits"})
			}
		}
	}

	pub fn to_16bits555(&self) -> Result<Bitmap, Error>{
		unsafe{
			let ptr = ffi::FreeImage_ConvertTo16Bits555(self.ptr);
			if !ptr.is_null() {
				Ok(Bitmap{ptr})
			}else{
				Err(Error{msg: "Couldn't convert bitmap to 16 bits 555"})
			}
		}
	}

	pub fn to_16bits565(&self) -> Result<Bitmap, Error>{
		unsafe{
			let ptr = ffi::FreeImage_ConvertTo16Bits565(self.ptr);
			if !ptr.is_null() {
				Ok(Bitmap{ptr})
			}else{
				Err(Error{msg: "Couldn't convert bitmap to 16 bits 565"})
			}
		}
	}

	pub fn to_24bits(&self) -> Result<Bitmap, Error>{
		unsafe{
			let ptr = ffi::FreeImage_ConvertTo24Bits(self.ptr);
			if !ptr.is_null() {
				Ok(Bitmap{ptr})
			}else{
				Err(Error{msg: "Couldn't convert bitmap to 24 bits"})
			}
		}
	}

	pub fn to_32bits(&self) -> Result<Bitmap, Error>{
		unsafe{
			let ptr = ffi::FreeImage_ConvertTo32Bits(self.ptr);
			if !ptr.is_null() {
				Ok(Bitmap{ptr})
			}else{
				Err(Error{msg: "Couldn't convert bitmap to 32 bits"})
			}
		}
	}

	pub fn to(&self, ty: Type) -> Result<Bitmap, Error>{
		unsafe{
			let ptr = ffi::FreeImage_ConvertToType(self.ptr, ty as u32, 1);
			if !ptr.is_null() {
				Ok(Bitmap{ptr})
			}else{
				Err(Error{msg: "Couldn't convert bitmap to selected type"})
			}
		}
	}

    pub fn red_mask(&self) -> u32 {
        unsafe { ffi::FreeImage_GetRedMask(self.ptr) }
    }

    pub fn green_mask(&self) -> u32 {
        unsafe { ffi::FreeImage_GetGreenMask(self.ptr) }
    }

    pub fn blue_mask(&self) -> u32 {
        unsafe { ffi::FreeImage_GetBlueMask(self.ptr) }
	}

	pub fn channel(&self, channel: ColorChannel) -> Option<Bitmap> {
		let ptr = unsafe { ffi::FreeImage_GetChannel(self.ptr, channel as i32) };
		if ptr != ptr::null_mut() {
			Some(Bitmap{ ptr })
		}else{
			None
		}
	}

	pub fn set_channel(&mut self, bitmap: &Bitmap, channel: ColorChannel) -> Result<(), Error> {
		if unsafe { ffi::FreeImage_SetChannel(self.ptr, bitmap.ptr, channel as i32) } != 0 {
			Ok(())
		}else{
			Err(Error{msg: "Couldn't set channel"})
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
