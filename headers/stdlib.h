#ifndef _STDLIB_H
#define _STDLIB_H

#include <stddef.h>

int abs(int i);
void *bsearch(const void *key, const void *base, size_t nmemb, size_t size,
              int (*compar)(const void *, const void *));
int atoi(const char *str);
void qsort(void *base, size_t nmemb, size_t size,
           int (*compar)(const void *, const void *));

void *malloc(size_t size);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
void abort(void);

#endif