#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

#define PAGE_SIZE 2000
bool page[PAGE_SIZE][PAGE_SIZE];
int xSize = PAGE_SIZE;
int ySize = PAGE_SIZE;

void printfPage() {
    for (int y = 0; y < ySize; y++) {
        for (int x = 0; x < xSize; x++) {
            if (page[y][x]) {
                printf("#");
            } else {
                printf(".");
            }
        }
        printf("\n");
    }
}

void fold(int fx, int fy) {
    if (fx == 0 && fy != 0) {
        for (int y = fy; y < PAGE_SIZE; y++) {
            for (int x = 0; x < PAGE_SIZE; x++) {
                int newY = fy - abs(y - fy);
                if (newY >= 0)
                    page[newY][x] |= page[y][x];
            }
        }
        ySize = fy + 1;
    } else if (fx != 0 && fy == 0) {
        for (int y = 0; y < PAGE_SIZE; y++) {
            for (int x = fx; x < PAGE_SIZE; x++) {
                int newX = fx - abs(x - fx);
                if (newX >= 0)
                    page[y][newX] |= page[y][x];
            }
        }
        xSize = fx + 1;
    } else
        assert(false);
}

int main() {
    for (int i = 0; i < PAGE_SIZE; i++) {
        for (int j = 0; j < PAGE_SIZE; j++) {
            page[i][j] = false;
        }
    }
    FILE *f = fopen(FILE_NAME, "r");

    int fx = 0, fy = 0;
    while (fscanf(f, "%d,%d\n", &fx, &fy) != 0)
        page[fy][fx] = true;

    char *line = NULL;
    size_t len = 0;
    while (getline(&line, &len, f) != -1) {
        if (line[11] == 'x') {
            fx = atoi(line + 13);
            fold(fx, 0);
        } else {
            fy = atoi(line + 13);
            fold(0, fy);
        }
        printf("\n\n");
        printfPage();
    }
}
