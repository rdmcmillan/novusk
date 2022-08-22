#ifndef _STRING_H
#define _STRING_H

// Warning: This file was autogenerated with cbindgen.
// To add bindings, edit the source and rerun cbindgen.

/**
 * Copy `n` bytes from `src` to `dest`
 */
const void *memcpy(void *dest, const void *src, size_t n);

/**
 * Copy `n` bytes from `src` to `dest`, guaranteeing correct behavior for overlapping strings.
 */
const void *memmove(void *dest,
                    const void *src,
                    size_t n);

/**
 * Copy no more than `n` from `src` to `dest`, stopping when `c` is found.
 * Returns the position in `dest` one byte passed where `c` was copied, or NULL if `c` was not
 * found in the first `n` bytes of `src`.
 */
const void *memccpy(void *dest,
                    const void *src,
                    int c,
                    size_t n);

/**
 * Set `n` bytes of `s` to `c`
 */
const void *memset(void *s, int c, size_t n);

/**
 * Compare the first `n` bytes of `s1` to `s2`
 */
int memcmp(const void *s1, const void *s2, int n);

#endif /* _STRING_H */
