fn mismatch(line: u32) -> ! {
    #[cfg(procmacro2_backtrace)]
    {
        let backtrace = std::backtrace::Backtrace::force_capture();
        panic!("compiler/fallback mismatch L{}\n\n{}", line, backtrace)
    }
    #[cfg(not(procmacro2_backtrace))]
    {
        panic!("compiler/fallback mismatch L{}", line)
    }
}