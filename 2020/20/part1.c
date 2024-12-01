#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 0
#define FILE_NAME "input"
#else
#define FILE_NAME "test"
#endif

typedef struct tile tile;
struct tile {
    int id;
    bool data[10][10];
    bool up[10];
    bool down[10];
    bool left[10];
    bool right[10];
};

typedef enum rotation rotation;
enum rotation {
    ROT_0,
    ROT_90,
    ROT_180,
    ROT_270,
};

typedef struct key key;
struct key {
    int index;
    rotation rotation;
    bool flip;
    bool string[10];
};

void dump_tiles(tile *tiles, int len) {
    for (int i = 0; i < len; i++) {
        printf("Tile %d:\n", tiles[i].id);
        for (int j = 0; j < 10; j++) {
            for (int k = 0; k < 10; k++) {
                printf("%c", tiles[i].data[j][k] ? '#' : '.');
            }
            printf("\n");
        }
        printf("\n");
    }
}

void rotate_clockwise(tile *tile) {
    // transpose
    bool tmp[10][10] = {0};
    memcpy(tmp, tile->data, sizeof(tmp));
    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 10; j++) {
            tile->data[i][j] = tmp[j][i];
        }
    }
    // flip all rows
    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 5; j++) {
            bool tmp = tile->data[i][j];
            tile->data[i][j] = tile->data[i][9 - j];
            tile->data[i][9 - j] = tmp;
        }
    }
    for (int i = 0; i < 10; i++) {
        tile->up[i] = tile->data[0][i];
        tile->down[i] = tile->data[9][i];
        tile->left[i] = tile->data[i][0];
        tile->right[i] = tile->data[i][9];
    }
}

void rotate_counterclockwise(tile *tile) {
    // transpose
    bool tmp[10][10] = {0};
    memcpy(tmp, tile->data, sizeof(tmp));
    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 10; j++) {
            tile->data[i][j] = tmp[j][i];
        }
    }
    // flip all cols
    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 5; j++) {
            bool tmp = tile->data[j][i];
            tile->data[j][i] = tile->data[9 - j][i];
            tile->data[9 - j][i] = tmp;
        }
    }
    for (int i = 0; i < 10; i++) {
        tile->up[i] = tile->data[0][i];
        tile->down[i] = tile->data[9][i];
        tile->left[i] = tile->data[i][0];
        tile->right[i] = tile->data[i][9];
    }
}

void flip(tile *tile) {
    bool tmp[10][10] = {0};
    memcpy(tmp, tile->data, sizeof(tmp));

    for (int i = 0; i < 10; i++) {
        for (int j = 0; j < 10; j++) {
            tile->data[i][j] = tmp[i][9 - j];
        }
    }
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    tile tiles[9];
    memset(tiles, 0, sizeof(tiles));
    int tiles_len = 0;

    tile *current_tile = NULL;
    while (getline(&line, &len, file) != EOF) {
        current_tile = &tiles[tiles_len++];
        current_tile->id = atoi(line + 5);
        for (int i = 0; i < 10; i++) {
            getline(&line, &len, file);
            for (int j = 0; j < 10; j++) {
                current_tile->data[i][j] = line[j] == '#';
            }
        }
        for (int i = 0; i < 10; i++) {
            current_tile->up[i] = current_tile->data[0][i];
            current_tile->down[i] = current_tile->data[9][i];
            current_tile->left[i] = current_tile->data[i][0];
            current_tile->right[i] = current_tile->data[i][9];
        }
        getline(&line, &len, file);
    }

    key *keys = malloc(tiles_len * 8 * 4 * sizeof(key));
    int keys_len = 0;
    for (int i = 0; i < tiles_len; i++) {
        tile tmp = tiles[i];
        // iterate all rotations
        for (rotation rotation = ROT_0; rotation <= ROT_270; ++rotation) {
            key key = {
                .index = i,
                .rotation = rotation,
                .flip = false,
            };
            memcpy(key.string, tmp.up, sizeof(key.string));
            keys[keys_len++] = key;
            memcpy(key.string, tmp.left, sizeof(key.string));
            keys[keys_len++] = key;
            memcpy(key.string, tmp.down, sizeof(key.string));
            keys[keys_len++] = key;
            memcpy(key.string, tmp.right, sizeof(key.string));
            keys[keys_len++] = key;
            rotate_clockwise(&tmp);
        }
        // flip and repeat
        for (rotation rotation = ROT_0; rotation <= ROT_270; ++rotation) {
            key key = {
                .index = i,
                .rotation = rotation,
                .flip = true,
            };
            memcpy(key.string, tmp.up, sizeof(key.string));
            keys[keys_len++] = key;
            memcpy(key.string, tmp.left, sizeof(key.string));
            keys[keys_len++] = key;
            memcpy(key.string, tmp.down, sizeof(key.string));
            keys[keys_len++] = key;
            memcpy(key.string, tmp.right, sizeof(key.string));
            keys[keys_len++] = key;
            rotate_clockwise(&tmp);
        }
    }

    int match_count = 0;
    for (int i = 0; i < keys_len; i++) {
        for (int j = 0; j < keys_len; j++) {
            if (i == j) {
                continue;
            }
            if (keys[i].index == keys[j].index) {
                continue;
            }
            if (memcmp(keys[i].string, keys[j].string,
                       sizeof(keys[i].string)) == 0) {
                printf("match %d %d\n", i, j);
                ++match_count;
            }
        }
    }
    printf("match count: %d (%d)\n", match_count, keys_len * keys_len);
}
