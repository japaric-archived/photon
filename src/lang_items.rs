#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[lang = "start"]
extern "C" fn start(
    _main: fn(),
    _argc: isize,
    _argv: *const *const u8,
) -> isize {
    0
}
