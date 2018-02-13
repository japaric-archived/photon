#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[lang = "start"]
extern "C" fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize
where
    T: Termination,
{
    0
}

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> i32;
}
