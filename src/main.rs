
extern crate clap;

pub mod cli;
pub mod work;


use windows::Result;
pub mod bindings {
    windows::include_bindings!();
}
pub use bindings::{
    Windows::Win32::System::Com::CoInitialize,
    Windows::Win32::Foundation::PWSTR,
    Windows::Win32::Storage::StructuredStorage::PROPVARIANT,
    Windows::Win32::System::PropertiesSystem::{
        SHGetPropertyStoreFromParsingName,
        GPS_READWRITE,
        PROPERTYKEY,
        IPropertyStore,
        InitPropVariantFromStringAsVector,
        PSGetPropertyKeyFromName,
        InitPropVariantFromPropVariantVectorElem,
    },
};


fn main() {

    match unsafe { CoInitialize(std::ptr::null_mut()) } {
        Ok(_) => { },
        Err(e) => {
            eprintln!("{:?} failed to init com32", e);
            std::process::exit(1);
        }
    };

    let app = cli::build_app();
    let args = app.get_matches();
    let work = work::WorkTodo::new(&args);
    match work.do_work() {
        Ok(()) => { },
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    };
}
