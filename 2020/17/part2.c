#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define FILE_NAME "input"
#else
#define FILE_NAME "test"
#endif

#define MAX_X 50
#define MAX_Y 50
#define MAX_Z 50
#define MAX_W 50

#define START_X MAX_X / 2
#define START_Y MAX_Y / 2
#define START_Z MAX_Z / 2
#define START_W MAX_W / 2

bool dimension[MAX_X][MAX_Y][MAX_Z][MAX_W];
bool dimension_clone[MAX_X][MAX_Y][MAX_Z][MAX_W];

int count_neighbours(int x, int y, int z, int w) {
    int count = 0;
    for (int dx = -1; dx <= 1; dx++) {
        if (x + dx < 0 || x + dx >= MAX_X)
            continue;
        for (int dy = -1; dy <= 1; dy++) {
            if (y + dy < 0 || y + dy >= MAX_Y)
                continue;
            for (int dz = -1; dz <= 1; dz++) {
                if (z + dz < 0 || z + dz >= MAX_Z)
                    continue;
                for (int dw = -1; dw <= 1; dw++) {
                    if (w + dw < 0 || w + dw >= MAX_W)
                        continue;
                    if (dx == 0 && dy == 0 && dz == 0 && dw == 0)
                        continue;
                    assert(x + dx >= 0 && x + dx < MAX_X);
                    assert(y + dy >= 0 && y + dy < MAX_Y);
                    assert(z + dz >= 0 && z + dz < MAX_Z);
                    assert(w + dw >= 0 && w + dw < MAX_W);
                    if (dimension[x + dx][y + dy][z + dz][w + dw])
                        count++;
                }
            }
        }
    }
    return count;
}

void print_total(bool d[MAX_X][MAX_Y][MAX_Z][MAX_W]) {
    int total_cubes = 0;
    for (int x = 0; x < MAX_X; x++) {
        for (int y = 0; y < MAX_Y; y++) {
            for (int z = 0; z < MAX_Z; z++) {
                for (int w = 0; w < MAX_W; w++) {
                    if (d[x][y][z][w])
                        total_cubes++;
                }
            }
        }
    }
    printf("total_cubes = %d\n", total_cubes);
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    memset(dimension, 0, sizeof(dimension));
    memset(dimension_clone, 0, sizeof(dimension));
    int x = 0;
    int y = 0;
    int z = 0;
    int w = 0;
    char c;
    while (true) {
        c = getc(file);
        if (c == '#') {
            dimension[START_X + x][START_Y + y][START_Z + z][START_W + w] =
                true;
            x++;
        } else if (c == '.') {
            dimension[START_X + x][START_Y + y][START_Z + z][START_W + w] =
                false;
            x++;
        } else if (c == '\n') {
            x = 0;
            y++;
        } else if (c == EOF)
            break;
    }

    int iterations = 0;
    while (iterations < 7) {
        memcpy(dimension_clone, dimension, sizeof(dimension));

        int total = 0;
        for (int x = 0; x < MAX_X; x++) {
            for (int y = 0; y < MAX_Y; y++) {
                for (int z = 0; z < MAX_Z; z++) {
                    for (int w = 0; w < MAX_W; w++) {
                        int count = count_neighbours(x, y, z, w);
                        if (dimension[x][y][z] && (count != 2 && count != 3))
                            dimension_clone[x][y][z][w] = false;
                        else if (!dimension[x][y][z][w] && count == 3) {
                            dimension_clone[x][y][z][w] = true;
                        }
                        if (dimension[x][y][z][w])
                            total++;
                    }
                }
            }
        }
        iterations++;
        memcpy(dimension, dimension_clone, sizeof(dimension));

        printf("iteration %d total %d\n", iterations, total);
    }
}
