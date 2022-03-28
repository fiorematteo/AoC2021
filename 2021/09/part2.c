#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FILE_NAME "input"
#define SIDE_LEN 100
//#define FILE_NAME "test"
//#define SIDE_LEN 10
//#define TEST_LEN 5

typedef struct {
    int x;
    int y;
} point;

typedef struct {
    point low;
    point *points;
    int len;
} basin;

bool visited[SIDE_LEN][SIDE_LEN];

basin mergeBasins(basin b1, basin b2) {
    basin merged;
    merged.low.x = b1.low.x;
    merged.low.y = b1.low.y;
    merged.len = b1.len + b2.len;
    merged.points = malloc(sizeof(point) * merged.len);
    memcpy(merged.points, b1.points, sizeof(point) * b1.len);
    memcpy(merged.points + b1.len, b2.points, sizeof(point) * b2.len);
    return merged;
}

basin explore(int board[SIDE_LEN][SIDE_LEN], point p) {
    if (p.x == -1 || p.x == SIDE_LEN || p.y == -1 || p.y == SIDE_LEN ||
        board[p.y][p.x] == 9) {
        return (basin){.low = p, .points = NULL, .len = 0};
    }

    visited[p.x][p.y] = true;
    basin b = {.low = p, .points = malloc(sizeof(point)), .len = 1};
    b.points[0] = p;
    if (!visited[p.x - 1][p.y])
        b = mergeBasins(b, explore(board, (point){.x = p.x - 1, .y = p.y}));
    if (!visited[p.x + 1][p.y])
        b = mergeBasins(b, explore(board, (point){.x = p.x + 1, .y = p.y}));
    if (!visited[p.x][p.y - 1])
        b = mergeBasins(b, explore(board, (point){.x = p.x, .y = p.y - 1}));
    if (!visited[p.x][p.y + 1])
        b = mergeBasins(b, explore(board, (point){.x = p.x, .y = p.y + 1}));
    return b;
};

int main() {
    FILE *file = fopen(FILE_NAME, "r");

    int board[SIDE_LEN][SIDE_LEN];

    int i = 0;
    int j = 0;
    char c;
    while ((c = fgetc(file)) != EOF) {
        if (c == '\n') {
            i++;
            j = 0;
        } else {
            board[i][j] = c - '0';
            j++;
        }
    }

    point points[1000];
    int pointCount = 0;

    for (i = 0; i < SIDE_LEN; i++) {
        for (j = 0; j < SIDE_LEN; j++) {
            if (i == 0 || board[i][j] < board[i - 1][j])
                if (j == 0 || board[i][j] < board[i][j - 1])
                    if (i == SIDE_LEN - 1 || board[i][j] < board[i + 1][j])
                        if (j == SIDE_LEN - 1 ||
                            board[i][j] < board[i][j + 1]) {
                            points[pointCount] = (point){.x = j, .y = i};
                            pointCount++;
                        }
        }
    }

    basin basins[1000];
    int basinCount = 0;
    for (i = 0; i < pointCount; i++) {
        basins[basinCount] = explore(board, points[i]);
        basinCount++;
    }

    int max1 = 0;
    int max2 = 0;
    int max3 = 0;

    for (i = 0; i < basinCount; i++) {
        if (basins[i].len > max1) {
            max3 = max2;
            max2 = max1;
            max1 = basins[i].len;
        } else if (basins[i].len > max2) {
            max3 = max2;
            max2 = basins[i].len;
        } else if (basins[i].len > max3) {
            max3 = basins[i].len;
        }
    }

    printf("soluzione: %d\n", max1 * max2 * max3);
}
