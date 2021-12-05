#include <stdio.h>
#include <stdlib.h>

//#define FILE_NAME "test"
//#define BOARD_SIZE 10
#define FILE_NAME "input"
#define BOARD_SIZE 1000

int main() {
    int board[BOARD_SIZE][BOARD_SIZE];
    for (int i = 0; i < BOARD_SIZE; i++) {
        for (int j = 0; j < BOARD_SIZE; j++) {
            board[i][j] = 0;
        }
    }

    FILE *f = fopen(FILE_NAME, "r");
    int x1, y1, x2, y2;
    while (fscanf(f, "%d,%d -> %d,%d\n", &x1, &y1, &x2, &y2) != EOF) {
        board[y1][x1]++;
        while (x1 != x2 || y1 != y2) {
            if (x1 < x2) {
                x1++;
            } else if (x1 > x2) {
                x1--;
            }
            if (y1 < y2) {
                y1++;
            } else if (y1 > y2) {
                y1--;
            }
            board[y1][x1]++;
        }
    }
    fclose(f);
    // print board
    for (int i = 0; i < BOARD_SIZE; i++) {
        for (int j = 0; j < BOARD_SIZE; j++) {
            if (board[i][j] == 0) {
                printf(".");
            } else {
                printf("%d", board[i][j]);
            }
        }
        printf("\n");
    }
    // check board for number equal to 2
    int count = 0;
    for (int i = 0; i < BOARD_SIZE; i++) {
        for (int j = 0; j < BOARD_SIZE; j++) {
            if (board[i][j] >= 2) {
                count++;
            }
        }
    }
    printf("count: %d\n", count);
}
