extern crate libc;

use std::ffi::{CString, CStr};
use std::os::raw::c_uint;
use std::io::Write;

// This really is infrastructure one could publish
// in another way than an example package
// for example as a crate "use gap"?
// Next up would of course to use GAP as library (yet again)

// GAP integer types
pub type Char = ::std::os::raw::c_char;
pub type Int1 = i8;
pub type Int2 = i16;
pub type Int4 = i32;
pub type Int8 = i64;
pub type UChar = u8;
pub type UInt1 = u8;
pub type UInt2 = u16;
pub type UInt4 = u32;
pub type UInt8 = u64;
pub type Int = Int8;
pub type UInt = UInt8;

// TODO: find way to import these from GAP
pub const MODULE_BUILTIN: UInt = 10010;
pub const MODULE_STATIC: UInt = 10011;
pub const MODULE_DYNAMIC: UInt = 10012;

/* Todo: This is a pointer to some raw memory */
pub type Bag = *mut *mut UInt;
pub type Obj = *mut *mut UInt;
pub type ObjFunc = Obj;

extern "C" {
    pub fn GVarName(name: *const Char) -> UInt;
    pub fn NameGVarObj(var: UInt) -> Obj;
    pub fn AssGVar(gvar: UInt, val: Obj);

    pub fn NEW_PREC(size: UInt) -> Obj;

    pub fn InitHandlerFunc(hdlr: extern fn(x: Obj) -> Obj, cookie: *const Char);
    pub fn InitGVarFuncsFromTable(tab: *const StructGVarFunc);

    pub fn ArgStringToList(nams_c: *const Char) -> Obj;
    pub fn NewFunction(name: Obj, narg: Int, nams: Obj, hdlr: extern fn(x: Obj) -> Obj) -> Obj;
    pub fn MakeReadOnlyGVar(gvar: UInt);
}

pub enum HandlerE {
    None,
    NoArg(unsafe extern "C" fn(self_: Obj) -> Obj),
    OneArg(unsafe extern "C" fn(self_: Obj
                                , p1 : Obj) -> Obj),
    TwoArg(unsafe extern "C" fn(self_: Obj
                                , p1 : Obj
                                , p2 : Obj) -> Obj),
    ThreeArg(unsafe extern "C" fn(self_: Obj
                                  , p1 : Obj
                                  , p2 : Obj
                                  , p3 : Obj) -> Obj),
    FourArg(unsafe extern "C" fn(self_: Obj
                                 , p1 : Obj
                                 , p2 : Obj
                                 , p3 : Obj
                                 , p4 : Obj) -> Obj),
    FiveArg(unsafe extern "C" fn(self_: Obj
                                 , p1 : Obj
                                 , p2 : Obj
                                 , p3 : Obj
                                 , p4 : Obj
                                 , p5 : Obj) -> Obj),
    SixArg(unsafe extern "C" fn(self_: Obj
                                , p1 : Obj
                                , p2 : Obj
                                , p3 : Obj
                                , p4 : Obj
                                , p5 : Obj
                                , p6 : Obj) -> Obj),
    ListArg(unsafe extern "C" fn(self_: Obj, list : Obj) -> Obj),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union HandlerT {
    none : i64,
    a0 : unsafe extern "C" fn(self_: Obj) -> Obj,
    a1 : unsafe extern "C" fn(self_: Obj, p1 : Obj) -> Obj,
    a2 : unsafe extern "C" fn(self_: Obj, p1 : Obj, p2 : Obj) -> Obj,
    a3 : unsafe extern "C" fn(self_: Obj, p1 : Obj, p2 : Obj, p3 : Obj) -> Obj,
    a4 : unsafe extern "C" fn(self_: Obj, p1 : Obj, p2 : Obj, p3 : Obj, p4 : Obj) -> Obj,
    a5 : unsafe extern "C" fn(self_: Obj, p1 : Obj, p2 : Obj, p3 : Obj, p4 : Obj, p5 : Obj) -> Obj,
    a6 : unsafe extern "C" fn(self_: Obj, p1 : Obj, p2 : Obj, p3 : Obj, p4 : Obj, p5 : Obj, p6 : Obj) -> Obj,
    aX : unsafe extern "C" fn(self_: Obj, list : Obj) -> Obj,
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructGVarFunc {
    pub name: *const Char,
    pub nargs: Int,
    pub args: *const Char,
    pub handler: HandlerT,
    pub cookie: *const Char
}

// This should really be a constant, but apparently
// const null pointers are a very nightly feature.
pub fn GVarGuard() -> StructGVarFunc {
    return StructGVarFunc {
        name: std::ptr::null(),
        nargs: 0,
        args: std::ptr::null(),
        handler: HandlerT { none : 0 },
        cookie: std::ptr::null()
    };
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StructInitInfo {
    pub type_: UInt,
    pub name: *const Char,
    pub revision_c: *const Char,
    pub revision_h: *const Char,
    pub version: UInt,
    pub crc: Int,
    pub initKernel: ::std::option::Option<unsafe extern "C" fn(arg1: *mut StructInitInfo)
                                                               -> ::std::os::raw::c_long>,
    pub initLibrary: ::std::option::Option<unsafe extern "C" fn(arg1: *mut StructInitInfo)
                                                                -> ::std::os::raw::c_long>,
    pub checkInit: ::std::option::Option<unsafe extern "C" fn(arg1: *mut StructInitInfo)
                                                              -> ::std::os::raw::c_long>,
    pub preSave: ::std::option::Option<unsafe extern "C" fn(arg1: *mut StructInitInfo)
                                                            -> ::std::os::raw::c_long>,
    pub postSave: ::std::option::Option<unsafe extern "C" fn(arg1: *mut StructInitInfo)
                                                             -> ::std::os::raw::c_long>,
    pub postRestore: ::std::option::Option<unsafe extern "C" fn(arg1: *mut StructInitInfo)
                                                                -> ::std::os::raw::c_long>,
    pub filename: *const Char,
    pub isGapRootRelative: Int,
}

#[no_mangle]
pub extern fn TestCommand(self_: Obj) -> Obj {
    writeln!(&mut std::io::stderr(), "Hello from a Rust function called from GAP");
    unsafe{
        let result = NEW_PREC(0);
        return result;
    }
}

#[no_mangle]
pub extern fn TestCommandWithParams(self_: Obj, param: Obj, param2: Obj) -> Obj {
    return param;
}

#[no_mangle]
pub extern fn SolveEquations(self_: Obj, list : Obj) -> Obj {
    return list;
}

#[no_mangle]
pub extern fn InitKernel( module_: *mut StructInitInfo ) -> Int {
    unsafe{
    }
    return 0;
}

pub fn GVarFuncs() -> Vec<StructGVarFunc> {
    return vec![
        GVarFunc("TestCommand", 0, false, "", HandlerE::NoArg(TestCommand)),
        GVarFunc("TestCommandWithParams", 2, false, "p1, p2", HandlerE::TwoArg(TestCommandWithParams)),
        GVarFunc("SolveEquations", 1, true, "p", HandlerE::ListArg(SolveEquations)),
        GVarGuard()];
}

#[no_mangle]
pub extern fn InitLibrary( module_: *mut StructInitInfo ) -> Int {
    unsafe{
        InitGVarFuncsFromTable(GVarFuncs().as_ptr());
    }
    return 0;
}

pub fn GVarFunc(name: &'static str, narg: usize, vararg: bool, argnam: &'static str, f : HandlerE) -> StructGVarFunc
{
    let (harg, h) = match f {
        HandlerE::None => (0, HandlerT { none: 0 }),
        HandlerE::NoArg(f) => (0,HandlerT { a0 : f }),
        HandlerE::OneArg(f) => (1,HandlerT { a1 : f }),
        HandlerE::TwoArg(f) => (2,HandlerT { a2 : f }),
        HandlerE::ThreeArg(f) => (3,HandlerT { a3 : f }),
        HandlerE::FourArg(f) => (4,HandlerT { a4 : f }),
        HandlerE::FiveArg(f) => (5,HandlerT { a5 : f }),
        HandlerE::SixArg(f) => (6,HandlerT { a6 : f }),
        HandlerE::ListArg(f) => (7,HandlerT { aX : f }),
    };
    /* Currently GAP does the following
       if vararg = true (so the last argument of the function is a vararg)
          *or* vararg = false *and* narg > 6
           then
          all parameters are passed in a PLIST, which is passed as the first argument to
          the handler
       otherwise
          the up to 6 parameters are passed as Obj pointers

       Hence we catch wrongly installed handlers here

       possibly we should be parsing argnam, but really this should happen
       statically (at compile time), because we know all properties of the
       handler that we want to install.

       Maybe writing an appropriate macro could work?
     */
    if ( (vararg == true && narg != 1) ||
          (vararg == false &&
           (narg <= 6 && harg != narg) ||
           (narg > 6 && narg != 1))) {
        // Error.
    }

    return StructGVarFunc {
        name: CString::new(name).unwrap().into_raw(),
        nargs: if vararg {
            -(narg as i64)
        } else {
            narg as i64
        },
        args : CString::new(argnam).unwrap().into_raw(),
        handler: h,
        cookie: CString::new(format!("{}:{}", file!(), name)).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern fn Init__Dynamic() -> *mut StructInitInfo {
    return Box::into_raw(Box::new(
        StructInitInfo {
            type_: MODULE_DYNAMIC,
            name: CString::new("ExampleForRust").unwrap().into_raw(),
            revision_c: CString::new("").unwrap().into_raw(),
            revision_h: CString::new("").unwrap().into_raw(),
            version: 0,
            crc: 0,
            initKernel: Some( InitKernel ),
            initLibrary: Some( InitLibrary ),
            checkInit: None,
            preSave: None,
            postSave: None,
            postRestore: None,
            filename: CString::new(file!()).unwrap().into_raw(),
            isGapRootRelative: 0
        }));
}

