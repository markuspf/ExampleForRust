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
    pub fn InitGVarFuncsFromTable( tab: *const StructGVarFunc );

    pub fn ArgStringToList(nams_c: *const Char) -> Obj;
    pub fn NewFunction(name: Obj, narg: Int, nams: Obj, hdlr: extern fn(x: Obj) -> Obj) -> Obj;
    pub fn MakeReadOnlyGVar(gvar: UInt);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
enum Handler {
    None,
    NoArgs(::std::option::Option<unsafe extern "C" fn(self_: Obj) -> Obj>),
    OneArg(::std::option::Option<unsafe extern "C" fn(self_: Obj, p1 : Obj) -> Obj>),
    TwoArg(::std::option::Option<unsafe extern "C" fn(self_: Obj, p1 : Obj, p2 : Obj) -> Obj>),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StructGVarFunc {
    pub name: *const Char,
    pub nargs: Int,
    pub args: *const Char,
    pub handler: ::std::option::Option<unsafe extern "C" fn(self_: Obj) -> Obj>,
    pub cookie: *const Char
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

pub fn mkstr(s: &str) -> *const Char {
    return CString::new("RUSTING").unwrap().as_ptr();
}

#[no_mangle]
pub extern fn InitKernel( module_: *mut StructInitInfo ) -> Int {
    unsafe{
        InitHandlerFunc(TestCommand, CString::new("RUSTING").unwrap().as_ptr());
    }
    return 0;
}

#[no_mangle]
pub extern fn InitLibrary( module_: *mut StructInitInfo ) -> Int {
    unsafe{
        let tmp = NEW_PREC(0);
        let gvar = GVarName(CString::new("RUSTING").unwrap().as_ptr());
        AssGVar(gvar, tmp);

        let gvar2 = GVarName(CString::new("RUSTING_testfunc").unwrap().as_ptr());
        let func = NewFunction( NameGVarObj(gvar2), 0, ArgStringToList(CString::new("").unwrap().as_ptr()), TestCommand);
        AssGVar(gvar2, func);
        MakeReadOnlyGVar(gvar2);

        BlaBla();
    }
    return 0;
}

pub fn GVarFunc(name: &'static str, nargs: Int, argnam: &'static str, f: unsafe extern "C" fn(Obj) -> Obj) -> StructGVarFunc
{
    return StructGVarFunc {
        name: CString::new(name).unwrap().into_raw(),
        nargs: nargs,
        args: CString::new(argnam).unwrap().into_raw(),
        handler: Some(f),
        cookie: CString::new(file!()).unwrap().into_raw()
    }
}

#[no_mangle]
pub fn BlaBla() {
    let funcs: Vec<StructGVarFunc> = vec![
        GVarFunc("TestCommand", 0, "", TestCommand),
        GVarFunc("TestCommandWithParams", 2, "p1, p2", TestCommand),
        StructGVarFunc {
            name: std::ptr::null(),
            nargs: 0,
            args: std::ptr::null(),
            handler: None,
            cookie: std::ptr::null()
        },
    ];
    unsafe{
        InitGVarFuncsFromTable(funcs.as_ptr());
    };
}

#[no_mangle]
pub extern fn Init__Dynamic() -> *mut StructInitInfo {
    return Box::into_raw(Box::new(
        StructInitInfo {
            type_: MODULE_DYNAMIC,
            name: CString::new("rusting").unwrap().into_raw(),
            revision_c: CString::new("").unwrap().as_ptr(),
            revision_h: CString::new("").unwrap().as_ptr(),
            version: 0,
            crc: 0,
            initKernel: Some( InitKernel ),
            initLibrary: Some( InitLibrary ),
            checkInit: None,
            preSave: None,
            postSave: None,
            postRestore: None,
            filename: CString::new("").unwrap().as_ptr(),
            isGapRootRelative: 0
        }));
}

