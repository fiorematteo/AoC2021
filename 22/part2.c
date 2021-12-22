#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

typedef struct diff diff;
struct diff {
    long x0;
    long x1;
    long y0;
    long y1;
    long z0;
    long z1;
    long counter;
};
diff *diffs;
int diffLen = 0;
bool inRange(long a, long b, long c) { return a <= c && c <= b; }
long min(long a, long b) { return a < b ? a : b; }
long max(long a, long b) { return a > b ? a : b; }

long volume(diff d) {
    assert(d.counter == 1 || d.counter == -1);
    return d.counter *
           labs((d.x1 - d.x0 + 1) * (d.y1 - d.y0 + 1) * (d.z1 - d.z0 + 1));
}

void printDiff(diff d) {
    printf(d.counter > 0 ? "on" : "off");
    printf(" %ld to %ld, %ld to %ld, %ld to %ld -> %ld\n", d.x0, d.x1, d.y0, d.y1,
           d.z0, d.z1, volume(d));
}

bool intersection(diff a, diff b, diff *c) {
    *c = (diff){
        .x0 = max(a.x0, b.x0),
        .x1 = min(a.x1, b.x1),
        .y0 = max(a.y0, b.y0),
        .y1 = min(a.y1, b.y1),
        .z0 = max(a.z0, b.z0),
        .z1 = min(a.z1, b.z1),
        .counter = -a.counter,
    };
    if (c->x0 <= c->x1 && c->y0 <= c->y1 && c->z0 <= c->z1) {
        return true;
    }
    return false;
}

void addDiff(diff d) {
    int tmpLen = diffLen;
    for (int i = 0; i < tmpLen; i++) {
        diff tmp;
        if (intersection(diffs[i], d, &tmp)) {
            diffs = realloc(diffs, (diffLen + 1) * sizeof(diff));
            diffs[diffLen++] = tmp;
        }
    }
    if (d.counter == 1) {
        diffs = realloc(diffs, (diffLen + 1) * sizeof(diff));
        diffs[diffLen++] = d;
    }
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    char *line = NULL;
    size_t len = 0;
    while (getline(&line, &len, file) != EOF) {
        int x0, x1, y0, y1, z0, z1;
        sscanf(line, "on x=%d..%d,y=%d..%d,z=%d..%d", &x0, &x1, &y0, &y1, &z0,
               &z1);
        sscanf(line, "off x=%d..%d,y=%d..%d,z=%d..%d", &x0, &x1, &y0, &y1, &z0,
               &z1);
        diff d = {x0, x1, y0, y1, z0, z1, line[1] == 'n' ? 1 : -1};
        addDiff(d);
        printDiff(d);
    }
    long sum = 0;
    long positive = 0;
    long negative = 0;
    for (int i = 0; i < diffLen; i++) {
        if (diffs[i].counter == 1) {
            positive += volume(diffs[i]);
        } else {
            negative += volume(diffs[i]);
        }
        sum += volume(diffs[i]);
    }
    printf("positive: %ld\n", positive);
    printf("negative: %ld\n", negative);
    printf("sum: %ld\n", sum);
}
