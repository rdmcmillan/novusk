#ifndef _BITS_STDINT_INTN_H
#define _BITS_STDINT_INTN_H

// Warning: This file was autogenerated with cbindgen.
// To add bindings, edit the source and rerun cbindgen.

typedef signed char int8_t;

typedef short int16_t;

typedef int int32_t;

#if defined(__WORDSIZE) && (__WORDSIZE == 64)
typedef long long int64_t;
#endif

void __use_types_stdint_intn_h(int8_t, int16_t, int32_t, int64_t);

#endif /* _BITS_STDINT_INTN_H */
