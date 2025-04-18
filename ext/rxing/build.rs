fn main() {
    // Check if compiling on Windows
    #[cfg(target_os = "windows")]
    {
        use std::env;
        use std::fs;
        use std::path::Path;

        let out_dir = env::var("OUT_DIR").unwrap();
        let strings_h_path = Path::new(&out_dir).join("strings.h");

        // Create a simple strings.h implementation for Windows
        fs::write(
            &strings_h_path,
            r#"#ifndef STRINGS_H
#define STRINGS_H
#include <string.h>

#ifdef __cplusplus
extern "C" {
#endif

static inline int strcasecmp(const char *s1, const char *s2) {
    return _stricmp(s1, s2);
}

static inline int strncasecmp(const char *s1, const char *s2, size_t n) {
    return _strnicmp(s1, s2, n);
}

#ifdef __cplusplus
}
#endif

#endif /* STRINGS_H */
"#,
        ).unwrap();

        // Add OUT_DIR to BINDGEN_EXTRA_CLANG_ARGS environment variable
        println!("cargo:rustc-env=BINDGEN_EXTRA_CLANG_ARGS=-I{}", out_dir);
    }
}