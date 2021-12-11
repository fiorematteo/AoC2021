#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#define FILE_NAME "input"
//#define FILE_NAME "test"

int main() {
    int board[10][10];
    FILE *file = fopen(FILE_NAME, "r");

    char c;
    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 10; j++) {
            c = fgetc(file);
            if (c == '\n') {
                j--;
                continue;
            }
            board[i][j] = c - '0';
        }
    }
    fclose(file);

    int count = 0;
    for (int days = 1; days <= 1000; days++) {
        for (int i = 0; i < 10; i++) {
            for (int j = 0; j < 10; j++) {
                board[i][j]++;
            }
        }

        bool flash = true;
        while (flash) {
            flash = false;
            for (int i = 0; i < 10; i++) {
                for (int j = 0; j < 10; j++) {
                    if (board[i][j] > 9) {
                        count++;
                        flash = true;
                        if (i < 9 && board[i + 1][j] != 0)
                            board[i + 1][j]++;
                        if (i > 0 && board[i - 1][j] != 0)
                            board[i - 1][j]++;
                        if (j < 9 && board[i][j + 1] != 0)
                            board[i][j + 1]++;
                        if (j > 0 && board[i][j - 1] != 0)
                            board[i][j - 1]++;
                        if (i < 9 && j < 9 && board[i + 1][j + 1] != 0)
                            board[i + 1][j + 1]++;
                        if (i > 0 && j > 0 && board[i - 1][j - 1] != 0)
                            board[i - 1][j - 1]++;
                        if (i > 0 && j < 9 && board[i - 1][j + 1] != 0)
                            board[i - 1][j + 1]++;
                        if (i < 9 && j > 0 && board[i + 1][j - 1] != 0)
                            board[i + 1][j - 1]++;
                        board[i][j] = 0;
                    }
                }
            }
        }
        int countFlash = 0;
        for (int i = 0; i < 10; i++) {
            for (int j = 0; j < 10; j++) {
                if (board[i][j]==0)
                    countFlash++;
            }
        }
        if (countFlash == 100){
            printf("%d\n", days);
            break;
        }
    }
}
