#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

#define MAX_CAVES 50
bool graph[MAX_CAVES][MAX_CAVES];
int graphSize = 0;

char *caves[MAX_CAVES];
bool visited[MAX_CAVES];

int end, start;

int addCave(char *cave) {
    for (int i = 0; i < graphSize; i++) {
        if (strcmp(caves[i], cave) == 0) {
            return i;
        }
    }
    caves[graphSize] = malloc(strlen(cave) + 1);
    strcpy(caves[graphSize], cave);
    visited[graphSize] = false;
    return graphSize++;
}

bool isBig(int i) {
    if ('A' <= caves[i][0] && caves[i][0] <= 'Z') {
        return true;
    }
    return false;
}

void solve(int curr, bool twice, int *count) {
    if (curr == end) {
        (*count)++;
        return;
    }

    for (int i = 0; i < graphSize; i++) {
        if (graph[curr][i]) {
            if (isBig(i)) {
                solve(i, twice, count);

            } else if (!visited[i]) {
                visited[i] = true;
                solve(i, twice, count);
                visited[i] = false;
            } else if (!twice && i != start) {
                solve(i, true, count);
            }
        }
    }
}

void drawGraph() {
    int i, j;
    for (i = 0; i < graphSize; i++) {
        for (j = 0; j < graphSize; j++)
            printf("%d ", graph[i][j]);
        printf("\n");
    }
}

int main() {
    char *line = NULL;
    size_t len = 0;
    FILE *f = fopen(FILE_NAME, "r");
    while (getline(&line, &len, f) != -1) {
        char *cave1 = strtok(line, "-");
        char *cave2 = strtok(NULL, "\n");
        int caveIndex1 = addCave(cave1);
        int caveIndex2 = addCave(cave2);
        graph[caveIndex1][caveIndex2] = true;
        graph[caveIndex2][caveIndex1] = true;
    }

    for (int i = 0; i < graphSize; i++) {
        if (strcmp(caves[i], "start") == 0)
            start = i;
        if (strcmp(caves[i], "end") == 0)
            end = i;
    }
    drawGraph();

    memset(visited, false, sizeof(visited));
    visited[start] = true;
    int count = 0;
    solve(start, false, &count);
    printf("paths: %d\n", count);
}
