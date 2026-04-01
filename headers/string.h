#ifndef _STRING_H
#define _STRING_H

/* Tinyrlibc implement these. */
char *strcat(char *destination, const char *source);
char *strcpy(char *destination, const char *source);
#define strchr(s, c) __builtin_strchr(s, c)
#define strstr(haystack, needle) __builtin_strstr(haystack, needle)
#define memchr(s, c, n) __builtin_memchr(s, c, n)

/* Clang builtins. */
#define strlen(str) __builtin_strlen(str)
#define strcmp(str1, str2) __builtin_strcmp(str1, str2)
#define strncmp(str1, str2, size) __builtin_strncmp(str1, str2, size)
#define memcpy(dst, src, size) __builtin_memcpy(dst, src, size)
#define memcmp(ptr1, ptr2, size) __builtin_memcmp(ptr1, ptr2, size)
#define memset(dst, value, size) __builtin_memset(dst, value, size)
#define memmove(dst, src, size) __builtin_memmove(dst, src, size)



#endif