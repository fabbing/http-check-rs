use dd_rs_checks::sink::cshlib::{self, SharedLibrary, callback::Callback};
use dd_rs_checks::check::Check;

use http_check::check::HttpCheck;
use libc::c_char;

// Wrapper type to satisfy HRTB requirements
struct HttpCheckWrapper;

impl<'a> Check<'a, SharedLibrary> for HttpCheckWrapper {
    fn build(
        sink: &'a SharedLibrary,
        init_cfg: &dd_rs_checks::Mapping,
        instance_cfg: &dd_rs_checks::Mapping,
    ) -> impl Check<'a, SharedLibrary> {
        HttpCheck::build(sink, init_cfg, instance_cfg)
    }

    fn run(&mut self) -> impl std::future::Future<Output = dd_rs_checks::Result<()>> + Send + Sync {
        async { Ok(()) }
    }
}

//(char *, char *, char *, const aggregator_t *, const char **);
#[unsafe(no_mangle)]
pub extern "C" fn Run(
    check_id: *const c_char,
    init_config: *const c_char,
    instance_config: *const c_char,
    callback: *const Callback,
    error: *mut *const c_char,
) {
    cshlib::run::<HttpCheckWrapper>(check_id, init_config, instance_config, callback, error)
}

#[unsafe(no_mangle)]
pub extern "C" fn Version(_error: *mut *const c_char) -> *const c_char {
    // FIXME NULL terminated!
    http_check::version::VERSION.as_ptr().cast()
}
