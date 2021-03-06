/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct Pupper {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Pupper() {
    assert_eq!(::std::mem::size_of::<Pupper>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Pupper ) ));
    assert_eq! (::std::mem::align_of::<Pupper>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Pupper ) ));
}
impl Clone for Pupper {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct Doggo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Doggo() {
    assert_eq!(::std::mem::size_of::<Doggo>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Doggo ) ));
    assert_eq! (::std::mem::align_of::<Doggo>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Doggo ) ));
}
impl Clone for Doggo {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct SuchWow {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_SuchWow() {
    assert_eq!(::std::mem::size_of::<SuchWow>() , 1usize , concat ! (
               "Size of: " , stringify ! ( SuchWow ) ));
    assert_eq! (::std::mem::align_of::<SuchWow>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( SuchWow ) ));
}
impl Clone for SuchWow {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct Opaque {
    pub _bindgen_opaque_blob: u8,
}
#[test]
fn bindgen_test_layout_Opaque() {
    assert_eq!(::std::mem::size_of::<Opaque>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Opaque ) ));
    assert_eq! (::std::mem::align_of::<Opaque>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Opaque ) ));
}
extern "C" {
    #[link_name = "\u{1}_ZN6Opaque17eleven_out_of_tenEv"]
    pub fn Opaque_eleven_out_of_ten(this: *mut Opaque) -> SuchWow;
}
extern "C" {
    #[link_name = "\u{1}_ZN6OpaqueC1E6Pupper"]
    pub fn Opaque_Opaque(this: *mut Opaque, pup: Pupper);
}
impl Clone for Opaque {
    fn clone(&self) -> Self { *self }
}
impl Opaque {
    #[inline]
    pub unsafe fn eleven_out_of_ten(&mut self) -> SuchWow {
        Opaque_eleven_out_of_ten(self)
    }
    #[inline]
    pub unsafe fn new(pup: Pupper) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Opaque_Opaque(&mut __bindgen_tmp, pup);
        __bindgen_tmp
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN6Opaque11MAJESTIC_AFE"]
    pub static mut Opaque_MAJESTIC_AF: Doggo;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct Whitelisted {
    pub some_member: Opaque,
}
#[test]
fn bindgen_test_layout_Whitelisted() {
    assert_eq!(::std::mem::size_of::<Whitelisted>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Whitelisted ) ));
    assert_eq! (::std::mem::align_of::<Whitelisted>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Whitelisted ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Whitelisted ) ) . some_member as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( Whitelisted ) , "::" ,
                stringify ! ( some_member ) ));
}
impl Clone for Whitelisted {
    fn clone(&self) -> Self { *self }
}
