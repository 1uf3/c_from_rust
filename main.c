#include <stdio.h>
typedef unsigned int   uint;

#define NDEV 10

struct inode;

struct devsw {
    int (*read)(struct inode*, char*, int);
    int (*write)(struct inode*, char*, int);
};

struct devsw devsw[NDEV];

extern void consoleinit(struct devsw* devsw);

int main() {
    consoleinit(devsw);

    printf("Read function pointer at index 1: %p\n", (void*)devsw[1].read);
    printf("Write function pointer at index 1: %p\n", (void*)devsw[1].write);

    if (devsw[1].read) {
        int result = devsw[1].read(NULL, NULL, 0);
        printf("Read function result: %d\n", result);
    }

    if (devsw[1].write) {
        int result = devsw[1].write(NULL, NULL, 0);
        printf("Write function result: %d\n", result);
    }

    return 0;
}