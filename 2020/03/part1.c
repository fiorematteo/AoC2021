#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FILE_NAME "input"
//#define FILE_NAME "test"

#define BOARD_WIDTH 31
#define BOARD_HEIGHT 323

int simulate(bool board[BOARD_HEIGHT][BOARD_WIDTH], int slope_x, int slope_y) {
    int x_slope = 3;
    int y_slope = 1;
    int x = 0;
    int y = 0;
    int tree_counter;
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

    printf("%d", simulate(board, 3, 1));
}
