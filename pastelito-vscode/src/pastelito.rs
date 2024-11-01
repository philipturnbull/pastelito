// Generated by `wit-bindgen` 0.34.0. DO NOT EDIT!
// Options used:
pub type Results = vscode::pastelito::types::Results;
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_apply_default_rules_cabi<T: Guest>(arg0: *mut u8,arg1: usize,) -> *mut u8 {#[cfg(target_arch="wasm32")]
_rt::run_ctors_once();let len0 = arg1;
let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
let result1 = T::apply_default_rules(_rt::string_lift(bytes0));
let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
let vscode::pastelito::types::Results{ warnings:warnings3, measurements:measurements3, } = result1;
let vec7 = warnings3;
let len7 = vec7.len();
let layout7 = _rt::alloc::Layout::from_size_align_unchecked(vec7.len() * 24, 4);
let result7 = if layout7.size() != 0 {
  let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
  if ptr.is_null()
  {
    _rt::alloc::handle_alloc_error(layout7);
  }
  ptr
}else {
  ::core::ptr::null_mut()
};
for (i, e) in vec7.into_iter().enumerate() {
  let base = result7.add(i * 24);
  {
    let vscode::pastelito::types::Warning{ message:message4, range:range4, } = e;
    let vec5 = (message4.into_bytes()).into_boxed_slice();
    let ptr5 = vec5.as_ptr().cast::<u8>();
    let len5 = vec5.len();
    ::core::mem::forget(vec5);
    *base.add(4).cast::<usize>() = len5;
    *base.add(0).cast::<*mut u8>() = ptr5.cast_mut();
    let vscode::pastelito::types::Range{ start_line:start_line6, start_char_utf16:start_char_utf166, end_line:end_line6, end_char_utf16:end_char_utf166, } = range4;
    *base.add(8).cast::<i32>() = _rt::as_i32(start_line6);
    *base.add(12).cast::<i32>() = _rt::as_i32(start_char_utf166);
    *base.add(16).cast::<i32>() = _rt::as_i32(end_line6);
    *base.add(20).cast::<i32>() = _rt::as_i32(end_char_utf166);
  }
}
*ptr2.add(4).cast::<usize>() = len7;
*ptr2.add(0).cast::<*mut u8>() = result7;
let vec8 = (measurements3).into_boxed_slice();
let ptr8 = vec8.as_ptr().cast::<u8>();
let len8 = vec8.len();
::core::mem::forget(vec8);
*ptr2.add(12).cast::<usize>() = len8;
*ptr2.add(8).cast::<*mut u8>() = ptr8.cast_mut();
ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_apply_default_rules<T: Guest>(arg0: *mut u8,) {
  let l0 = *arg0.add(0).cast::<*mut u8>();
  let l1 = *arg0.add(4).cast::<usize>();
  let base4 = l0;
  let len4 = l1;
  for i in 0..len4 {
    let base = base4.add(i * 24);
    {
      let l2 = *base.add(0).cast::<*mut u8>();
      let l3 = *base.add(4).cast::<usize>();
      _rt::cabi_dealloc(l2, l3, 1);
    }
  }
  _rt::cabi_dealloc(base4, len4 * 24, 4);
  let l5 = *arg0.add(8).cast::<*mut u8>();
  let l6 = *arg0.add(12).cast::<usize>();
  let base7 = l5;
  let len7 = l6;
  _rt::cabi_dealloc(base7, len7 * 20, 4);
}
pub trait Guest {
  fn apply_default_rules(input: _rt::String,) -> Results;
}
#[doc(hidden)]

macro_rules! __export_world_pastelito_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "apply-default-rules"]
    unsafe extern "C" fn export_apply_default_rules(arg0: *mut u8,arg1: usize,) -> *mut u8 {
      $($path_to_types)*::_export_apply_default_rules_cabi::<$ty>(arg0, arg1)
    }
    #[export_name = "cabi_post_apply-default-rules"]
    unsafe extern "C" fn _post_return_apply_default_rules(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_apply_default_rules::<$ty>(arg0)
    }
  };);
}
#[doc(hidden)]
pub(crate) use __export_world_pastelito_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit::<u8>; 16]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 16]);
#[allow(dead_code)]
pub mod vscode {
  #[allow(dead_code)]
  pub mod pastelito {
    #[allow(dead_code, clippy::all)]
    pub mod types {
      #[used]
      #[doc(hidden)]
      static __FORCE_SECTION_REF: fn() =
      super::super::super::__link_custom_section_describing_imports;
      
      use super::super::super::_rt;
      #[repr(C)]
      #[derive(Clone, Copy)]
      pub struct Range {
        pub start_line: u32,
        pub start_char_utf16: u32,
        pub end_line: u32,
        pub end_char_utf16: u32,
      }
      impl ::core::fmt::Debug for Range {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("Range").field("start-line", &self.start_line).field("start-char-utf16", &self.start_char_utf16).field("end-line", &self.end_line).field("end-char-utf16", &self.end_char_utf16).finish()
        }
      }
      #[derive(Clone)]
      pub struct Warning {
        pub message: _rt::String,
        pub range: Range,
      }
      impl ::core::fmt::Debug for Warning {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("Warning").field("message", &self.message).field("range", &self.range).finish()
        }
      }
      #[repr(C)]
      #[derive(Clone, Copy)]
      pub struct Measurement {
        pub key: u32,
        pub range: Range,
      }
      impl ::core::fmt::Debug for Measurement {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("Measurement").field("key", &self.key).field("range", &self.range).finish()
        }
      }
      #[derive(Clone)]
      pub struct Results {
        pub warnings: _rt::Vec::<Warning>,
        pub measurements: _rt::Vec::<Measurement>,
      }
      impl ::core::fmt::Debug for Results {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("Results").field("warnings", &self.warnings).field("measurements", &self.measurements).finish()
        }
      }

    }

  }
}
mod _rt {
  pub use alloc_crate::string::String;
  pub use alloc_crate::vec::Vec;

  #[cfg(target_arch = "wasm32")]
  pub fn run_ctors_once() {
    wit_bindgen::rt::run_ctors_once();
  }
  pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
    if cfg!(debug_assertions) {
      String::from_utf8(bytes).unwrap()
    } else {
      String::from_utf8_unchecked(bytes)
    }
  }
  
  pub fn as_i32<T: AsI32>(t: T) -> i32 {
    t.as_i32()
  }

  pub trait AsI32 {
    fn as_i32(self) -> i32;
  }

  impl<'a, T: Copy + AsI32> AsI32 for &'a T {
    fn as_i32(self) -> i32 {
      (*self).as_i32()
    }
  }
  
  impl AsI32 for i32 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for u32 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for i16 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for u16 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for i8 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for u8 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for char {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for usize {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  pub use alloc_crate::alloc;
  pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
    if size == 0 {
      return;
    }
    let layout = alloc::Layout::from_size_align_unchecked(size, align);
    alloc::dealloc(ptr, layout);
  }
  extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_pastelito_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::__export_world_pastelito_cabi!($ty with_types_in $($path_to_types_root)*);
  )
}
#[doc(inline)]
pub(crate) use __export_pastelito_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.34.0:vscode:pastelito:pastelito:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 436] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xb4\x02\x01A\x02\x01\
A\x06\x01B\x0a\x01r\x04\x0astart-liney\x10start-char-utf16y\x08end-liney\x0eend-\
char-utf16y\x04\0\x05range\x03\0\0\x01r\x02\x07messages\x05range\x01\x04\0\x07wa\
rning\x03\0\x02\x01r\x02\x03keyy\x05range\x01\x04\0\x0bmeasurement\x03\0\x04\x01\
p\x03\x01p\x05\x01r\x02\x08warnings\x06\x0cmeasurements\x07\x04\0\x07results\x03\
\0\x08\x03\0\x16vscode:pastelito/types\x05\0\x02\x03\0\0\x07results\x03\0\x07res\
ults\x03\0\x01\x01@\x01\x05inputs\0\x02\x04\0\x13apply-default-rules\x01\x03\x04\
\0\x1avscode:pastelito/pastelito\x04\0\x0b\x0f\x01\0\x09pastelito\x03\0\0\0G\x09\
producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.219.1\x10wit-bindgen-rus\
t\x060.34.0";

#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
  wit_bindgen::rt::maybe_link_cabi_realloc();
}

