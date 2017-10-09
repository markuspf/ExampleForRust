extern crate libc;

use std::ffi::{CString, CStr};
use std::os::raw::c_uint;
use std::io::Write;

// This really is infrastructure one could publish
// in another way than an example package

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
    pub fn InitGVarFuncsFromTable( tab: *mut StructGVarFunc );

    pub fn ArgStringToList(nams_c: *const Char) -> Obj;
    pub fn NewFunction(name: Obj, narg: Int, nams: Obj, hdlr: extern fn(x: Obj) -> Obj) -> Obj;
    pub fn MakeReadOnlyGVar(gvar: UInt);
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

#[no_mangle]
pub fn BlaBla() {
    let mut funcs: Vec<StructGVarFunc> = vec![
        StructGVarFunc {
            name: CString::new("TestCommand").unwrap().into_raw(),
            nargs: 0,
            args: CString::new("").unwrap().into_raw(),
            handler: Some(TestCommand),
            cookie: CString::new("blurblurb").unwrap().into_raw()
        },
        StructGVarFunc {
            name: CString::new("TestCommandPars").unwrap().into_raw(),
            nargs: 1,
            args: CString::new("bla").unwrap().into_raw(),
            handler: Some(TestCommand),
            cookie: CString::new("").unwrap().into_raw()
        },
        StructGVarFunc {
            name: CString::new("").unwrap().into_raw(),
            nargs: 0,
            args: CString::new("bla").unwrap().into_raw(),
            handler: Some(TestCommand),
            cookie: CString::new("").unwrap().into_raw()
        },
    ];
    unsafe{
        InitGVarFuncsFromTable(funcs.as_mut_ptr());
    };
}

#[no_mangle]
pub extern fn Init__Dynamic() -> *mut StructInitInfo {
    return Box::into_raw(Box::new(
        StructInitInfo {
            type_: MODULE_DYNAMIC,
            name: CString::new("rusting").unwrap().as_ptr(),
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

