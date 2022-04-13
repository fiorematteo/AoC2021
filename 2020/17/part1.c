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

#define MAX_X 100
#define MAX_Y 100
#define MAX_Z 100

#define START_X MAX_X / 2
#define START_Y MAX_Y / 2
#define START_Z MAX_Z / 2

bool dimension[MAX_X][MAX_Y][MAX_Z];
bool dimension_clone[MAX_X][MAX_Y][MAX_Z];

int count_neighbours(int x, int y, int z) {
    int count = 0;
    for (int dx = -1; dx <= 1; dx++) {
        for (int dy = -1; dy <= 1; dy++) {
            for (int dz = -1; dz <= 1; dz++) {
                if (x + dx < 0 || x + dx >= MAX_X || y + dy < 0 ||
                    y + dy >= MAX_Y || z + dz < 0 || z + dz >= MAX_Z)
                    continue;
                if (dx == 0 && dy == 0 && dz == 0)
                    continue;
                assert(x + dx >= 0 && x + dx < MAX_X);
                assert(y + dy >= 0 && y + dy < MAX_Y);
                assert(z + dz >= 0 && z + dz < MAX_Z);
                if (dimension[x + dx][y + dy][z + dz])
                    count++;
            }
        }
    }
    return count;
}

void print_total(bool d[MAX_X][MAX_Y][MAX_Z]) {
    int total_cubes = 0;
    for (int x = 0; x < MAX_X; x++) {
        for (int y = 0; y < MAX_Y; y++) {
            for (int z = 0; z < MAX_Z; z++) {
                if (d[x][y][z])
                    total_cubes++;
            }
        }
    }
    printf("total_cubes = %d\n", total_cubes);
}

void print_slice(bool d[MAX_X][MAX_Y][MAX_Z], int z) {
    printf("Z value=%d\n", z);
    int off = 8;
    for (int x = START_X - off; x < START_X + off; x++) {
        for (int y = START_Y - off; y < START_Y + off; y++) {
            printf(d[y][x][z] ? "#" : ".");
        }
        printf("\n");
    }
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
    char c;
    while (true) {
        c = getc(file);
        if (c == '#') {
            dimension[START_X + x][START_Y + y][START_Z + z] = true;
            x++;
        } else if (c == '.') {
            dimension[START_X + x][START_Y + y][START_Z + z] = false;
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
        printf("iteration %d ", iterations);
        print_total(dimension);
        // for (int z = START_Z - 2; z < START_Z + 3; z++) {
        //    print_slice(dimension, z);
        //}

        for (int x = 0; x < MAX_X; x++) {
            for (int y = 0; y < MAX_Y; y++) {
                for (int z = 0; z < MAX_Z; z++) {
                    int count = count_neighbours(x, y, z);
                    if (dimension[x][y][z] && (count != 2 && count != 3))
                        dimension_clone[x][y][z] = false;
                    else if (!dimension[x][y][z] && count == 3) {
                        dimension_clone[x][y][z] = true;
                    }
                }
            }
        }
        iterations++;
        memcpy(dimension, dimension_clone, sizeof(dimension));
    }
}
