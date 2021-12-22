#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

#define STARTX 100
#define STARTY 100
#define STARTZ 100
#define SIZE 200

bool board[SIZE][SIZE][SIZE];

bool bounds(int x, int y, int z) {
    x += STARTX;
    y += STARTY;
    z += STARTZ;
    return x >= 0 && x < SIZE && y >= 0 && y < SIZE && z >= 0 && z < SIZE;
}

int count() {
    int count = 0;
    for (int x = 0; x < SIZE; x++) {
        for (int y = 0; y < SIZE; y++) {
            for (int z = 0; z < SIZE; z++) {
                if (board[x][y][z]) {
                    count++;
                }
            }
        }
    }
    return count;
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");

    char *line = NULL;
    size_t len = 0;
    while (getline(&line, &len, file) != EOF) {
        int x0, x1, y0, y1, z0, z1;
        sscanf(line, "on x=%d..%d,y=%d..%d,z=%d..%d", &x0, &x1, &y0, &y1,
               &z0, &z1);
        sscanf(line, "off x=%d..%d,y=%d..%d,z=%d..%d", &x0, &x1, &y0, &y1,
               &z0, &z1);
        if (!(bounds(x0, y0, z0) && bounds(x1, y1, z1)))
            continue;
        if (x0 > x1) {
            int tmp = x0;
            x0 = x1;
            x1 = tmp;
        }
        if (y0 > y1) {
            int tmp = y0;
            y0 = y1;
            y1 = tmp;
        }
        if (z0 > z1) {
            int tmp = z0;
            z0 = z1;
            z1 = tmp;
        }
        printf("from %d to %d, from %d to %d, from %d to %d\n", x0, x1, y0, y1,
               z0, z1);
        for (int x = x0; x <= x1; x++) {
            for (int y = y0; y <= y1; y++) {
                for (int z = z0; z <= z1; z++) {
                    board[x + STARTX][y + STARTY][z + STARTZ] = line[1] == 'n';
                }
            }
        }
        line = NULL;
    }
    printf("this many cubes: %d\n", count());
}
