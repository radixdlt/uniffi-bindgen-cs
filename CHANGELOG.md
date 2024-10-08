----

### v0.7.0+v0.25.0

- **BREAKING**: Emit `byte[]` instead of `List<byte>` for `bytes` type [#53](https://github.com/NordSecurity/uniffi-bindgen-cs/pull/53).

----

### v0.6.0+v0.25.0

- **BREAKING**: Update to uniffi v0.25.0.
- Implement Display trait method for objects. Override ToString() C# object method.

----

### v0.5.1+v0.24.0

- Bump to correct crate version.

### v0.5.0+v0.24.0

- **BREAKING**: Update uniffi to 0.24.
- **BREAKING**: Remove `package` configuration option, use `namespace` instead.
- Implement `bytes` type.
- Implement `--library` command line option.
- Default config file to `uniffi.toml` in crate root, if no config file is specified in
    command line options.

----

### v0.4.1+v0.23.0

- Bump to correct crate version.

### v0.4.0+v0.23.0

- **BREAKING**: Emit flat enum variants using `PascalCase` instead of `SCREAMING_SNAKE_CASE`.
- Lowercase numeric types in generated bindings code.

----

### v0.3.0, v0.3.1

This is a version somewhere between 0.23.0 and 0.24.0. This was supposed to be a temporary stepping
stone for the actual 0.24.0 version, but ended up never being actually used (at least by us). It
is reverted in main branch. Use v0.2.0 instead.

- **DO NOT USE, UNFINISHED**

----

### v0.2.4+v0.23.0

- Fix missing imports when ImplicitUsings is not enabled.
- Allow configuration of global methods class name (uniffi.toml: global_methods_class_name).

### v0.2.3+v0.23.0

- Fix 0.2 release to be compatible with mozilla/uniffi-rs 0.23.0 after docstring changes.

### v0.2.2+v0.23.0

- Implement docstrings.

### v0.2.1+v0.23.0

- Add `namespace` configuration option to `uniffi.toml`.

### v0.2.0+v0.23.0

- **BREAKING**: Update uniffi to 0.23.0.
- Allow `uniffi-bindgen-cs` to be used as a library.

----
