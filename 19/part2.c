#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

#define BIS 1000

typedef struct scanner scanner;
typedef struct vec3 vec3;
struct vec3 {
    int x;
    int y;
    int z;
};
struct scanner {
    vec3 *beacons;
    int len;
    int *distances;
    int distancesLen;
};

scanner *scanners;
vec3 *goodBeacons;
int goodBeaconsLen;

void printGood() {
    for (int i = 0; i < goodBeaconsLen; i++) {
        printf("x:%d y:%d z:%d\n", goodBeacons[i].x, goodBeacons[i].y,
               goodBeacons[i].z);
    }
}

void computeDistances(scanner *s) {
    s->distancesLen = 0;
    for (int i = 0; i < s->len; i++) {
        for (int j = 0; j < s->len; j++) {
            s->distances[s->distancesLen++] =
                pow(s->beacons[i].x - s->beacons[j].x, 2) +
                pow(s->beacons[i].y - s->beacons[j].y, 2) +
                pow(s->beacons[i].z - s->beacons[j].z, 2);
        }
    }
}

int sign[8][3] = {{1, 1, 1},  {-1, 1, 1},  {1, -1, 1},  {-1, -1, 1},
                  {1, 1, -1}, {-1, 1, -1}, {1, -1, -1}, {-1, -1, -1}};

int perm[6][3] = {{0, 1, 2}, {0, 2, 1}, {1, 0, 2},
                  {1, 2, 0}, {2, 1, 0}, {2, 0, 1}};

scanner rotate(scanner scan, int s, int p) {
    scanner newScan;
    newScan.len = scan.len;
    newScan.beacons = malloc(sizeof(vec3) * scan.len);
    newScan.distances = malloc(sizeof(int) * scan.distancesLen);
    memcpy(newScan.beacons, scan.beacons, sizeof(vec3) * scan.len);
    memcpy(newScan.distances, scan.distances, sizeof(int) * scan.distancesLen);
    for (int i = 0; i < scan.len; i++) {
        int values[3] = {scan.beacons[i].x, scan.beacons[i].y,
                         scan.beacons[i].z};
        newScan.beacons[i].x = values[perm[p][0]] * sign[s][0];
        newScan.beacons[i].y = values[perm[p][1]] * sign[s][1];
        newScan.beacons[i].z = values[perm[p][2]] * sign[s][2];
    }
    return newScan;
}

vec3 scannerPositions[100];
int scannerPositionsLen = 0;

vec3 toAdd[1000];
int toAddLen;
bool match(scanner s1, scanner s2) {
    for (int i = 0; i < s1.len; i++) {
        for (int j = 0; j < s2.len; j++) {
            toAddLen = 0;
            int count = 0;
            int dx = s1.beacons[i].x - s2.beacons[j].x;
            int dy = s1.beacons[i].y - s2.beacons[j].y;
            int dz = s1.beacons[i].z - s2.beacons[j].z;
            for (int k = 0; k < s1.len; k++) {
                for (int l = 0; l < s2.len; l++) {
                    if (s1.beacons[k].x == s2.beacons[l].x + dx &&
                        s1.beacons[k].y == s2.beacons[l].y + dy &&
                        s1.beacons[k].z == s2.beacons[l].z + dz) {
                        count++;
                    }
                }
            }
            if (count >= 12) {
                scannerPositions[scannerPositionsLen++] = (vec3){dx, dy, dz};
                for (int p = 0; p < s2.len; p++) {
                    s2.beacons[p].x += dx;
                    s2.beacons[p].y += dy;
                    s2.beacons[p].z += dz;
                    toAdd[toAddLen++] = s2.beacons[p];
                }
                return true;
            }
        }
    }
    return false;
    assert("unreachable" && 0);
}

void merge() {
    for (int i = 0; i < toAddLen; i++) {
        bool flag = false;
        for (int j = 0; j < scanners[0].len; j++) {
            if (scanners[0].beacons[j].x == toAdd[i].x &&
                scanners[0].beacons[j].y == toAdd[i].y &&
                scanners[0].beacons[j].z == toAdd[i].z) {
                flag = true;
            }
        }
        if (!flag) {
            scanners[0].beacons = realloc(
                scanners[0].beacons, sizeof(vec3) * (scanners[0].len + 10));
            scanners[0].beacons[scanners[0].len++] = toAdd[i];
        }
    }
}

int manhattan(vec3 a, vec3 b) {
    return abs(a.x - b.x) + abs(a.y - b.y) + abs(a.z - b.z);
}

int main() {
    scanners = malloc(sizeof(scanner) * 1000);
    int scannersLen = -1;
    FILE *fs = fopen(FILE_NAME, "r");
    char *line = NULL;
    size_t len = 0;
    while (getline(&line, &len, fs) != EOF) {
        if (strncmp(line, "---", 3) == 0) {
            scannersLen++;
            scanners[scannersLen].len = 0;
            scanners[scannersLen].beacons = malloc(sizeof(vec3) * BIS);
            scanners[scannersLen].distances = malloc(sizeof(int) * BIS * BIS);
            scanners[scannersLen].distancesLen = 0;
            continue;
        }
        if (line[0] == '\n') {
            continue;
        }
        line[strlen(line) - 1] = '\0';
        sscanf(line, "%d,%d,%d",
               &scanners[scannersLen].beacons[scanners[scannersLen].len].x,
               &scanners[scannersLen].beacons[scanners[scannersLen].len].y,
               &scanners[scannersLen].beacons[scanners[scannersLen].len].z);
        scanners[scannersLen].len++;
    }
    fclose(fs);
    scannersLen++;
    for (int i = 0; i < scannersLen; i++) {
        computeDistances(&scanners[i]);
    }

    bool validScanner[scannersLen];
    memset(validScanner, true, sizeof(validScanner));
    int validLen = scannersLen;

    while (validLen > 1) {
        computeDistances(&scanners[0]);
        for (int i = 1; i < scannersLen; i++) {
            if (!validScanner[i])
                continue;
            int count = 0;
            for (int j = 0; j < scanners[i].len * scanners[i].len; j++) {
                for (int k = 0; k < scanners[0].len * scanners[0].len; k++) {
                    if (scanners[i].distances[j] == scanners[0].distances[k] &&
                        scanners[i].distances[j] != 0 &&
                        scanners[0].distances[k] != 0) {
                        count++;
                    }
                }
            }
            printf("count: %d\n", count);
            if (count >= 12) {
                for (int s = 0; s < 8 && validScanner[i]; s++) {
                    for (int p = 0; p < 6; p++) {
                        scanner newScan = rotate(scanners[i], s, p);
                        if (match(scanners[0], newScan)) {
                            merge();
                            validScanner[i] = false;
                            validLen--;
                        }
                    }
                }
            }
        }
    }
    printf("%d\n", scanners[0].len);

    int max = 0;
    for (int i = 0; i < scannerPositionsLen; i++) {
        for (int j = 0; j < scannerPositionsLen; j++) {
            if (i == j)
                continue;
            vec3 a = scannerPositions[i];
            vec3 b = scannerPositions[j];
            if (i == 0)
                a = scannerPositions[i];
            if (j == 0)
                b = scannerPositions[i];
            int dist = manhattan(a, b);
            if (dist > max)
                max = dist;
        }
    }
    printf("%d\n", max);
    return 0;
}
