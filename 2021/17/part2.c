#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

#define BRUTE 1000

typedef struct {
    int x1;
    int x2;
    int y1;
    int y2;
} targetArea;

bool simulate(int vx, int vy, targetArea *target) {
    int x = 0;
    int y = 0;
    while (y > target->y1) {
        x += vx;
        y += vy;
        vy--;
        if (vx > 0)
            vx--;
        if (target->x1 <= x && x <= target->x2 && target->y1 <= y &&
            y <= target->y2) {
            return true;
        }
    }
    return false;
}

int main() {
    FILE *fp = fopen(FILE_NAME, "r");
    int x1, x2, y1, y2;
    fscanf(fp, " target area: x=%d..%d, y=%d..%d\n", &x1, &x2, &y1, &y2);
    targetArea target = {x1, x2, y1, y2};

    int count = 0;
    for (int vx = -BRUTE; vx <= BRUTE; vx++) {
        for (int vy = -BRUTE; vy <= BRUTE; vy++) {
            if (simulate(vx, vy, &target)) {
                printf("%d,%d\n", vx, vy);
                count++;
            }
        }
    }
    printf("count=%d\n", count);
}
