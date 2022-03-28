#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

#define MAX_CAVES 50
bool tunnels[MAX_CAVES][MAX_CAVES];
int tunnelSize = 0;

char *caves[MAX_CAVES];
bool visited[MAX_CAVES];

int end, start;

int addCave(char *cave) {
    for (int i = 0; i < tunnelSize; i++)
        if (strcmp(caves[i], cave) == 0)
            return i;
    caves[tunnelSize] = malloc(strlen(cave) + 1);
    strcpy(caves[tunnelSize], cave);
    visited[tunnelSize] = false;
    return tunnelSize++;
}

void solve(int curr, bool twice, int *count) {
    if (curr == end) {
        (*count)++;
        return;
    }

    for (int i = 0; i < tunnelSize; i++) {
        if (tunnels[curr][i]) {
            if (caves[i][0] < 'a') {
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

void drawTunnels() {
    for (int i = 0; i < tunnelSize; i++) {
        for (int j = 0; j < tunnelSize; j++)
            printf("%d ", tunnels[i][j]);
        printf("\n");
    }
}

#define setStart(i) if (strcmp(caves[i], "start") == 0) start = i;
#define setEnd(i) if (strcmp(caves[i], "end") == 0) end = i;

int main() {
    char *line = NULL;
    size_t len = 0;
    FILE *f = fopen(FILE_NAME, "r");
    while (getline(&line, &len, f) != -1) {
        int caveIndex1 = addCave(strtok(line, "-"));
        int caveIndex2 = addCave(strtok(NULL, "\n"));
        tunnels[caveIndex1][caveIndex2] = true;
        tunnels[caveIndex2][caveIndex1] = true;
        setStart(caveIndex1);
        setStart(caveIndex2);
        setEnd(caveIndex1);
        setEnd(caveIndex2);
    }

    drawTunnels();
    memset(visited, false, sizeof(visited));
    visited[start] = true;
    int count = 0;
    solve(start, false, &count);
    printf("paths: %d\n", count);
}
