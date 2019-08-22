# tess2-sys

This crate contains FFI definitions and build script for building libtess2 from [memononen](https://github.com/memononen/libtess2)

## how to use

```
make build           # build the project
make update-bindings # update bindings
make clean           # cleanup and refresh
make updatedeps      # update libtess2 dependencies
make fmt             # format code
make test            # run test
```

You may need to update bindings.rs for enums after running "make update-bindings".

