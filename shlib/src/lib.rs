use dd_rs_checks::check::Check;
use dd_rs_checks::sink::cshlib::{self, SharedLibrary, callback::Callback};

use http_check::check::HttpCheck;
use http_check::config::{Init, Instance};
use libc::c_char;

//(char *, char *, char *, const aggregator_t *, const char **);
#[unsafe(no_mangle)]
pub extern "C" fn Run(
    check_id: *const c_char,
    init_config: *const c_char,
    instance_config: *const c_char,
    callback: *const Callback,
    error: *mut *const c_char,
) {
    // FIXME
    //cshlib::run::<HttpCheckWrapper>(check_id, init_config, instance_config, callback, error)
}

#[unsafe(no_mangle)]
pub extern "C" fn Version(_error: *mut *const c_char) -> *const c_char {
    // FIXME NULL terminated!
    http_check::version::VERSION.as_ptr().cast()
}
