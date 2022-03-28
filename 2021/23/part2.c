#include <assert.h>
#include <limits.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

int rooms[4] = {3, 5, 7, 9};
int typeCost[4] = {1, 10, 100, 1000};
int stoppable[8] = {1, 2, 4, 6, 8, 10, 11, 12};

bool isAmphipod(char c) { return c == 'A' || c == 'B' || c == 'C' || c == 'D'; }

bool inOwnRoom(char **data, int x, int y) {
    if (!isAmphipod(data[y][x]))
        return false;
    if (x == rooms[data[y][x] - 'A'])
        return true;
    return false;
}

bool roomComplete(char **data, int x) {
    for (int y = 2; y < 6; y++)
        if (!inOwnRoom(data, x, y))
            return false;
    return true;
}

bool roomsComplete(char **data) {
    for (int i = 3; i <= 9; i += 2)
        if (!roomComplete(data, i))
            return false;
    return true;
}

bool isBlocking(char **data, int x, int y) {
    for (int i = y + 1; i < 6; i++)
        if (isAmphipod(data[i][x]) && !inOwnRoom(data, x, i))
            return true;
    return false;
}

bool isBlocked(char **data, int x, int y) {
    if (y < 3)
        return false;
    return data[y - 1][x] != '.';
}

bool canMove(char **data, int x, int y) {
    return ((!inOwnRoom(data, x, y) || isBlocking(data, x, y)) &&
            !isBlocked(data, x, y));
}

bool isRoomEmpty(char **data, int x) {
    for (int y = 2; y < 6; y++)
        if (data[y][x] != '.')
            return false;
    return true;
}

bool hasRoomAvailable(char **data, int x, int y) {
    char amphipod = data[y][x];
    int room = rooms[amphipod - 'A'];
    if (isRoomEmpty(data, x))
        return true;
    for (int i = 2; i < 6; i++)
        if (data[i][room] != '.' && !inOwnRoom(data, room, i))
            return false;
    return true;
}

bool isPathEmpty(char **data, int x, int targetX) {
    while (x != targetX) {
        x += (targetX > x) ? 1 : -1;
        if (data[1][x] != '.')
            return false;
    }
    return true;
}

int moveinPos(char **data, int room) {
    for (int y = 5; y >= 0; y--)
        if (data[y][room] == '.')
            return y;
    assert(false);
}

char **copy(char **data) {
    char **newData = malloc(sizeof(char *) * 7);
    for (int y = 0; y < 7; y++) {
        newData[y] = malloc(sizeof(char) * 14);
        memcpy(newData[y], data[y], 14);
    }
    return newData;
}

int moveCost(char **data, int x, int y, int roomX, int roomY) {
    return ((y - 1) + abs(x - roomX) + (roomY - 1)) *
           typeCost[data[y][x] - 'A'];
}

int move(char **data, int x, int y, int roomX, int roomY) {
    int cost = moveCost(data, x, y, roomX, roomY);
    char tmp = data[y][x];
    data[y][x] = data[roomY][roomX];
    data[roomY][roomX] = tmp;
    return cost;
}

void printData(char **data) {
    for (int y = 0; y < 7; y++) {
        for (int x = 0; x < 13; x++) {
            printf("%c", data[y][x]);
        }
        printf("\n");
    }
    printf("\n");
}

long solve(char **data) {
    // printData(data);
    if (roomsComplete(data))
        return 0;

    long costs[100];
    int costs_len = 0;

    for (int y = 1; y < 7; y++) {
        for (int x = 1; x < 14; x++) {
            bool amphipod = isAmphipod(data[y][x]);
            if (!amphipod)
                continue;
            if (canMove(data, x, y)) {
                int room = rooms[data[y][x] - 'A'];
                if (hasRoomAvailable(data, x, y) &&
                    isPathEmpty(data, x, room)) {
                    char **newData = copy(data);
                    long c =
                        move(newData, x, y, room, moveinPos(newData, room));
                    long cost = solve(newData);
                    for (int i = 0; i < 5; i++)
                        free(newData[i]);
                    free(newData);
                    assert(costs_len < 90);
                    if (cost >= 0)
                        costs[costs_len++] = c + cost;
                } else if (y > 1) {
                    for (int i = 0; i < 8; i++) {
                        if (!isPathEmpty(data, x, stoppable[i]))
                            continue;
                        char **newData = copy(data);
                        long c = move(newData, x, y, stoppable[i], 1);
                        long cost = solve(newData);
                        for (int k = 0; k < 5; k++)
                            free(newData[k]);
                        free(newData);
                        assert(costs_len < 90);
                        if (cost >= 0)
                            costs[costs_len++] = c + cost;
                    }
                }
            }
        }
    }
    assert(costs_len <= 90);
    long result = 100000000000;
    if (costs_len == 0)
        return result;
    for (int i = 0; i < costs_len; i++) {
        if (costs[i] < result)
            result = costs[i];
    }
    return result;
}

int main() {
    FILE *fs = fopen(FILE_NAME, "r");
    char **board;
    board = malloc(sizeof(char *) * 7);
    for (int y = 0; y < 7; y++) {
        board[y] = malloc(sizeof(char) * 14);
    }
    char *line = NULL;
    size_t len = 0;
    int lineIndex = 0;
    while (getline(&line, &len, fs) != -1) {
        for (int i = 0; i < 13; i++) {
            if (i < (int)strlen(line) - 1) {
                board[lineIndex][i] = line[i];
            } else
                board[lineIndex][i] = ' ';
        }
        lineIndex++;
    }
    fclose(fs);
    memcpy(board[5], board[3], sizeof(char) * 14);
    memcpy(board[6], board[4], sizeof(char) * 14);
    char *add = "  #D#C#B#A#";
    strncpy(board[3], add, 10);
    add = "  #D#B#A#C#";
    strncpy(board[4], add, 10);
    printf("%ld\n", solve(board));
}
