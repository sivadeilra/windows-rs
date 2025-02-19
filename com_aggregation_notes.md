

Two different ways you could do it:
1. Put the user data all together (contiguous in memory), and separately put the interface pointers all together (contiguous, in a different memory range)
2. Put the interface pointers next to the data for each "class".



Both have tradeoffs.


# Interfaces, then data

In this design, we have to have some way to group together all of the interface pointers, then
tell the interface pointers how they can find their app data.






class Base : IBase { ... }
class Foo : Base, IFoo, IBar { ... }



ComObject alloc:
  refcount: AtomicU32,

MyBase_Impl:
  interface: IBase*


Foo_Impl:
  interface: IFoo*
  interface: IBar*
  

When we compile class Foo, we know about Base but we don't know the full set of Base's interfaces.
All we can do is reason about Base_Impl.

Need to re-invent virtual destructors...

