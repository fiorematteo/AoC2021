#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

int main() {
    FILE *fp = fopen(FILE_NAME, "r");
    int y = 0;
    while (getc(fp) != 'y') {
    }
    fscanf(fp, "=%d\n", &y);
    printf("%d\n", y);
    int vy = 0;
    vy = y * (y + 1) / 2;
    printf("vy: %d\n", vy);
}
