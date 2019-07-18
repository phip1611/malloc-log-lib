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

=> you will get a `malloc-log-lib.txt`-file in your pwd
looks like this:
```
timestamp;kind;size;pid;pointer;
1563492050566161;MALLOC;40;14661;0x2467290;
1563492050567320;MALLOC;40;14661;0x24678b0;
1563492050567612;MALLOC;2;14661;0x2467890;
```

## A few personal notes
This is my first ever Rust-Project. In my opinion (from what I've read and experienced) Rust
is not well suited for situations when you depend on static initialisation/construction and 
destruction of global objects. Of course I understand that Rust's main goal is to run from
the beginning to the end of main() and no further. But for a "system programming language" I
definitely feel less empowered than with C++. 

A **real problem** is that I can't run code after main() is over in Rust. In C++ I used the 
destructor of a global class instance to flush a buffer. I can't make this in Rust so 
I have to write all records immediately and can't buffer them..

I'm sure a lot of my Code could be written much 
better from a more experienced perspective. Right now my code looks kinda messy because of the 
mashup of libc and Rust .. I tried to put my C++-Thinking into this. So don't condemn me
too much for the code please! :D

I like the ideas of Rust and I will defnitely deep-dive even more into it. I want to do a 
"real" Rust-Project (Binary) that "starts at main" - then the real fun begins :)

Feel free to comment or contribute

Philipp