    
use libc::c_int;
use libc::c_uint;
use libc::c_schar;
use libc::c_double;
use libc::c_long;
use libc::size_t;
use libc::c_longlong;
use libc::c_ulonglong;
use libc::c_ulong;
use libc::c_float;
use libc::c_ushort;
use libc::uint64_t;
use libc::int64_t;
use libc::int32_t;
use libc::uint32_t;
use libc::uint16_t;
use libc::uint8_t;
use libc::wchar_t;
use libc::c_void;
use freeimage_consts;
use core::intrinsics::transmute;

pub type Struct__IO_FILE = c_void;
//pub type FILE = Struct__IO_FILE;
pub type __FILE = Struct__IO_FILE;
/*pub type ptrdiff_t = c_long;
pub type size_t = c_ulong;
pub type wchar_t = c_int;*/
pub type wint_t = c_uint;
pub struct Union_Unnamed1 {
    data: [u32; 1],
}
impl Union_Unnamed1 {
    pub fn __wch(&mut self) -> *const c_uint {
        unsafe { transmute(self) }
    }
    pub fn __wchb(&mut self) -> *const [c_schar; 4] {
        unsafe { transmute(self) }
    }
}
pub struct __mbstate_t {
    __count: c_int,
    __value: Union_Unnamed1,
}
pub type mbstate_t = __mbstate_t;
pub type Struct_tm = c_void;
pub struct Struct___locale_struct {
    __locales: [*const Struct___locale_data; 13],
    __ctype_b: *const c_ushort,
    __ctype_tolower: *const c_int,
    __ctype_toupper: *const c_int,
    __names: [*const c_schar; 13],
}
pub type Struct___locale_data = c_void;
pub type __locale_t = *const Struct___locale_struct;
pub type locale_t = __locale_t;
pub type FIBITMAP = Struct_FIBITMAP;
pub struct Struct_FIBITMAP {
    data: *const c_void,
}
pub type FIMULTIBITMAP = Struct_FIMULTIBITMAP;
pub struct Struct_FIMULTIBITMAP {
    data: *const c_void,
}
/*pub type int8_t = c_schar;
pub type int16_t = c_short;
pub type int32_t = c_int;
pub type int64_t = c_long;
pub type uint8_t = c_uchar;
pub type uint16_t = c_ushort;
pub type uint32_t = c_uint;
pub type uint64_t = c_ulong;
pub type int_least8_t = c_schar;
pub type int_least16_t = c_short;
pub type int_least32_t = c_int;
pub type int_least64_t = c_long;
pub type uint_least8_t = c_uchar;
pub type uint_least16_t = c_ushort;
pub type uint_least32_t = c_uint;
pub type uint_least64_t = c_ulong;
pub type int_fast8_t = c_schar;
pub type int_fast16_t = c_long;
pub type int_fast32_t = c_long;
pub type int_fast64_t = c_long;
pub type uint_fast8_t = c_uchar;
pub type uint_fast16_t = c_ulong;
pub type uint_fast32_t = c_ulong;
pub type uint_fast64_t = c_ulong;
pub type intptr_t = c_long;
pub type uintptr_t = c_ulong;*/
pub type intmax_t = c_long;
pub type uintmax_t = c_ulong;
pub type __gwchar_t = c_int;
pub struct imaxdiv_t {
    quot: c_long,
    rem: c_long,
}
pub type BOOL = int32_t;
pub type BYTE = uint8_t;
pub type WORD = uint16_t;
pub type DWORD = uint32_t;
pub type LONG = int32_t;
pub type FIINT64 = int64_t;
pub type FIUINT64 = uint64_t;
pub struct Struct_tagRGBQUAD {
    rgbBlue: BYTE,
    rgbGreen: BYTE,
    rgbRed: BYTE,
    rgbReserved: BYTE,
}
pub type RGBQUAD = Struct_tagRGBQUAD;
pub struct Struct_tagRGBTRIPLE {
    rgbtBlue: BYTE,
    rgbtGreen: BYTE,
    rgbtRed: BYTE,
}
pub type RGBTRIPLE = Struct_tagRGBTRIPLE;
pub struct Struct_tagBITMAPINFOHEADER {
    biSize: DWORD,
    biWidth: LONG,
    biHeight: LONG,
    biPlanes: WORD,
    biBitCount: WORD,
    biCompression: DWORD,
    biSizeImage: DWORD,
    biXPelsPerMeter: LONG,
    biYPelsPerMeter: LONG,
    biClrUsed: DWORD,
    biClrImportant: DWORD,
}
pub type BITMAPINFOHEADER = Struct_tagBITMAPINFOHEADER;
pub type PBITMAPINFOHEADER = *const Struct_tagBITMAPINFOHEADER;
pub struct Struct_tagBITMAPINFO {
    bmiHeader: BITMAPINFOHEADER,
    bmiColors: [RGBQUAD; 1],
}
pub type BITMAPINFO = Struct_tagBITMAPINFO;
pub type PBITMAPINFO = *const Struct_tagBITMAPINFO;
pub struct Struct_tagFIRGB16 {
    red: WORD,
    green: WORD,
    blue: WORD,
}
pub type FIRGB16 = Struct_tagFIRGB16;
pub struct Struct_tagFIRGBA16 {
    red: WORD,
    green: WORD,
    blue: WORD,
    alpha: WORD,
}
pub type FIRGBA16 = Struct_tagFIRGBA16;
pub struct Struct_tagFIRGBF {
    red: c_float,
    green: c_float,
    blue: c_float,
}
pub type FIRGBF = Struct_tagFIRGBF;
pub struct Struct_tagFIRGBAF {
    red: c_float,
    green: c_float,
    blue: c_float,
    alpha: c_float,
}
pub type FIRGBAF = Struct_tagFIRGBAF;
pub struct Struct_tagFICOMPLEX {
    r: c_double,
    i: c_double,
}
pub type FICOMPLEX = Struct_tagFICOMPLEX;
pub type FIICCPROFILE = Struct_FIICCPROFILE;
pub struct Struct_FIICCPROFILE {
    flags: WORD,
    size: DWORD,
    data: *const c_void,
}
pub type FREE_IMAGE_FORMAT = freeimage_consts::FormatIdentifier;
pub type FREE_IMAGE_TYPE = c_int;
pub type Enum_FREE_IMAGE_TYPE = c_uint;
pub static FIT_UNKNOWN: c_uint = 0;
pub static FIT_BITMAP: c_uint = 1;
pub static FIT_UINT16: c_uint = 2;
pub static FIT_INT16: c_uint = 3;
pub static FIT_UINT32: c_uint = 4;
pub static FIT_INT32: c_uint = 5;
pub static FIT_FLOAT: c_uint = 6;
pub static FIT_DOUBLE: c_uint = 7;
pub static FIT_COMPLEX: c_uint = 8;
pub static FIT_RGB16: c_uint = 9;
pub static FIT_RGBA16: c_uint = 10;
pub static FIT_RGBF: c_uint = 11;
pub static FIT_RGBAF: c_uint = 12;
pub type FREE_IMAGE_COLOR_TYPE = c_int;
pub type Enum_FREE_IMAGE_COLOR_TYPE = c_uint;
pub static FIC_MINISWHITE: c_uint = 0;
pub static FIC_MINISBLACK: c_uint = 1;
pub static FIC_RGB: c_uint = 2;
pub static FIC_PALETTE: c_uint = 3;
pub static FIC_RGBALPHA: c_uint = 4;
pub static FIC_CMYK: c_uint = 5;
pub type FREE_IMAGE_QUANTIZE = c_int;
pub type Enum_FREE_IMAGE_QUANTIZE = c_uint;
pub static FIQ_WUQUANT: c_uint = 0;
pub static FIQ_NNQUANT: c_uint = 1;
pub type FREE_IMAGE_DITHER = c_int;
pub type Enum_FREE_IMAGE_DITHER = c_uint;
pub static FID_FS: c_uint = 0;
pub static FID_BAYER4x4: c_uint = 1;
pub static FID_BAYER8x8: c_uint = 2;
pub static FID_CLUSTER6x6: c_uint = 3;
pub static FID_CLUSTER8x8: c_uint = 4;
pub static FID_CLUSTER16x16: c_uint = 5;
pub static FID_BAYER16x16: c_uint = 6;
pub type FREE_IMAGE_JPEG_OPERATION = c_int;
pub type Enum_FREE_IMAGE_JPEG_OPERATION = c_uint;
pub static FIJPEG_OP_NONE: c_uint = 0;
pub static FIJPEG_OP_FLIP_H: c_uint = 1;
pub static FIJPEG_OP_FLIP_V: c_uint = 2;
pub static FIJPEG_OP_TRANSPOSE: c_uint = 3;
pub static FIJPEG_OP_TRANSVERSE: c_uint = 4;
pub static FIJPEG_OP_ROTATE_90: c_uint = 5;
pub static FIJPEG_OP_ROTATE_180: c_uint = 6;
pub static FIJPEG_OP_ROTATE_270: c_uint = 7;
pub type FREE_IMAGE_TMO = c_int;
pub type Enum_FREE_IMAGE_TMO = c_uint;
pub static FITMO_DRAGO03: c_uint = 0;
pub static FITMO_REINHARD05: c_uint = 1;
pub static FITMO_FATTAL02: c_uint = 2;
pub type FREE_IMAGE_FILTER = c_int;
pub type Enum_FREE_IMAGE_FILTER = c_uint;
pub static FILTER_BOX: c_uint = 0;
pub static FILTER_BICUBIC: c_uint = 1;
pub static FILTER_BILINEAR: c_uint = 2;
pub static FILTER_BSPLINE: c_uint = 3;
pub static FILTER_CATMULLROM: c_uint = 4;
pub static FILTER_LANCZOS3: c_uint = 5;
pub type FREE_IMAGE_COLOR_CHANNEL = c_int;
pub type Enum_FREE_IMAGE_COLOR_CHANNEL = c_uint;
pub static FICC_RGB: c_uint = 0;
pub static FICC_RED: c_uint = 1;
pub static FICC_GREEN: c_uint = 2;
pub static FICC_BLUE: c_uint = 3;
pub static FICC_ALPHA: c_uint = 4;
pub static FICC_BLACK: c_uint = 5;
pub static FICC_REAL: c_uint = 6;
pub static FICC_IMAG: c_uint = 7;
pub static FICC_MAG: c_uint = 8;
pub static FICC_PHASE: c_uint = 9;
pub type FREE_IMAGE_MDTYPE = c_int;
pub type Enum_FREE_IMAGE_MDTYPE = c_uint;
pub static FIDT_NOTYPE: c_uint = 0;
pub static FIDT_BYTE: c_uint = 1;
pub static FIDT_ASCII: c_uint = 2;
pub static FIDT_SHORT: c_uint = 3;
pub static FIDT_LONG: c_uint = 4;
pub static FIDT_RATIONAL: c_uint = 5;
pub static FIDT_SBYTE: c_uint = 6;
pub static FIDT_UNDEFINED: c_uint = 7;
pub static FIDT_SSHORT: c_uint = 8;
pub static FIDT_SLONG: c_uint = 9;
pub static FIDT_SRATIONAL: c_uint = 10;
pub static FIDT_FLOAT: c_uint = 11;
pub static FIDT_DOUBLE: c_uint = 12;
pub static FIDT_IFD: c_uint = 13;
pub static FIDT_PALETTE: c_uint = 14;
pub static FIDT_LONG8: c_uint = 16;
pub static FIDT_SLONG8: c_uint = 17;
pub static FIDT_IFD8: c_uint = 18;
pub type FREE_IMAGE_MDMODEL = c_int;
pub type Enum_FREE_IMAGE_MDMODEL = c_int;
pub static FIMD_NODATA: c_int = -1;
pub static FIMD_COMMENTS: c_int = 0;
pub static FIMD_EXIF_MAIN: c_int = 1;
pub static FIMD_EXIF_EXIF: c_int = 2;
pub static FIMD_EXIF_GPS: c_int = 3;
pub static FIMD_EXIF_MAKERNOTE: c_int = 4;
pub static FIMD_EXIF_INTEROP: c_int = 5;
pub static FIMD_IPTC: c_int = 6;
pub static FIMD_XMP: c_int = 7;
pub static FIMD_GEOTIFF: c_int = 8;
pub static FIMD_ANIMATION: c_int = 9;
pub static FIMD_CUSTOM: c_int = 10;
pub static FIMD_EXIF_RAW: c_int = 11;
pub type FIMETADATA = Struct_FIMETADATA;
pub struct Struct_FIMETADATA {
    data: *const c_void,
}
pub type FITAG = Struct_FITAG;
pub struct Struct_FITAG {
    data: *const c_void,
}
pub type fi_handle = *const c_void;
pub type FI_ReadProc =
    ::std::option::Option<extern "C" fn
                              (arg1: *const c_void, arg2: c_uint, arg3: c_uint,
                               arg4: fi_handle) -> c_uint>;
pub type FI_WriteProc =
    ::std::option::Option<extern "C" fn
                              (arg1: *const c_void, arg2: c_uint, arg3: c_uint,
                               arg4: fi_handle) -> c_uint>;
pub type FI_SeekProc =
    ::std::option::Option<extern "C" fn
                              (arg1: fi_handle, arg2: c_long, arg3: c_int)
                              -> c_int>;
pub type FI_TellProc =
    ::std::option::Option<extern "C" fn(arg1: fi_handle) -> c_long>;
pub type FreeImageIO = Struct_FreeImageIO;
pub struct Struct_FreeImageIO {
    read_proc: FI_ReadProc,
    write_proc: FI_WriteProc,
    seek_proc: FI_SeekProc,
    tell_proc: FI_TellProc,
}
pub type FIMEMORY = Struct_FIMEMORY;
pub struct Struct_FIMEMORY {
    data: *const c_void,
}
pub type FI_FormatProc = ::std::option::Option<extern "C" fn() -> *const c_schar>;
pub type FI_DescriptionProc =
    ::std::option::Option<extern "C" fn() -> *const c_schar>;
pub type FI_ExtensionListProc =
    ::std::option::Option<extern "C" fn() -> *const c_schar>;
pub type FI_RegExprProc = ::std::option::Option<extern "C" fn() -> *const c_schar>;
pub type FI_OpenProc =
    ::std::option::Option<extern "C" fn
                              (arg1: *const FreeImageIO, arg2: fi_handle,
                               arg3: BOOL) -> *const c_void>;
pub type FI_CloseProc =
    ::std::option::Option<extern "C" fn
                              (arg1: *const FreeImageIO, arg2: fi_handle,
                               arg3: *const c_void)>;
pub type FI_PageCountProc =
    ::std::option::Option<extern "C" fn
                              (arg1: *const FreeImageIO, arg2: fi_handle,
                               arg3: *const c_void) -> c_int>;
pub type FI_PageCapabilityProc =
    ::std::option::Option<extern "C" fn
                              (arg1: *const FreeImageIO, arg2: fi_handle,
                               arg3: *const c_void) -> c_int>;
pub type FI_LoadProc =
    ::std::option::Option<extern "C" fn
                              (arg1: *const FreeImageIO, arg2: fi_handle,
                               arg3: c_int, arg4: c_int, arg5: *const c_void)
                              -> *const FIBITMAP>;
pub type FI_SaveProc =
    ::std::option::Option<extern "C" fn
                              (arg1: *const FreeImageIO, arg2: *const FIBITMAP,
                               arg3: fi_handle, arg4: c_int, arg5: c_int,
                               arg6: *const c_void) -> BOOL>;
pub type FI_ValidateProc =
    ::std::option::Option<extern "C" fn
                              (arg1: *const FreeImageIO, arg2: fi_handle)
                              -> BOOL>;
pub type FI_MimeProc = ::std::option::Option<extern "C" fn() -> *const c_schar>;
pub type FI_SupportsExportBPPProc =
    ::std::option::Option<extern "C" fn(arg1: c_int) -> BOOL>;
pub type FI_SupportsExportTypeProc =
    ::std::option::Option<extern "C" fn(arg1: FREE_IMAGE_TYPE) -> BOOL>;
pub type FI_SupportsICCProfilesProc =
    ::std::option::Option<extern "C" fn() -> BOOL>;
pub type FI_SupportsNoPixelsProc =
    ::std::option::Option<extern "C" fn() -> BOOL>;
pub type Plugin = Struct_Plugin;
pub struct Struct_Plugin {
    format_proc: FI_FormatProc,
    description_proc: FI_DescriptionProc,
    extension_proc: FI_ExtensionListProc,
    regexpr_proc: FI_RegExprProc,
    open_proc: FI_OpenProc,
    close_proc: FI_CloseProc,
    pagecount_proc: FI_PageCountProc,
    pagecapability_proc: FI_PageCapabilityProc,
    load_proc: FI_LoadProc,
    save_proc: FI_SaveProc,
    validate_proc: FI_ValidateProc,
    mime_proc: FI_MimeProc,
    supports_export_bpp_proc: FI_SupportsExportBPPProc,
    supports_export_type_proc: FI_SupportsExportTypeProc,
    supports_icc_profiles_proc: FI_SupportsICCProfilesProc,
    supports_no_pixels_proc: FI_SupportsNoPixelsProc,
}
pub type FI_InitProc =
    ::std::option::Option<extern "C" fn(arg1: *const Plugin, arg2: c_int)>;
pub type FreeImage_OutputMessageFunction =
    ::std::option::Option<extern "C" fn
                              (arg1: FREE_IMAGE_FORMAT, arg2: *const c_schar)>;
pub type FreeImage_OutputMessageFunctionStdCall =
    ::std::option::Option<extern "C" fn
                              (arg1: FREE_IMAGE_FORMAT, arg2: *const c_schar)>;
extern "C" {
    pub fn wcscpy(__dest: *const wchar_t, __src: *const wchar_t) -> *const wchar_t;
    pub fn wcsncpy(__dest: *const wchar_t, __src: *const wchar_t, __n: size_t) ->
     *const wchar_t;
    pub fn wcscat(__dest: *const wchar_t, __src: *const wchar_t) -> *const wchar_t;
    pub fn wcsncat(__dest: *const wchar_t, __src: *const wchar_t, __n: size_t) ->
     *const wchar_t;
    pub fn wcscmp(__s1: *const wchar_t, __s2: *const wchar_t) -> c_int;
    pub fn wcsncmp(__s1: *const wchar_t, __s2: *const wchar_t, __n: size_t) -> c_int;
    pub fn wcscasecmp(__s1: *const wchar_t, __s2: *const wchar_t) -> c_int;
    pub fn wcsncasecmp(__s1: *const wchar_t, __s2: *const wchar_t, __n: size_t) -> c_int;
    pub fn wcscasecmp_l(__s1: *const wchar_t, __s2: *const wchar_t, __loc: __locale_t) ->
     c_int;
    pub fn wcsncasecmp_l(__s1: *const wchar_t, __s2: *const wchar_t, __n: size_t,
                         __loc: __locale_t) -> c_int;
    pub fn wcscoll(__s1: *const wchar_t, __s2: *const wchar_t) -> c_int;
    pub fn wcsxfrm(__s1: *const wchar_t, __s2: *const wchar_t, __n: size_t) -> size_t;
    pub fn wcscoll_l(__s1: *const wchar_t, __s2: *const wchar_t, __loc: __locale_t) ->
     c_int;
    pub fn wcsxfrm_l(__s1: *const wchar_t, __s2: *const wchar_t, __n: size_t,
                     __loc: __locale_t) -> size_t;
    pub fn wcsdup(__s: *const wchar_t) -> *const wchar_t;
    pub fn wcschr(__wcs: *const wchar_t, __wc: wchar_t) -> *const wchar_t;
    pub fn wcsrchr(__wcs: *const wchar_t, __wc: wchar_t) -> *const wchar_t;
    pub fn wcscspn(__wcs: *const wchar_t, __reject: *const wchar_t) -> size_t;
    pub fn wcsspn(__wcs: *const wchar_t, __accept: *const wchar_t) -> size_t;
    pub fn wcspbrk(__wcs: *const wchar_t, __accept: *const wchar_t) -> *const wchar_t;
    pub fn wcsstr(__haystack: *const wchar_t, __needle: *const wchar_t) -> *const wchar_t;
    pub fn wcstok(__s: *const wchar_t, __delim: *const wchar_t,
                  __ptr: *const *const wchar_t) -> *const wchar_t;
    pub fn wcslen(__s: *const wchar_t) -> size_t;
    pub fn wcsnlen(__s: *const wchar_t, __maxlen: size_t) -> size_t;
    pub fn wmemchr(__s: *const wchar_t, __c: wchar_t, __n: size_t) -> *const wchar_t;
    pub fn wmemcmp(__s1: *const wchar_t, __s2: *const wchar_t, __n: size_t) -> c_int;
    pub fn wmemcpy(__s1: *const wchar_t, __s2: *const wchar_t, __n: size_t) ->
     *const wchar_t;
    pub fn wmemmove(__s1: *const wchar_t, __s2: *const wchar_t, __n: size_t) ->
     *const wchar_t;
    pub fn wmemset(__s: *const wchar_t, __c: wchar_t, __n: size_t) ->
     *const wchar_t;
    pub fn btowc(__c: c_int) -> wint_t;
    pub fn wctob(__c: wint_t) -> c_int;
    pub fn mbsinit(__ps: *const mbstate_t) -> c_int;
    pub fn mbrtowc(__pwc: *const wchar_t, __s: *const c_schar, __n: size_t,
                   __p: *const mbstate_t) -> size_t;
    pub fn wcrtomb(__s: *const c_schar, __wc: wchar_t, __ps: *const mbstate_t) ->
     size_t;
    pub fn __mbrlen(__s: *const c_schar, __n: size_t, __ps: *const mbstate_t) ->
     size_t;
    pub fn mbrlen(__s: *const c_schar, __n: size_t, __ps: *const mbstate_t) -> size_t;
    pub fn mbsrtowcs(__dst: *const wchar_t, __src: *const *const c_schar, __len: size_t,
                     __ps: *const mbstate_t) -> size_t;
    pub fn wcsrtombs(__dst: *const c_schar, __src: *const *const wchar_t, __len: size_t,
                     __ps: *const mbstate_t) -> size_t;
    pub fn mbsnrtowcs(__dst: *const wchar_t, __src: *const *const c_schar,
                      __nmc: size_t, __len: size_t, __ps: *const mbstate_t) ->
     size_t;
    pub fn wcsnrtombs(__dst: *const c_schar, __src: *const *const wchar_t,
                      __nwc: size_t, __len: size_t, __ps: *const mbstate_t) ->
     size_t;
    pub fn wcstod(__nptr: *const wchar_t, __endptr: *const *const wchar_t) -> c_double;
    pub fn wcstof(__nptr: *const wchar_t, __endptr: *const *const wchar_t) -> c_float;
    pub fn wcstold(__nptr: *const wchar_t, __endptr: *const *const wchar_t) -> c_double;
    pub fn wcstol(__nptr: *const wchar_t, __endptr: *const *const wchar_t,
                  __base: c_int) -> c_long;
    pub fn wcstoul(__nptr: *const wchar_t, __endptr: *const *const wchar_t,
                   __base: c_int) -> c_ulong;
    pub fn wcstoll(__nptr: *const wchar_t, __endptr: *const *const wchar_t,
                   __base: c_int) -> c_longlong;
    pub fn wcstoull(__nptr: *const wchar_t, __endptr: *const *const wchar_t,
                    __base: c_int) -> c_ulonglong;
    pub fn wcpcpy(__dest: *const wchar_t, __src: *const wchar_t) -> *const wchar_t;
    pub fn wcpncpy(__dest: *const wchar_t, __src: *const wchar_t, __n: size_t) ->
     *const wchar_t;
    pub fn open_wmemstream(__bufloc: *const *const wchar_t,
                           __sizeloc: *const size_t) -> *const __FILE;
    pub fn fwide(__fp: *const __FILE, __mode: c_int) -> c_int;
    pub fn fwprintf(__stream: *const __FILE, __format: *const wchar_t, ...) -> c_int;
    pub fn wprintf(__format: *const wchar_t, ...) -> c_int;
    pub fn swprintf(__s: *const wchar_t, __n: size_t, __format: *const wchar_t, ...)
     -> c_int;
    pub fn fgetwc(__stream: *const __FILE) -> wint_t;
    pub fn getwc(__stream: *const __FILE) -> wint_t;
    pub fn getwchar() -> wint_t;
    pub fn fputwc(__wc: wchar_t, __stream: *const __FILE) -> wint_t;
    pub fn putwc(__wc: wchar_t, __stream: *const __FILE) -> wint_t;
    pub fn putwchar(__wc: wchar_t) -> wint_t;
    pub fn fgetws(__ws: *const wchar_t, __n: c_int, __stream: *const __FILE) ->
     *const wchar_t;
    pub fn fputws(__ws: *const wchar_t, __stream: *const __FILE) -> c_int;
    pub fn ungetwc(__wc: wint_t, __stream: *const __FILE) -> wint_t;
    pub fn wcsftime(__s: *const wchar_t, __maxsize: size_t, __format: *const wchar_t,
                    __tp: *const Struct_tm) -> size_t;
    pub fn imaxabs(__n: intmax_t) -> intmax_t;
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
    pub fn strtoimax(__nptr: *const c_schar, __endptr: *const *const c_schar,
                     __base: c_int) -> intmax_t;
    pub fn strtoumax(__nptr: *const c_schar, __endptr: *const *const c_schar,
                     __base: c_int) -> uintmax_t;
    pub fn wcstoimax(__nptr: *const __gwchar_t, __endptr: *const *const __gwchar_t,
                     __base: c_int) -> intmax_t;
    pub fn wcstoumax(__nptr: *const __gwchar_t, __endptr: *const *const __gwchar_t,
                     __base: c_int) -> uintmax_t;
    pub fn FreeImage_Initialise(load_local_plugins_only: BOOL);
    pub fn FreeImage_DeInitialise();
    pub fn FreeImage_GetVersion() -> *const c_schar;
    pub fn FreeImage_GetCopyrightMessage() -> *const c_schar;
    pub fn FreeImage_SetOutputMessageStdCall(omf:
                                                 FreeImage_OutputMessageFunctionStdCall);
    pub fn FreeImage_SetOutputMessage(omf: FreeImage_OutputMessageFunction);
    pub fn FreeImage_OutputMessageProc(fif: c_int, fmt: *const c_schar, ...);
    pub fn FreeImage_Allocate(width: c_int, height: c_int, bpp: c_int,
                              red_mask: c_uint, green_mask: c_uint,
                              blue_mask: c_uint) -> *const FIBITMAP;
    pub fn FreeImage_AllocateT(_type: FREE_IMAGE_TYPE, width: c_int,
                               height: c_int, bpp: c_int, red_mask: c_uint,
                               green_mask: c_uint, blue_mask: c_uint) ->
     *const FIBITMAP;
    pub fn FreeImage_Clone(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_Unload(dib: *const FIBITMAP);
    pub fn FreeImage_HasPixels(dib: *const FIBITMAP) -> BOOL;
    pub fn FreeImage_Load(fif: FREE_IMAGE_FORMAT, filename: *const c_schar,
                          flags: c_int) -> *const FIBITMAP;
    pub fn FreeImage_LoadU(fif: FREE_IMAGE_FORMAT, filename: *const wchar_t,
                           flags: c_int) -> *const FIBITMAP;
    pub fn FreeImage_LoadFromHandle(fif: FREE_IMAGE_FORMAT,
                                    io: *const FreeImageIO, handle: fi_handle,
                                    flags: c_int) -> *const FIBITMAP;
    pub fn FreeImage_Save(fif: FREE_IMAGE_FORMAT, dib: *const FIBITMAP,
                          filename: *const c_schar, flags: c_int) -> BOOL;
    pub fn FreeImage_SaveU(fif: FREE_IMAGE_FORMAT, dib: *const FIBITMAP,
                           filename: *const wchar_t, flags: c_int) -> BOOL;
    pub fn FreeImage_SaveToHandle(fif: FREE_IMAGE_FORMAT, dib: *const FIBITMAP,
                                  io: *const FreeImageIO, handle: fi_handle,
                                  flags: c_int) -> BOOL;
    pub fn FreeImage_OpenMemory(data: *const BYTE, size_in_bytes: DWORD) ->
     *const FIMEMORY;
    pub fn FreeImage_CloseMemory(stream: *const FIMEMORY);
    pub fn FreeImage_LoadFromMemory(fif: FREE_IMAGE_FORMAT,
                                    stream: *const FIMEMORY, flags: c_int) ->
     *const FIBITMAP;
    pub fn FreeImage_SaveToMemory(fif: FREE_IMAGE_FORMAT, dib: *const FIBITMAP,
                                  stream: *const FIMEMORY, flags: c_int) ->
     BOOL;
    pub fn FreeImage_TellMemory(stream: *const FIMEMORY) -> c_long;
    pub fn FreeImage_SeekMemory(stream: *const FIMEMORY, offset: c_long,
                                origin: c_int) -> BOOL;
    pub fn FreeImage_AcquireMemory(stream: *const FIMEMORY,
                                   data: *const *const BYTE,
                                   size_in_bytes: *const DWORD) -> BOOL;
    pub fn FreeImage_ReadMemory(buffer: *const c_void, size: c_uint,
                                count: c_uint, stream: *const FIMEMORY) ->
     c_uint;
    pub fn FreeImage_WriteMemory(buffer: *const c_void, size: c_uint, count: c_uint,
                                 stream: *const FIMEMORY) -> c_uint;
    pub fn FreeImage_LoadMultiBitmapFromMemory(fif: FREE_IMAGE_FORMAT,
                                               stream: *const FIMEMORY,
                                               flags: c_int) ->
     *const FIMULTIBITMAP;
    pub fn FreeImage_SaveMultiBitmapToMemory(fif: FREE_IMAGE_FORMAT,
                                             bitmap: *const FIMULTIBITMAP,
                                             stream: *const FIMEMORY,
                                             flags: c_int) -> BOOL;
    pub fn FreeImage_RegisterLocalPlugin(proc_address: FI_InitProc,
                                         format: *const c_schar,
                                         description: *const c_schar,
                                         extension: *const c_schar,
                                         regexpr: *const c_schar) ->
     FREE_IMAGE_FORMAT;
    pub fn FreeImage_RegisterExternalPlugin(path: *const c_schar, format: *const c_schar,
                                            description: *const c_schar,
                                            extension: *const c_schar,
                                            regexpr: *const c_schar) ->
     FREE_IMAGE_FORMAT;
    pub fn FreeImage_GetFIFCount() -> c_int;
    pub fn FreeImage_SetPluginEnabled(fif: FREE_IMAGE_FORMAT, enable: BOOL) ->
     c_int;
    pub fn FreeImage_IsPluginEnabled(fif: FREE_IMAGE_FORMAT) -> c_int;
    pub fn FreeImage_GetFIFFromFormat(format: *const c_schar) -> FREE_IMAGE_FORMAT;
    pub fn FreeImage_GetFIFFromMime(mime: *const c_schar) -> FREE_IMAGE_FORMAT;
    pub fn FreeImage_GetFormatFromFIF(fif: FREE_IMAGE_FORMAT) -> *const c_schar;
    pub fn FreeImage_GetFIFExtensionList(fif: FREE_IMAGE_FORMAT) -> *const c_schar;
    pub fn FreeImage_GetFIFDescription(fif: FREE_IMAGE_FORMAT) -> *const c_schar;
    pub fn FreeImage_GetFIFRegExpr(fif: FREE_IMAGE_FORMAT) -> *const c_schar;
    pub fn FreeImage_GetFIFMimeType(fif: FREE_IMAGE_FORMAT) -> *const c_schar;
    pub fn FreeImage_GetFIFFromFilename(filename: *const c_schar) ->
     FREE_IMAGE_FORMAT;
    pub fn FreeImage_GetFIFFromFilenameU(filename: *const wchar_t) ->
     FREE_IMAGE_FORMAT;
    pub fn FreeImage_FIFSupportsReading(fif: FREE_IMAGE_FORMAT) -> BOOL;
    pub fn FreeImage_FIFSupportsWriting(fif: FREE_IMAGE_FORMAT) -> BOOL;
    pub fn FreeImage_FIFSupportsExportBPP(fif: FREE_IMAGE_FORMAT, bpp: c_int)
     -> BOOL;
    pub fn FreeImage_FIFSupportsExportType(fif: FREE_IMAGE_FORMAT,
                                           _type: FREE_IMAGE_TYPE) -> BOOL;
    pub fn FreeImage_FIFSupportsICCProfiles(fif: FREE_IMAGE_FORMAT) -> BOOL;
    pub fn FreeImage_FIFSupportsNoPixels(fif: FREE_IMAGE_FORMAT) -> BOOL;
    pub fn FreeImage_OpenMultiBitmap(fif: FREE_IMAGE_FORMAT,
                                     filename: *const c_schar, create_new: BOOL,
                                     read_only: BOOL,
                                     keep_cache_in_memory: BOOL, flags: c_int)
     -> *const FIMULTIBITMAP;
    pub fn FreeImage_OpenMultiBitmapFromHandle(fif: FREE_IMAGE_FORMAT,
                                               io: *const FreeImageIO,
                                               handle: fi_handle,
                                               flags: c_int) ->
     *const FIMULTIBITMAP;
    pub fn FreeImage_SaveMultiBitmapToHandle(fif: FREE_IMAGE_FORMAT,
                                             bitmap: *const FIMULTIBITMAP,
                                             io: *const FreeImageIO,
                                             handle: fi_handle, flags: c_int)
     -> BOOL;
    pub fn FreeImage_CloseMultiBitmap(bitmap: *const FIMULTIBITMAP,
                                      flags: c_int) -> BOOL;
    pub fn FreeImage_GetPageCount(bitmap: *const FIMULTIBITMAP) -> c_int;
    pub fn FreeImage_AppendPage(bitmap: *const FIMULTIBITMAP,
                                data: *const FIBITMAP);
    pub fn FreeImage_InsertPage(bitmap: *const FIMULTIBITMAP, page: c_int,
                                data: *const FIBITMAP);
    pub fn FreeImage_DeletePage(bitmap: *const FIMULTIBITMAP, page: c_int);
    pub fn FreeImage_LockPage(bitmap: *const FIMULTIBITMAP, page: c_int) ->
     *const FIBITMAP;
    pub fn FreeImage_UnlockPage(bitmap: *const FIMULTIBITMAP,
                                data: *const FIBITMAP, changed: BOOL);
    pub fn FreeImage_MovePage(bitmap: *const FIMULTIBITMAP, target: c_int,
                              source: c_int) -> BOOL;
    pub fn FreeImage_GetLockedPageNumbers(bitmap: *const FIMULTIBITMAP,
                                          pages: *const c_int,
                                          count: *const c_int) -> BOOL;
    pub fn FreeImage_GetFileType(filename: *const c_schar, size: c_int) ->
     FREE_IMAGE_FORMAT;
    pub fn FreeImage_GetFileTypeU(filename: *const wchar_t, size: c_int) ->
     FREE_IMAGE_FORMAT;
    pub fn FreeImage_GetFileTypeFromHandle(io: *const FreeImageIO,
                                           handle: fi_handle, size: c_int) ->
     FREE_IMAGE_FORMAT;
    pub fn FreeImage_GetFileTypeFromMemory(stream: *const FIMEMORY, size: c_int)
     -> FREE_IMAGE_FORMAT;
    pub fn FreeImage_GetImageType(dib: *const FIBITMAP) -> FREE_IMAGE_TYPE;
    pub fn FreeImage_IsLittleEndian() -> BOOL;
    pub fn FreeImage_LookupX11Color(szColor: *const c_schar, nRed: *const BYTE,
                                    nGreen: *const BYTE, nBlue: *const BYTE) ->
     BOOL;
    pub fn FreeImage_LookupSVGColor(szColor: *const c_schar, nRed: *const BYTE,
                                    nGreen: *const BYTE, nBlue: *const BYTE) ->
     BOOL;
    pub fn FreeImage_GetBits(dib: *const FIBITMAP) -> *const BYTE;
    pub fn FreeImage_GetScanLine(dib: *const FIBITMAP, scanline: c_int) ->
     *const BYTE;
    pub fn FreeImage_GetPixelIndex(dib: *const FIBITMAP, x: c_uint, y: c_uint,
                                   value: *const BYTE) -> BOOL;
    pub fn FreeImage_GetPixelColor(dib: *const FIBITMAP, x: c_uint, y: c_uint,
                                   value: *const RGBQUAD) -> BOOL;
    pub fn FreeImage_SetPixelIndex(dib: *const FIBITMAP, x: c_uint, y: c_uint,
                                   value: *const BYTE) -> BOOL;
    pub fn FreeImage_SetPixelColor(dib: *const FIBITMAP, x: c_uint, y: c_uint,
                                   value: *const RGBQUAD) -> BOOL;
    pub fn FreeImage_GetColorsUsed(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetBPP(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetWidth(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetHeight(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetLine(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetPitch(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetDIBSize(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetPalette(dib: *const FIBITMAP) -> *const RGBQUAD;
    pub fn FreeImage_GetDotsPerMeterX(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetDotsPerMeterY(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_SetDotsPerMeterX(dib: *const FIBITMAP, res: c_uint);
    pub fn FreeImage_SetDotsPerMeterY(dib: *const FIBITMAP, res: c_uint);
    pub fn FreeImage_GetInfoHeader(dib: *const FIBITMAP) ->
     *const BITMAPINFOHEADER;
    pub fn FreeImage_GetInfo(dib: *const FIBITMAP) -> *const BITMAPINFO;
    pub fn FreeImage_GetColorType(dib: *const FIBITMAP) ->
     FREE_IMAGE_COLOR_TYPE;
    pub fn FreeImage_GetRedMask(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetGreenMask(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetBlueMask(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetTransparencyCount(dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_GetTransparencyTable(dib: *const FIBITMAP) -> *const BYTE;
    pub fn FreeImage_SetTransparent(dib: *const FIBITMAP, enabled: BOOL);
    pub fn FreeImage_SetTransparencyTable(dib: *const FIBITMAP,
                                          table: *const BYTE, count: c_int);
    pub fn FreeImage_IsTransparent(dib: *const FIBITMAP) -> BOOL;
    pub fn FreeImage_SetTransparentIndex(dib: *const FIBITMAP, index: c_int);
    pub fn FreeImage_GetTransparentIndex(dib: *const FIBITMAP) -> c_int;
    pub fn FreeImage_HasBackgroundColor(dib: *const FIBITMAP) -> BOOL;
    pub fn FreeImage_GetBackgroundColor(dib: *const FIBITMAP,
                                        bkcolor: *const RGBQUAD) -> BOOL;
    pub fn FreeImage_SetBackgroundColor(dib: *const FIBITMAP,
                                        bkcolor: *const RGBQUAD) -> BOOL;
    pub fn FreeImage_GetThumbnail(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_SetThumbnail(dib: *const FIBITMAP,
                                  thumbnail: *const FIBITMAP) -> BOOL;
    pub fn FreeImage_GetICCProfile(dib: *const FIBITMAP) -> *const FIICCPROFILE;
    pub fn FreeImage_CreateICCProfile(dib: *const FIBITMAP, data: *const c_void,
                                      size: c_long) -> *const FIICCPROFILE;
    pub fn FreeImage_DestroyICCProfile(dib: *const FIBITMAP);
    pub fn FreeImage_ConvertLine1To4(target: *const BYTE, source: *const BYTE,
                                     width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine8To4(target: *const BYTE, source: *const BYTE,
                                     width_in_pixels: c_int,
                                     palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine16To4_555(target: *const BYTE,
                                          source: *const BYTE,
                                          width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine16To4_565(target: *const BYTE,
                                          source: *const BYTE,
                                          width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine24To4(target: *const BYTE, source: *const BYTE,
                                      width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine32To4(target: *const BYTE, source: *const BYTE,
                                      width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine1To8(target: *const BYTE, source: *const BYTE,
                                     width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine4To8(target: *const BYTE, source: *const BYTE,
                                     width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine16To8_555(target: *const BYTE,
                                          source: *const BYTE,
                                          width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine16To8_565(target: *const BYTE,
                                          source: *const BYTE,
                                          width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine24To8(target: *const BYTE, source: *const BYTE,
                                      width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine32To8(target: *const BYTE, source: *const BYTE,
                                      width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine1To16_555(target: *const BYTE,
                                          source: *const BYTE,
                                          width_in_pixels: c_int,
                                          palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine4To16_555(target: *const BYTE,
                                          source: *const BYTE,
                                          width_in_pixels: c_int,
                                          palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine8To16_555(target: *const BYTE,
                                          source: *const BYTE,
                                          width_in_pixels: c_int,
                                          palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine16_565_To16_555(target: *const BYTE,
                                                source: *const BYTE,
                                                width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine24To16_555(target: *const BYTE,
                                           source: *const BYTE,
                                           width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine32To16_555(target: *const BYTE,
                                           source: *const BYTE,
                                           width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine1To16_565(target: *const BYTE,
                                          source: *const BYTE,
                                          width_in_pixels: c_int,
                                          palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine4To16_565(target: *const BYTE,
                                          source: *const BYTE,
                                          width_in_pixels: c_int,
                                          palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine8To16_565(target: *const BYTE,
                                          source: *const BYTE,
                                          width_in_pixels: c_int,
                                          palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine16_555_To16_565(target: *const BYTE,
                                                source: *const BYTE,
                                                width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine24To16_565(target: *const BYTE,
                                           source: *const BYTE,
                                           width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine32To16_565(target: *const BYTE,
                                           source: *const BYTE,
                                           width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine1To24(target: *const BYTE, source: *const BYTE,
                                      width_in_pixels: c_int,
                                      palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine4To24(target: *const BYTE, source: *const BYTE,
                                      width_in_pixels: c_int,
                                      palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine8To24(target: *const BYTE, source: *const BYTE,
                                      width_in_pixels: c_int,
                                      palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine16To24_555(target: *const BYTE,
                                           source: *const BYTE,
                                           width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine16To24_565(target: *const BYTE,
                                           source: *const BYTE,
                                           width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine32To24(target: *const BYTE, source: *const BYTE,
                                       width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine1To32(target: *const BYTE, source: *const BYTE,
                                      width_in_pixels: c_int,
                                      palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine4To32(target: *const BYTE, source: *const BYTE,
                                      width_in_pixels: c_int,
                                      palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine8To32(target: *const BYTE, source: *const BYTE,
                                      width_in_pixels: c_int,
                                      palette: *const RGBQUAD);
    pub fn FreeImage_ConvertLine16To32_555(target: *const BYTE,
                                           source: *const BYTE,
                                           width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine16To32_565(target: *const BYTE,
                                           source: *const BYTE,
                                           width_in_pixels: c_int);
    pub fn FreeImage_ConvertLine24To32(target: *const BYTE, source: *const BYTE,
                                       width_in_pixels: c_int);
    pub fn FreeImage_ConvertTo4Bits(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ConvertTo8Bits(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ConvertToGreyscale(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ConvertTo16Bits555(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ConvertTo16Bits565(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ConvertTo24Bits(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ConvertTo32Bits(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ColorQuantize(dib: *const FIBITMAP,
                                   quantize: FREE_IMAGE_QUANTIZE) ->
     *const FIBITMAP;
    pub fn FreeImage_ColorQuantizeEx(dib: *const FIBITMAP,
                                     quantize: FREE_IMAGE_QUANTIZE,
                                     PaletteSize: c_int, ReserveSize: c_int,
                                     ReservePalette: *const RGBQUAD) ->
     *const FIBITMAP;
    pub fn FreeImage_Threshold(dib: *const FIBITMAP, T: BYTE) -> *const FIBITMAP;
    pub fn FreeImage_Dither(dib: *const FIBITMAP, algorithm: FREE_IMAGE_DITHER)
     -> *const FIBITMAP;
    pub fn FreeImage_ConvertFromRawBits(bits: *const BYTE, width: c_int,
                                        height: c_int, pitch: c_int,
                                        bpp: c_uint, red_mask: c_uint,
                                        green_mask: c_uint, blue_mask: c_uint,
                                        topdown: BOOL) -> *const FIBITMAP;
    pub fn FreeImage_ConvertToRawBits(bits: *const BYTE, dib: *const FIBITMAP,
                                      pitch: c_int, bpp: c_uint,
                                      red_mask: c_uint, green_mask: c_uint,
                                      blue_mask: c_uint, topdown: BOOL);
    pub fn FreeImage_ConvertToFloat(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ConvertToRGBF(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ConvertToUINT16(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ConvertToRGB16(dib: *const FIBITMAP) -> *const FIBITMAP;
    pub fn FreeImage_ConvertToStandardType(src: *const FIBITMAP,
                                           scale_linear: BOOL) ->
     *const FIBITMAP;
    pub fn FreeImage_ConvertToType(src: *const FIBITMAP,
                                   dst_type: FREE_IMAGE_TYPE,
                                   scale_linear: BOOL) -> *const FIBITMAP;
    pub fn FreeImage_ToneMapping(dib: *const FIBITMAP, tmo: FREE_IMAGE_TMO,
                                 first_param: c_double,
                                 second_param: c_double) -> *const FIBITMAP;
    pub fn FreeImage_TmoDrago03(src: *const FIBITMAP, gamma: c_double,
                                exposure: c_double) -> *const FIBITMAP;
    pub fn FreeImage_TmoReinhard05(src: *const FIBITMAP, intensity: c_double,
                                   contrast: c_double) -> *const FIBITMAP;
    pub fn FreeImage_TmoReinhard05Ex(src: *const FIBITMAP, intensity: c_double,
                                     contrast: c_double, adaptation: c_double,
                                     color_correction: c_double) ->
     *const FIBITMAP;
    pub fn FreeImage_TmoFattal02(src: *const FIBITMAP,
                                 color_saturation: c_double,
                                 attenuation: c_double) -> *const FIBITMAP;
    pub fn FreeImage_ZLibCompress(target: *const BYTE, target_size: DWORD,
                                  source: *const BYTE, source_size: DWORD) ->
     DWORD;
    pub fn FreeImage_ZLibUncompress(target: *const BYTE, target_size: DWORD,
                                    source: *const BYTE, source_size: DWORD) ->
     DWORD;
    pub fn FreeImage_ZLibGZip(target: *const BYTE, target_size: DWORD,
                              source: *const BYTE, source_size: DWORD) -> DWORD;
    pub fn FreeImage_ZLibGUnzip(target: *const BYTE, target_size: DWORD,
                                source: *const BYTE, source_size: DWORD) ->
     DWORD;
    pub fn FreeImage_ZLibCRC32(crc: DWORD, source: *const BYTE,
                               source_size: DWORD) -> DWORD;
    pub fn FreeImage_CreateTag() -> *const FITAG;
    pub fn FreeImage_DeleteTag(tag: *const FITAG);
    pub fn FreeImage_CloneTag(tag: *const FITAG) -> *const FITAG;
    pub fn FreeImage_GetTagKey(tag: *const FITAG) -> *const c_schar;
    pub fn FreeImage_GetTagDescription(tag: *const FITAG) -> *const c_schar;
    pub fn FreeImage_GetTagID(tag: *const FITAG) -> WORD;
    pub fn FreeImage_GetTagType(tag: *const FITAG) -> FREE_IMAGE_MDTYPE;
    pub fn FreeImage_GetTagCount(tag: *const FITAG) -> DWORD;
    pub fn FreeImage_GetTagLength(tag: *const FITAG) -> DWORD;
    pub fn FreeImage_GetTagValue(tag: *const FITAG) -> *const c_void;
    pub fn FreeImage_SetTagKey(tag: *const FITAG, key: *const c_schar) -> BOOL;
    pub fn FreeImage_SetTagDescription(tag: *const FITAG, description: *const c_schar)
     -> BOOL;
    pub fn FreeImage_SetTagID(tag: *const FITAG, id: WORD) -> BOOL;
    pub fn FreeImage_SetTagType(tag: *const FITAG, _type: FREE_IMAGE_MDTYPE) ->
     BOOL;
    pub fn FreeImage_SetTagCount(tag: *const FITAG, count: DWORD) -> BOOL;
    pub fn FreeImage_SetTagLength(tag: *const FITAG, length: DWORD) -> BOOL;
    pub fn FreeImage_SetTagValue(tag: *const FITAG, value: *const c_void) -> BOOL;
    pub fn FreeImage_FindFirstMetadata(model: FREE_IMAGE_MDMODEL,
                                       dib: *const FIBITMAP,
                                       tag: *const *const FITAG) ->
     *const FIMETADATA;
    pub fn FreeImage_FindNextMetadata(mdhandle: *const FIMETADATA,
                                      tag: *const *const FITAG) -> BOOL;
    pub fn FreeImage_FindCloseMetadata(mdhandle: *const FIMETADATA);
    pub fn FreeImage_SetMetadata(model: FREE_IMAGE_MDMODEL,
                                 dib: *const FIBITMAP, key: *const c_schar,
                                 tag: *const FITAG) -> BOOL;
    pub fn FreeImage_GetMetadata(model: FREE_IMAGE_MDMODEL,
                                 dib: *const FIBITMAP, key: *const c_schar,
                                 tag: *const *const FITAG) -> BOOL;
    pub fn FreeImage_GetMetadataCount(model: FREE_IMAGE_MDMODEL,
                                      dib: *const FIBITMAP) -> c_uint;
    pub fn FreeImage_CloneMetadata(dst: *const FIBITMAP, src: *const FIBITMAP) ->
     BOOL;
    pub fn FreeImage_TagToString(model: FREE_IMAGE_MDMODEL, tag: *const FITAG,
                                 Make: *const c_schar) -> *const c_schar;
    pub fn FreeImage_RotateClassic(dib: *const FIBITMAP, angle: c_double) ->
     *const FIBITMAP;
    pub fn FreeImage_Rotate(dib: *const FIBITMAP, angle: c_double,
                            bkcolor: *const c_void) -> *const FIBITMAP;
    pub fn FreeImage_RotateEx(dib: *const FIBITMAP, angle: c_double,
                              x_shift: c_double, y_shift: c_double,
                              x_origin: c_double, y_origin: c_double,
                              use_mask: BOOL) -> *const FIBITMAP;
    pub fn FreeImage_FlipHorizontal(dib: *const FIBITMAP) -> BOOL;
    pub fn FreeImage_FlipVertical(dib: *const FIBITMAP) -> BOOL;
    pub fn FreeImage_JPEGTransform(src_file: *const c_schar, dst_file: *const c_schar,
                                   operation: FREE_IMAGE_JPEG_OPERATION,
                                   perfect: BOOL) -> BOOL;
    pub fn FreeImage_JPEGTransformU(src_file: *const wchar_t, dst_file: *const wchar_t,
                                    operation: FREE_IMAGE_JPEG_OPERATION,
                                    perfect: BOOL) -> BOOL;
    pub fn FreeImage_Rescale(dib: *const FIBITMAP, dst_width: c_int,
                             dst_height: c_int, filter: FREE_IMAGE_FILTER) ->
     *const FIBITMAP;
    pub fn FreeImage_MakeThumbnail(dib: *const FIBITMAP, max_pixel_size: c_int,
                                   convert: BOOL) -> *const FIBITMAP;
    pub fn FreeImage_AdjustCurve(dib: *const FIBITMAP, LUT: *const BYTE,
                                 channel: FREE_IMAGE_COLOR_CHANNEL) -> BOOL;
    pub fn FreeImage_AdjustGamma(dib: *const FIBITMAP, gamma: c_double) -> BOOL;
    pub fn FreeImage_AdjustBrightness(dib: *const FIBITMAP,
                                      percentage: c_double) -> BOOL;
    pub fn FreeImage_AdjustContrast(dib: *const FIBITMAP, percentage: c_double)
     -> BOOL;
    pub fn FreeImage_Invert(dib: *const FIBITMAP) -> BOOL;
    pub fn FreeImage_GetHistogram(dib: *const FIBITMAP, histo: *const DWORD,
                                  channel: FREE_IMAGE_COLOR_CHANNEL) -> BOOL;
    pub fn FreeImage_GetAdjustColorsLookupTable(LUT: *const BYTE,
                                                brightness: c_double,
                                                contrast: c_double,
                                                gamma: c_double, invert: BOOL)
     -> c_int;
    pub fn FreeImage_AdjustColors(dib: *const FIBITMAP, brightness: c_double,
                                  contrast: c_double, gamma: c_double,
                                  invert: BOOL) -> BOOL;
    pub fn FreeImage_ApplyColorMapping(dib: *const FIBITMAP,
                                       srccolors: *const RGBQUAD,
                                       dstcolors: *const RGBQUAD, count: c_uint,
                                       ignore_alpha: BOOL, swap: BOOL) ->
     c_uint;
    pub fn FreeImage_SwapColors(dib: *const FIBITMAP, color_a: *const RGBQUAD,
                                color_b: *const RGBQUAD, ignore_alpha: BOOL) ->
     c_uint;
    pub fn FreeImage_ApplyPaletteIndexMapping(dib: *const FIBITMAP,
                                              srcindices: *const BYTE,
                                              dstindices: *const BYTE,
                                              count: c_uint, swap: BOOL) ->
     c_uint;
    pub fn FreeImage_SwapPaletteIndices(dib: *const FIBITMAP,
                                        index_a: *const BYTE,
                                        index_b: *const BYTE) -> c_uint;
    pub fn FreeImage_GetChannel(dib: *const FIBITMAP,
                                channel: FREE_IMAGE_COLOR_CHANNEL) ->
     *const FIBITMAP;
    pub fn FreeImage_SetChannel(dst: *const FIBITMAP, src: *const FIBITMAP,
                                channel: FREE_IMAGE_COLOR_CHANNEL) -> BOOL;
    pub fn FreeImage_GetComplexChannel(src: *const FIBITMAP,
                                       channel: FREE_IMAGE_COLOR_CHANNEL) ->
     *const FIBITMAP;
    pub fn FreeImage_SetComplexChannel(dst: *const FIBITMAP, src: *const FIBITMAP,
                                       channel: FREE_IMAGE_COLOR_CHANNEL) ->
     BOOL;
    pub fn FreeImage_Copy(dib: *const FIBITMAP, left: c_int, top: c_int,
                          right: c_int, bottom: c_int) -> *const FIBITMAP;
    pub fn FreeImage_Paste(dst: *const FIBITMAP, src: *const FIBITMAP,
                           left: c_int, top: c_int, alpha: c_int) -> BOOL;
    pub fn FreeImage_Composite(fg: *const FIBITMAP, useFileBkg: BOOL,
                               appBkColor: *const RGBQUAD, bg: *const FIBITMAP) ->
     *const FIBITMAP;
    pub fn FreeImage_JPEGCrop(src_file: *const c_schar, dst_file: *const c_schar,
                              left: c_int, top: c_int, right: c_int,
                              bottom: c_int) -> BOOL;
    pub fn FreeImage_JPEGCropU(src_file: *const wchar_t, dst_file: *const wchar_t,
                               left: c_int, top: c_int, right: c_int,
                               bottom: c_int) -> BOOL;
    pub fn FreeImage_PreMultiplyWithAlpha(dib: *const FIBITMAP) -> BOOL;
    pub fn FreeImage_FillBackground(dib: *const FIBITMAP, color: *const c_void,
                                    options: c_int) -> BOOL;
    pub fn FreeImage_EnlargeCanvas(src: *const FIBITMAP, left: c_int,
                                   top: c_int, right: c_int, bottom: c_int,
                                   color: *const c_void, options: c_int) ->
     *const FIBITMAP;
    pub fn FreeImage_AllocateEx(width: c_int, height: c_int, bpp: c_int,
                                color: *const RGBQUAD, options: c_int,
                                palette: *const RGBQUAD, red_mask: c_uint,
                                green_mask: c_uint, blue_mask: c_uint) ->
     *const FIBITMAP;
    pub fn FreeImage_AllocateExT(_type: FREE_IMAGE_TYPE, width: c_int,
                                 height: c_int, bpp: c_int, color: *const c_void,
                                 options: c_int, palette: *const RGBQUAD,
                                 red_mask: c_uint, green_mask: c_uint,
                                 blue_mask: c_uint) -> *const FIBITMAP;
    pub fn FreeImage_MultigridPoissonSolver(Laplacian: *const FIBITMAP,
                                            ncycle: c_int) -> *const FIBITMAP;
}
