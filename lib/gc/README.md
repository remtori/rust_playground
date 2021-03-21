# GC - Garbage Collection

Experimenting with Garbage Collection in Rust:

- No `placement new` so we can not move stuff to pre-allocated space

- No "sounded" way to calling a destructor (`Drop`) on GC-ed value
