#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FILE_NAME "test"
//#define FILE_NAME "input"

#define CAVE_SIZE 10
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

    data[0][0] = 0;
    for (int i = 0; i < 1000; i++) {
        for (int y = 0; y < CAVE_SIZE; y++) {
            for (int x = 0; x < CAVE_SIZE; x++) {
                if (x == 0 && y == 0)
                    continue;
                cost[x][y] = data[x][y] +
                             min(x > 0 ? cost[y][x - 1] : INT_MAX,
                                 y > 0 ? cost[y - 1][x] : INT_MAX,
                                 x < CAVE_SIZE - 1 ? cost[y][x + 1] : INT_MAX,
                                 y < CAVE_SIZE - 1 ? cost[y + 1][x] : INT_MAX);
            }
        }
    }
    for (int x = 0; x < CAVE_SIZE; x++) {
        for (int y = 0; y < CAVE_SIZE; y++) {
            printf(" %2d", cost[y][x]);
        }
        printf("\n");
    }
    printf("%d\n", cost[CAVE_SIZE - 1][CAVE_SIZE - 1]);
    return 0;
}
