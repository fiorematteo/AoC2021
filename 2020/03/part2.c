#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define FILE_NAME "input"
#define BOARD_WIDTH 31
#define BOARD_HEIGHT 323

#else
#define FILE_NAME "test"
#define BOARD_WIDTH 11
#define BOARD_HEIGHT 11
#endif

int simulate(const bool board[BOARD_HEIGHT][BOARD_WIDTH], int x_slope,
             int y_slope) {
    int x = x_slope;
    int y = y_slope;
    int tree_counter = 0;
    while (y < BOARD_HEIGHT) {
        if (board[y][x])
            tree_counter++;
        x = (x + x_slope) % BOARD_WIDTH;
        y += y_slope;
    }
    return tree_counter;
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    bool board[BOARD_HEIGHT][BOARD_WIDTH];
    for (int i = 0; i < BOARD_HEIGHT; i++) {
        for (int j = 0; j < BOARD_WIDTH; j++) {
            board[i][j] = false;
        }
    }
    char ch;
    int x = 0;
    int y = 0;
    while ((ch = getc(file)) != EOF) {
        switch (ch) {
        case '#':
            board[y][x] = true;
            x++;
            break;
        case '.':
            board[y][x] = false;
            x++;
            break;
        case '\n':
            x = 0;
            y++;
            break;
        default:
            return 1;
            break;
        }
    }

    long int a = simulate(board, 1, 1);
    long int b = simulate(board, 3, 1);
    long int c = simulate(board, 5, 1);
    long int d = simulate(board, 7, 1);
    long int e = simulate(board, 1, 2);
    printf("%ld * %ld * %ld * %ld * %ld = %ld\n", a, b, c, d, e, a * b * c * d * e);
}
