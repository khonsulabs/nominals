#![allow(unsafe_code)]

#[cfg(feature = "alloc")]
use alloc::string::String;
use core::fmt::{Debug, Display};
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

/// A string that can contain most formatted nominals without a heap allocation.
///
/// This type can store up to 47 bytes on the stack before requiring a heap
/// allocation. The total size of this structure is 64 bytes on a 64-bit
/// architecture.
#[derive(Debug)]
#[cfg_attr(feature = "alloc", derive(Clone))]
pub struct NominalString(MaybeInline);

impl NominalString {
    /// The maximum byte capacity this type can contain before allocating on the
    /// heap.
    pub const INLINE_CAPACITY: usize = MaybeInline::INLINE_CAPACITY as usize;

    /// Returns an empty string.
    #[must_use]
    pub const fn new() -> Self {
        Self(MaybeInline::new())
    }

    /// Pushes `ch` to the end of the string.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfMemoryError`] if no additiol space is available and the
    /// `alloc` feature is disabled.
    pub fn try_push(&mut self, ch: char) -> Result<(), OutOfMemoryError> {
        self.0.push(ch)
    }

    /// Pushes `str` to the end of the string.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfMemoryError`] if no additiol space is available and the
    /// `alloc` feature is disabled.
    pub fn try_push_str(&mut self, str: &str) -> Result<(), OutOfMemoryError> {
        self.0.push_str(str)
    }

    /// Pushes `ch` to the start of the string.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfMemoryError`] if no additiol space is available and the
    /// `alloc` feature is disabled.
    pub fn try_push_front(&mut self, ch: char) -> Result<(), OutOfMemoryError> {
        self.0.push_front(ch)
    }

    /// Returns true if this string is currently stored on the stack.
    #[must_use]
    pub fn is_inline(&self) -> bool {
        matches!(self.0, MaybeInline::Inline(_))
    }

    /// Returns the heap-allocated [`String`] inside of `self`, if `self` is
    /// heap allocated.
    ///
    /// # Errors
    ///
    /// If `self` is inline, `Err(self)` will be returned.
    #[cfg(feature = "alloc")]
    pub fn try_into_string(self) -> Result<String, Self> {
        match self.0 {
            MaybeInline::Inline(inline) => Err(Self(MaybeInline::Inline(inline))),
            MaybeInline::Heap(string) => Ok(string),
        }
    }
}

#[cfg(feature = "alloc")]
impl From<NominalString> for String {
    fn from(s: NominalString) -> Self {
        match s.try_into_string() {
            Ok(string) => string,
            Err(s) => String::from(&*s),
        }
    }
}

impl Display for NominalString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl Default for NominalString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "alloc")]
impl From<String> for NominalString {
    fn from(value: String) -> Self {
        NominalString(MaybeInline::Heap(value))
    }
}

impl From<&'_ str> for NominalString {
    fn from(value: &'_ str) -> Self {
        match u8::try_from(value.len()) {
            Ok(value_len) if value_len <= MaybeInline::INLINE_CAPACITY => {
                let mut bytes = [MaybeUninit::uninit(); MaybeInline::INLINE_CAPACITY as usize];

                for (dest, src) in bytes[0..value.len()].iter_mut().zip(value.as_bytes()) {
                    dest.write(*src);
                }
                NominalString(MaybeInline::Inline(InlineString {
                    length: value_len,
                    bytes,
                }))
            }
            _ => {
                #[cfg(feature = "alloc")]
                {
                    Self::from(String::from(value))
                }

                #[cfg(not(feature = "alloc"))]
                {
                    panic!("string too long to store on stack");
                }
            }
        }
    }
}

impl From<char> for NominalString {
    fn from(ch: char) -> Self {
        let mut s = Self::new();
        s.try_push(ch).expect("at least one char fits inline");
        s
    }
}

impl Deref for NominalString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl DerefMut for NominalString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_str_mut()
    }
}

impl Eq for NominalString {}

impl PartialEq<str> for NominalString {
    fn eq(&self, other: &str) -> bool {
        &**self == other
    }
}

impl PartialEq for NominalString {
    fn eq(&self, other: &Self) -> bool {
        self == &**other
    }
}

impl PartialEq<&'_ str> for NominalString {
    fn eq(&self, other: &&'_ str) -> bool {
        self == *other
    }
}

impl PartialOrd<str> for NominalString {
    fn partial_cmp(&self, other: &str) -> Option<core::cmp::Ordering> {
        Some((**self).cmp(other))
    }
}

impl Ord for NominalString {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (**self).cmp(&**other)
    }
}

impl PartialOrd for NominalString {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
enum MaybeInline {
    Inline(InlineString),
    #[cfg(feature = "alloc")]
    Heap(String),
}

impl Debug for MaybeInline {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Inline(_) => f.debug_tuple("Inline").field(&self.as_str()).finish(),
            #[cfg(feature = "alloc")]
            Self::Heap(_) => f.debug_tuple("Heap").field(&self.as_str()).finish(),
        }
    }
}

#[derive(Clone, Copy)]
struct InlineString {
    length: u8,
    bytes: [MaybeUninit<u8>; MaybeInline::INLINE_CAPACITY as usize],
}

impl InlineString {
    fn as_bytes(&self) -> &[u8] {
        // SAFETY: This function only returns access to the bytes that have
        // been initialized, indicated by `self.length`.
        unsafe { core::slice::from_raw_parts(self.bytes.as_ptr().cast(), usize::from(self.length)) }
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: This function only returns access to the bytes that have been
        // initialized, indicated by `self.length`. Because this borrow uses
        // `self`'s lifetime, it ensures only one exclusive reference can be
        // created.
        unsafe {
            core::slice::from_raw_parts_mut(
                self.bytes.as_mut_ptr().cast(),
                usize::from(self.length),
            )
        }
    }

    fn as_str(&self) -> &str {
        // SAFETY: This type only performs unicode-safe operations, and ensures
        // that the bytes through `length` are valid UTF-8. `as_bytes` only
        // returns the written-to portions of the string.
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    fn as_mut_str(&mut self) -> &mut str {
        // SAFETY: This type only performs unicode-safe operations, and ensures
        // that the bytes through `length` are valid UTF-8. `as_bytes_mut` only
        // returns the written-to portions of the string.
        unsafe { core::str::from_utf8_unchecked_mut(self.as_bytes_mut()) }
    }
}

impl MaybeInline {
    const INLINE_CAPACITY: u8 = 62;

    const fn new() -> MaybeInline {
        MaybeInline::Inline(InlineString {
            length: 0,
            bytes: [MaybeUninit::uninit(); Self::INLINE_CAPACITY as usize],
        })
    }

    #[allow(clippy::unnecessary_wraps)]
    fn push(&mut self, ch: char) -> Result<(), OutOfMemoryError> {
        match self {
            MaybeInline::Inline(inline) => {
                let char_len = u8::try_from(ch.len_utf8()).expect("4 < u8::MAX");
                let new_length = inline.length + char_len;
                if new_length <= Self::INLINE_CAPACITY {
                    let mut utf8_bytes = [0; 4];
                    ch.encode_utf8(&mut utf8_bytes);
                    for (dest, src) in inline.bytes
                        [usize::from(inline.length)..usize::from(new_length)]
                        .iter_mut()
                        .zip(&utf8_bytes[0..usize::from(char_len)])
                    {
                        dest.write(*src);
                    }
                    inline.length = new_length;
                } else {
                    #[cfg(feature = "alloc")]
                    {
                        let mut string = String::with_capacity(usize::from(new_length));
                        string.push_str(inline.as_str());
                        string.push(ch);
                        *self = MaybeInline::Heap(string);
                    }
                    #[cfg(not(feature = "alloc"))]
                    {
                        return Err(OutOfMemoryError);
                    }
                }
            }
            #[cfg(feature = "alloc")]
            MaybeInline::Heap(s) => s.insert(0, ch),
        }
        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    fn push_str(&mut self, str: &str) -> Result<(), OutOfMemoryError> {
        match self {
            MaybeInline::Inline(inline) => {
                if let Ok(insert_len) = u8::try_from(str.len()) {
                    let new_length = inline.length.checked_add(insert_len);
                    match new_length {
                        Some(new_length) if new_length <= Self::INLINE_CAPACITY => {
                            // SAFETY: This snippet copies initialized data into
                            // uninitialized locations, causing them to become
                            // initialized. No read access is performed on
                            // uninitialized data.
                            unsafe {
                                inline
                                    .bytes
                                    .as_mut_ptr()
                                    .cast::<u8>()
                                    .add(usize::from(inline.length))
                                    .copy_from(str.as_bytes().as_ptr(), str.len());
                            };

                            inline.length = new_length;
                            return Ok(());
                        }
                        _ => {}
                    }
                }

                #[cfg(feature = "alloc")]
                {
                    let new_length = usize::from(inline.length) + str.len();
                    let mut string = String::with_capacity(new_length);
                    string.push_str(inline.as_str());
                    string.push_str(str);
                    *self = MaybeInline::Heap(string);
                    Ok(())
                }
                #[cfg(not(feature = "alloc"))]
                {
                    return Err(OutOfMemoryError);
                }
            }
            #[cfg(feature = "alloc")]
            MaybeInline::Heap(s) => {
                s.push_str(str);
                Ok(())
            }
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn push_front(&mut self, ch: char) -> Result<(), OutOfMemoryError> {
        match self {
            MaybeInline::Inline(inline) => {
                let char_len = u8::try_from(ch.len_utf8()).expect("4 < u8::MAX");
                let new_length = inline.length + char_len;
                if new_length <= Self::INLINE_CAPACITY {
                    // SAFETY: This snippet copies initialized data into
                    // uninitialized locations, causing them to become
                    // initialized. No read access is performed on
                    // uninitialized data.
                    unsafe {
                        inline
                            .bytes
                            .as_mut_ptr()
                            .cast::<u8>()
                            .add(usize::from(char_len))
                            .copy_from(inline.as_bytes().as_ptr(), usize::from(inline.length));
                        inline.length = new_length;
                    };

                    ch.encode_utf8(inline.as_bytes_mut());
                } else {
                    #[cfg(feature = "alloc")]
                    {
                        let mut string = String::with_capacity(usize::from(new_length));
                        string.push(ch);
                        string.push_str(inline.as_str());
                        *self = MaybeInline::Heap(string);
                    }
                    #[cfg(not(feature = "alloc"))]
                    {
                        return Err(OutOfMemoryError);
                    }
                }
            }
            #[cfg(feature = "alloc")]
            MaybeInline::Heap(s) => s.insert(0, ch),
        }
        Ok(())
    }

    fn as_str(&self) -> &str {
        match self {
            MaybeInline::Inline(s) => s.as_str(),
            #[cfg(feature = "alloc")]
            MaybeInline::Heap(s) => s,
        }
    }

    fn as_str_mut(&mut self) -> &mut str {
        match self {
            MaybeInline::Inline(s) => s.as_mut_str(),
            #[cfg(feature = "alloc")]
            MaybeInline::Heap(s) => s.as_mut_str(),
        }
    }
}

/// No additional memory was able to be allocated.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct OutOfMemoryError;

#[test]
fn preconditions() {
    // The push[_front]() functions rely on the fact that at the time of writing
    // its code, INLINE_CAPACITY was a fixed value. This guaranteees that adding
    // the length of a utf-8 encoded char will never overflow. If we change INLINE_CAPACITY to be something that could be 251 or larger, we shou
    assert!(MaybeInline::INLINE_CAPACITY.checked_add(4).is_some());
}
