-- Copyright (C) 2017 Chris Liebert
local ffi = require "ffi"

function build_wrapper()
  -- Build the shared library and generate the header
  local make_cmd = "cargo build"
  print(make_cmd)
  if not os.execute(make_cmd) == 0 then
    error("Unable to execute " .. make_cmd)
  end
end

function load_wrapper()
  -- Load contents of example.h
  local c_header_filename = "example.h"
  local examplelib_interface_file = io.open(c_header_filename, "r")
  if not examplelib_interface_file then
    -- If example.h is not found, make sure the wrapper has been built with cargo
    -- build.rs will generate example.h
    build_wrapper()
    return load_wrapper()
  end
  local examplelib_interface_file_contents = examplelib_interface_file:read("*a")
  examplelib_interface_file:close()
  ffi.cdef(examplelib_interface_file_contents)

  local wrapper
  local success
  local shared_library = "target/debug/examplelib";
  success, wrapper = pcall(ffi.load, shared_library)
  if not success then
    -- If the shared library does not exist, try building it first
    build_wrapper()
    success, wrapper = pcall(ffi.load, shared_library)
    if not success then
      -- Unable to load shared library after trying to build it first
      error("Unable to load LuaJIT FFI module")
    end
  end
  return wrapper
end

-- Call some methods from the examplelib shared library
local examplelib = load_wrapper()

examplelib.print_hello()
examplelib.print_cstr("This is a C string in from LuaJIT")
examplelib.print_int(
  examplelib.add_ints(6, 7)
)
examplelib.print_double(
  examplelib.add_doubles(32.5, 6.34)
)

local intvec3 = ffi.new("IntVector3")
intvec3.x = 10
intvec3.y = 11
intvec3.z = 12
local doubled_intvec3 = examplelib.add_int_vector3s(intvec3, intvec3)
examplelib.print_int(doubled_intvec3.x)
examplelib.print_int(doubled_intvec3.y)
examplelib.print_int(doubled_intvec3.z)

collectgarbage()
os.exit()