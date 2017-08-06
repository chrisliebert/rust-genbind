rust-genbind
----------
Genbind is a tool designed to generate C header declarations for all Rust functions that are marked with #[no_mangle] in a project.

**How To**

First set `crate-type` to include "dylib" or "staticlib" in your Cargo.toml to instruct cargo to generate a C library (ex: `crate-type = ["staticlib", "rlib"]`). Then run genbind on the root source file containing one or more non-mangled functions to generate the C header file. Now build your project to generated the C library which will appear in the target/* directory. Link your C project to the newly generated C library.

**Extra**

Once you have the C header, it is also possible to generate other language bindings using SWIG: create a new file, `mywrapper.i` with the following (this assumes you have produced a C header called mywrapper.h):
```
#ifndef _MYWRAPPER_I_
#define _MYWRAPPER_I_

#ifdef SWIG

 %module mywrapperlibrary

 %{
  #include "mywrapper.i"
  #include <stdbool.h>
  #include <stdint.h>
  #include "mywrapper.h"
 %}

 %include "mywrapper.h"

#endif /* SWIG */

#endif /* _MYWRAPPER_I_ */

```
Now generate language bindings of your choice, see http://www.swig.org/tutorial.html:
`swig -python mywrapper.i`

Building as "dylib" makes it possible to invoke Rust methods from LuaJIT using the LuaJIT FFI Library, see http://luajit.org/ext_ffi.html

**TODO**

Generate equivalent C structure for structs marked with #repr(C).
Add support for pointers to Rust types marked with #repr(C) 
(see https://github.com/chrisliebert/quick-3d/blob/master/quick3d.h for an example of how the output would look). 
Provide a way to set cfg options such as #[cfg(target_os = "windows")], #[cfg(feature = "somefeature")] and/or translate to preprocessor directives

**License:**

This program and it's source are available under the "MIT License" please see LICENSE
