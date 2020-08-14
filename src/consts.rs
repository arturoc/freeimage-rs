use std::mem;
use crate::ffi;

#[repr(C)]
#[derive(Eq,PartialEq,Clone,Copy,Debug)]
pub enum Format {
	UNKNOWN	= -1,
	BMP		= 0,
	ICO		= 1,
	JPEG	= 2,
	JNG		= 3,
	KOALA	= 4,
	//LBM	 = 5,
	IFF		= 5,
	MNG		= 6,
	PBM		= 7,
	PBMRAW	= 8,
	PCD		= 9,
	PCX		= 10,
	PGM		= 11,
	PGMRAW	= 12,
	PNG		= 13,
	PPM		= 14,
	PPMRAW	= 15,
	RAS		= 16,
	TARGA	= 17,
	TIFF	= 18,
	WBMP	= 19,
	PSD		= 20,
	CUT		= 21,
	XBM		= 22,
	XPM		= 23,
	DDS		= 24,
	GIF		= 25,
	HDR		= 26,
	FAXG3   = 27,
	SGI		= 28,
	EXR		= 29,
	J2K		= 30,
	JP2		= 31,
	PFM		= 32,
	PICT	= 33,
	RAW		= 34
}

impl From<i32> for Format {
	fn from(n: i32) -> Format {
		unsafe{ mem::transmute(n) }
	}
}

#[repr(i32)]
#[derive(Eq,PartialEq,Clone,Copy,Debug)]
pub enum ColorChannel {
	RGB = ffi::FREE_IMAGE_COLOR_CHANNEL_FICC_RGB,
	RED = ffi::FREE_IMAGE_COLOR_CHANNEL_FICC_RED,
	GREEN = ffi::FREE_IMAGE_COLOR_CHANNEL_FICC_GREEN,
	BLUE = ffi::FREE_IMAGE_COLOR_CHANNEL_FICC_BLUE,
	ALPHA = ffi::FREE_IMAGE_COLOR_CHANNEL_FICC_ALPHA,
	BLACK = ffi::FREE_IMAGE_COLOR_CHANNEL_FICC_BLACK,
	REAL = ffi::FREE_IMAGE_COLOR_CHANNEL_FICC_REAL,
	IMAG = ffi::FREE_IMAGE_COLOR_CHANNEL_FICC_IMAG,
	MAG = ffi::FREE_IMAGE_COLOR_CHANNEL_FICC_MAG,
	PHASE = ffi::FREE_IMAGE_COLOR_CHANNEL_FICC_PHASE,
}

//pub enum SaveFlags{
pub static DEFAULT:i32 = 0; 		// Default option for all types.
pub static BMP_SAVE_RLE:i32=1; 	// Save with run length encoding.
pub static EXR_FLOAT:i32=0x0001; 		// Save data as float instead of as half (not recommended).
pub static EXR_NONE:i32=0x0002;  		// Save with no compression.
pub static EXR_ZIP:i32=0x0004;		// Save with zlib compression; in blocks of 16 scan lines.
pub static EXR_PIZ:i32=0x0008;		// Save with piz-based wavelet compression.
pub static EXR_PXR24:i32=0x0010;		// Save with lossy 24-bit float compression.
pub static EXR_B44:i32=0x0020;		// Save with lossy 44% float compression - goes to 22% when combined with EXR_LC.
pub static EXR_LC:i32=0x0040;			// Save images with one luminance and two chroma channels; rather than as RGB (lossy compression).
pub static JPEG_QUALITYSUPERB:i32=0x80;		// Save with superb quality (100:1).
pub static JPEG_QUALITYGOOD:i32=0x0100;		// Save with good quality (75:1).
pub static JPEG_QUALITYNORMAL:i32=0x0200;		// Save with normal quality (50:1).
pub static JPEG_QUALITYAVERAGE:i32=0x0400;	// Save with average quality (25:1).
pub static JPEG_QUALITYBAD:i32=0x0800;		// Save with bad quality (10:1).
pub static JPEG_PROGRESSIVE:i32=0x2000;		// Save as a progressive-JPEG (use | to combine with other save flags).
pub static JPEG_SUBSAMPLING_411:i32=0x1000;	// Save with high 4x1 chroma subsampling (4:1:1).
pub static JPEG_SUBSAMPLING_420:i32=0x4000;	// Save with medium 2x2 medium chroma (4:2:0).
pub static JPEG_SUBSAMPLING_422:i32=0x8000;	// Save with low 2x1 chroma subsampling (4:2:2).
pub static JPEG_SUBSAMPLING_444:i32=0x10000;	// Save with no chroma subsampling (4:4:4).
pub static PNG_Z_BEST_SPEED:i32=0x0001;		// Save using ZLib level 1 compression flag (default value is PNG_Z_DEFAULT_COMPRESSION).
pub static PNG_Z_DEFAULT_COMPRESSION:i32=0x0006;	// Save using ZLib level 6 compression flag (default recommended value).
pub static PNG_Z_BEST_COMPRESSION:i32=0x0009;		// save using ZLib level 9 compression flag (default value is PNG_Z_DEFAULT_COMPRESSION).
pub static PNG_Z_NO_COMPRESSION:i32=0x0100;		// Save without ZLib compression.
pub static PNG_INTERLACED:i32=0x0200;		// Save using Adam7 interlacing (use | to combine with other save flags).
pub static PNM_SAVE_ASCII:i32=1;		// If set the writer saves in ASCII format (i.e. P1; P2 or P3).
pub static TIFF_CMYK:i32=0x0001;			// Stores tags for separated CMYK (use | to combine with compression flags).
pub static TIFF_PACKBITS:i32=0x0100;		// Save using PACKBITS compression.
pub static TIFF_DEFLATE:i32=0x0200;		// Save using DEFLATE compression (a.k.a. ZLIB compression).
pub static TIFF_ADOBE_DEFLATE:i32=0x0400;	// Save using ADOBE DEFLATE compression.
pub static TIFF_NONE:i32=0x0800;			// Save without any compression.
pub static TIFF_CCITTFAX3:i32=0x1000;		// Save using CCITT Group 3 fax encoding.
pub static TIFF_CCITTFAX4:i32=0x2000;		// Save using CCITT Group 4 fax encoding.
pub static TIFF_LZW:i32=0x4000;			// Save using LZW compression.
pub static TIFF_JPEG:i32=0x8000;			// Save using JPEG compression.
//}
