# malloc-log-lib

I recently had a pretty cool project in the university (for/with [os.inf.tu-dresden.de](https://os.inf.tu-dresden.de)) and also I just started getting warm with Rust. Soo.. I thought let's port the project or at least the basic ideas from C/C++  to Rust!

The project was about creating a shared objects for Linux-Systems that you can preload into your binarys. It will trace malloc's and free's for you and give you stats about them in a logfile.

Travis CI: [![Build Status](https://travis-ci.com/phip1611/malloc-log-lib.svg?branch=master)](https://travis-ci.com/phip1611/malloc-log-lib)

## Basic Idea
Basically it's about to port the following code to Rust:
```c++
void * malloc(size_t size) {
     static void *(*real_malloc)(size_t) = nullptr;
     if (real_malloc == nullptr) {
         real_malloc = reinterpret_cast<void *(*)(size_t)> (dlsym(RTLD_NEXT, "malloc"));
     }
     // do some logging stuff
     void * ptr = real_malloc(size);
     return ptr;
}
```

(and also for free)

## How to use
`$ LD_PRELOAD=./target/debug/libmalloc_log_lib.so ./mallocfreetest`