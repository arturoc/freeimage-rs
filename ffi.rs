use std::libc::*;

// re-export constants
pub use consts::*;

pub struct Bitmap;

// C function bindings

extern "C stdcall" {
	pub fn FreeImage_Initialise( load_local_plugins_only: c_int );
	pub fn FreeImage_GetFileType( filename: *c_char, size: c_int ) -> FormatIdentifier;
	pub fn FreeImage_FIFSupportsReading( fif: FormatIdentifier ) -> c_int;
	pub fn FreeImage_Load( fif: FormatIdentifier, filename: *c_char, flags: c_int ) -> *Bitmap;
	pub fn FreeImage_Unload( dib: *Bitmap );
	pub fn FreeImage_GetWidth( dib: *Bitmap ) -> c_uint;
	pub fn FreeImage_GetHeight( dib: *Bitmap ) -> c_uint;
	pub fn FreeImage_GetPitch( dib: *Bitmap ) -> c_uint;
	pub fn FreeImage_GetBits( dib: *Bitmap ) -> *u8;
	pub fn FreeImage_GetScanLine( dib: *Bitmap, scanline: c_int ) -> *u8;
}
