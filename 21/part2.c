#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

#define GOAL 21

typedef struct wins wins;
struct wins {
    int64_t p1;
    int64_t p2;
};

int sums[27];

wins cache[21][21][10][10][2];

wins pG(int score1, int score2, int pos1, int pos2, bool turn) {
    if (score1 >= GOAL)
        return (wins){1, 0};
    if (score2 >= GOAL)
        return (wins){0, 1};

    wins cached = cache[score1][score2][pos1][pos2][turn];
    if (cached.p1 != -1 && cached.p2 != -1)
        return cached;

    wins total = {0, 0};
    wins recursive;
    for (int i = 0; i < 27; i++) {
        int sum = sums[i];
        if (turn) {
            int newPos = (pos1 + sum) % 10;
            int newScore = score1 + newPos + 1;
            recursive = pG(newScore, score2, newPos, pos2, !turn);
        } else {
            int newPos = (pos2 + sum) % 10;
            int newScore = score2 + newPos + 1;
            recursive = pG(score1, newScore, pos1, newPos, !turn);
        }
        total.p1 += recursive.p1;
        total.p2 += recursive.p2;
    }
    if (cached.p1 == -1 && cached.p2 == -1)
        cache[score1][score2][pos1][pos2][turn] = total;
    return total;
}

int main() {
    FILE *fs = fopen(FILE_NAME, "r");
    char *line = NULL;
    size_t len = 0;
    getline(&line, &len, fs);
    int pos1 = line[strlen(line) - 2] - '0';
    getline(&line, &len, fs);
    int pos2 = line[strlen(line) - 2] - '0';
    fclose(fs);
    printf("pos1: %d pos2: %d\n", pos1, pos2);

    int index = 0;
    for (int i = 1; i <= 3; i++)
        for (int j = 1; j <= 3; j++)
            for (int k = 1; k <= 3; k++)
                sums[index++] = i + j + k;

    for (int i = 0; i < 21; i++)
        for (int j = 0; j < 21; j++)
            for (int k = 0; k < 10; k++)
                for (int l = 0; l < 10; l++) {
                    cache[i][j][k][l][0] = (wins){-1, -1};
                    cache[i][j][k][l][1] = (wins){-1, -1};
                }

    pos1--;
    pos2--;
    wins win = pG(0, 0, pos1, pos2, true);
    printf("player1: %ld\n", win.p1);
    printf("player2: %ld\n", win.p2);
}
