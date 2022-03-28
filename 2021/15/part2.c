#include <assert.h>
#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

#define TILE_SIZE 100
#define CAVE_SIZE TILE_SIZE * 5
int data[CAVE_SIZE][CAVE_SIZE];
int cost[CAVE_SIZE][CAVE_SIZE];

int min(int a, int b, int c, int d) {
    int min = a;
    if (b < min) {
        min = b;
    }
    if (c < min) {
        min = c;
    }
    if (d < min) {
        min = d;
    }
    return min;
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    int fx = 0, fy = 0;
    char c;
    while (fscanf(file, "%c", &c) == 1) {
        if (c == '\n') {
            fx = 0;
            fy++;
        } else {
            data[fy][fx] = c - '0';
            fx++;
        }
    }
    fclose(file);

    for (int y = 0; y < TILE_SIZE; y++) {
        for (int x = 0; x < TILE_SIZE; x++) {
            for (int dy = 0; dy < 5; dy++) {
                for (int dx = 0; dx < 5; dx++) {
                    if (dx != 0 || dy != 0) {
                        int newValue = data[y][x] + dx + dy;
                        if (newValue > 9)
                            newValue -= 9;
                        data[y + TILE_SIZE * dy][x + TILE_SIZE * dx] = newValue;
                    }
                }
            }
        }
    }

    data[0][0] = 0;
    for (int i = 0; i < 2000; i++) {
        for (int y = 0; y < CAVE_SIZE; y++) {
            for (int x = 0; x < CAVE_SIZE; x++) {
                if (x == 0 && y == 0)
                    continue;
                cost[y][x] = data[y][x] +
                             min(x > 0 ? cost[y][x - 1] : INT_MAX,
                                 y > 0 ? cost[y - 1][x] : INT_MAX,
                                 x < CAVE_SIZE - 1 ? cost[y][x + 1] : INT_MAX,
                                 y < CAVE_SIZE - 1 ? cost[y + 1][x] : INT_MAX);
            }
        }
    }
    printf("%d\n", cost[CAVE_SIZE - 1][CAVE_SIZE - 1]);
    return 0;
}
