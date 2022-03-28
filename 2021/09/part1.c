#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FILE_NAME "input"
#define SIDE_LEN 100
//#define FILE_NAME "test"
//#define SIDE_LEN 10

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

    for (i = 0; i < SIDE_LEN; i++) {
        for (j = 0; j < SIDE_LEN; j++) {
            printf("%d ", board[i][j]);
        }
        printf("\n");
    }

    int sum = 0;
    for (i = 0; i < SIDE_LEN; i++) {
        for (j = 0; j < SIDE_LEN; j++) {
            if (i == 0 || board[i][j] < board[i - 1][j])
                if (j == 0 || board[i][j] < board[i][j - 1])
                    if (i == SIDE_LEN - 1 || board[i][j] < board[i + 1][j])
                        if (j == SIDE_LEN - 1 || board[i][j] < board[i][j + 1]) {
                            sum += board[i][j] + 1;
                        }
        }
    }
    printf("total sum: %d\n", sum);
}
