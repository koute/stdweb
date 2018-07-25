use std::mem;
use std::marker::PhantomData;

use webcore::value::Value;
use webcore::serialization::{JsSerialize, SerializedValue};

struct GlobalArena {
    memory: *mut u8,
    capacity: usize,
    length: usize
}

static mut VALUE_ARENA: GlobalArena = GlobalArena {
    memory: 0 as *mut u8,
    capacity: 0,
    length: 0
};

static mut ARENA: GlobalArena = GlobalArena {
    memory: 0 as *mut u8,
    capacity: 0,
    length: 0
};

pub struct RelativeSlice< 'a, T: 'a > {
    offset: usize,
    length: usize,
    tail: usize,
    phantom: PhantomData< &'a T >
}

impl< 'a, T: 'a > RelativeSlice< 'a, T > {
    #[inline]
    pub fn offset( &self ) -> usize {
        self.offset
    }

    #[inline]
    pub fn len( &self ) -> usize {
        self.length
    }

    #[inline]
    pub unsafe fn append( &mut self, value: T ) {
        debug_assert!( self.tail + mem::size_of::< T >() <= self.offset + self.length * mem::size_of::< T >() );

        let pointer = ARENA.memory.offset( self.tail as isize ) as *mut T;
        mem::forget( mem::replace( &mut *pointer, value ) );
        self.tail += mem::size_of::< T >();
    }
}

#[doc(hidden)]
pub fn serialize_value< 'a >( value: Value ) -> SerializedValue< 'a > {
    unsafe {
        let mut vec = Vec::from_raw_parts( VALUE_ARENA.memory as *mut Value, VALUE_ARENA.length, VALUE_ARENA.capacity );
        vec.push( value );
        let pointer = vec.last().unwrap() as *const Value;
        VALUE_ARENA.memory = vec.as_mut_ptr() as *mut u8;
        VALUE_ARENA.length = vec.len();
        VALUE_ARENA.capacity = vec.capacity();
        mem::forget( vec );

        JsSerialize::_into_js( &*pointer )
    }
}

#[inline]
pub fn reserve< 'a, T >( length: usize ) -> RelativeSlice< 'a, T > {
    unsafe {
        let offset = reserve_impl( length * mem::size_of::< T >(), mem::align_of::< T >() );
        debug_assert_eq!( ARENA.memory.offset( offset as isize ) as usize % mem::align_of::< T >(), 0 );

        RelativeSlice { offset, length, tail: offset, phantom: PhantomData }
    }
}

unsafe fn reserve_impl( byte_length: usize, align: usize ) -> usize {
    let misaligned_bytes = ARENA.memory as usize % align;
    let alignment_bytes = if misaligned_bytes > 0 {
        align - misaligned_bytes
    } else {
        0
    };

    let byte_length = byte_length + alignment_bytes;
    if ARENA.length + byte_length > ARENA.capacity {
        let mut vector = Vec::from_raw_parts( ARENA.memory, ARENA.length, ARENA.capacity );
        vector.reserve( byte_length );
        ARENA.capacity = vector.capacity();
        ARENA.memory = vector.as_mut_ptr();
        mem::forget( vector );

        __js_raw_asm!( "Module.STDWEB_PRIVATE.arena = $0;", ARENA.memory );
    }

    let offset = ARENA.length + alignment_bytes;
    ARENA.length += byte_length;
    offset
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ArenaRestorePoint {
    arena_length: usize,
    value_arena_length: usize
}

impl ArenaRestorePoint {
    #[doc(hidden)]
    #[inline]
    pub fn new() -> Self {
        unsafe {
            ArenaRestorePoint {
                arena_length: ARENA.length,
                value_arena_length: VALUE_ARENA.length
            }
        }
    }
}

impl Drop for ArenaRestorePoint {
    fn drop( &mut self ) {
        unsafe {
            debug_assert!( ARENA.length >= self.arena_length );
            ARENA.length = self.arena_length;

            debug_assert!( VALUE_ARENA.length >= self.value_arena_length );
            let count = VALUE_ARENA.length - self.value_arena_length;
            if count > 0 {
                let mut vec = Vec::from_raw_parts( VALUE_ARENA.memory as *mut Value, VALUE_ARENA.length, VALUE_ARENA.capacity );
                vec.truncate( self.value_arena_length );
                VALUE_ARENA.memory = vec.as_mut_ptr() as *mut u8;
                VALUE_ARENA.length = vec.len();
                VALUE_ARENA.capacity = vec.capacity();
                mem::forget( vec );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ARENA, VALUE_ARENA, GlobalArena};

    unsafe fn clear() {
        // This will leak, but in tests we don't care.
        ARENA = GlobalArena {
            memory: 0 as *mut u8,
            capacity: 0,
            length: 0
        };

        VALUE_ARENA = GlobalArena {
            memory: 0 as *mut u8,
            capacity: 0,
            length: 0
        };
    }

    #[test]
    fn empty_js_call_does_not_touch_arena() {
        unsafe {
            clear();
            js!();

            assert_eq!( ARENA.memory, 0 as *mut u8 );
            assert_eq!( ARENA.length, 0 );
            assert_eq!( ARENA.capacity, 0 );
        }
    }

    #[test]
    fn arena_is_properly_cleared_after_a_js_call() {
        unsafe {
            clear();
            js!( @{&[1, 2, 3, 4, 5, 6, 7, 8][..]}; );

            assert_ne!( ARENA.memory, 0 as *mut u8 );
            assert_eq!( ARENA.length, 0 );
            assert_ne!( ARENA.capacity, 0 );
        }
    }

    #[test]
    fn arena_is_not_reallocated_when_it_is_not_necessary() {
        unsafe {
            clear();
            js!( @{&[1, 2, 3, 4, 5, 6, 7, 8][..]}; );

            let memory = ARENA.memory;
            let capacity = ARENA.capacity;

            js!( @{&[1, 2, 3, 4, 5, 6, 7, 8][..]}; );

            assert_eq!( ARENA.memory, memory );
            assert_eq!( ARENA.capacity, capacity );
        }
    }
}
