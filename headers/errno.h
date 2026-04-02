/* Minimal errno.h for bare-metal esp-hal builds (no sysroot).
 * errno is a plain global provided by esp_hal_platform.c. */

#ifndef _ERRNO_H
#define _ERRNO_H

extern int errno;

#define EPERM        1
#define ENOENT       2
#define ESRCH        3
#define EINTR        4
#define EIO          5
#define ENXIO        6
#define E2BIG        7
#define ENOEXEC      8
#define EBADF        9
#define ECHILD       10
#define EAGAIN       11
#define ENOMEM       12
#define EACCES       13
#define EFAULT       14
#define EBUSY        16
#define EEXIST       17
#define ENODEV       19
#define ENOTDIR      20
#define EISDIR       21
#define EINVAL       22
#define EMFILE       24
#define ENOSPC       28
#define EROFS        30
#define EPIPE        32
#define ERANGE       34
#define EDEADLK      35
#define ENAMETOOLONG 36
#define ENOSYS       38
#define ENOTEMPTY    39
#define ELOOP        40
#define EWOULDBLOCK  EAGAIN
#define ENOMSG       42
#define EPROTO       71
#define EOVERFLOW    75
#define ENOTSOCK     88
#define ENOTSUP      95
#define EOPNOTSUPP   ENOTSUP
#define ETIMEDOUT    110
#define ENOTCONN     107

#endif /* _ERRNO_H */
