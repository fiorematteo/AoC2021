#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define FILE_NAME "input"
#define GRID_WIDTH 97
#define GRID_HEIGHT 92
#else
#define FILE_NAME "test"
#define GRID_WIDTH 10
#define GRID_HEIGHT 10
#endif

int count_around(int x, int y, char grid[GRID_HEIGHT][GRID_WIDTH]) {
    int total = 0;
    for (int i = y - 1; i <= y + 1; i++) {
        for (int j = x - 1; j <= x + 1; j++) {
            if (i == y && j == x)
                continue;
            if (i < 0 || j < 0 || i >= GRID_HEIGHT || j >= GRID_WIDTH)
                continue;
            if (grid[i][j] == '#')
                total++;
        }
    }
    return total;
}

bool compare_grid(char grid[GRID_HEIGHT][GRID_WIDTH],
                  char copy[GRID_HEIGHT][GRID_WIDTH]) {
    for (int i = 0; i < GRID_HEIGHT; i++) {
        for (int j = 0; j < GRID_WIDTH; j++) {
            if (grid[i][j] != copy[i][j])
                return false;
        }
    }
    return true;
}

void print_board(char grid[GRID_HEIGHT][GRID_WIDTH]) {
    for (int i = 0; i < GRID_HEIGHT; i++) {
        for (int j = 0; j < GRID_WIDTH; j++) {
            printf("%c", grid[i][j]);
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

    char grid[GRID_HEIGHT][GRID_WIDTH];
    memset(grid, '.', sizeof(grid));
    char *line = NULL;
    size_t len = 0;
    int line_index = 0;
    while (getline(&line, &len, file) != EOF) {
        line[strlen(line) - 1] = '\0';
        strncpy(grid[line_index++], line, GRID_WIDTH);
    }

    char grid_copy[GRID_HEIGHT][GRID_WIDTH];
    memset(grid_copy, '.', sizeof(grid_copy));
    while (true) {
        for (int i = 0; i < GRID_HEIGHT; i++) {
            for (int j = 0; j < GRID_WIDTH; j++) {
                int count = count_around(j, i, grid);
                switch (grid[i][j]) {
                case 'L':
                    if (!count)
                        grid_copy[i][j] = '#';
                    break;
                case '#':
                    if (count >= 4)
                        grid_copy[i][j] = 'L';
                case '.':
                    break;
                default:
                    assert(false);
                }
            }
        }
        if (compare_grid(grid, grid_copy))
            break;
        memcpy(grid, grid_copy, sizeof(grid));
    }
    int occupied = 0;
    for (int i = 0; i < GRID_HEIGHT; i++) {
        for (int j = 0; j < GRID_WIDTH; j++) {
            if (grid_copy[i][j] == '#')
                occupied++;
        }
    }
    printf("%d\n", occupied);
}
