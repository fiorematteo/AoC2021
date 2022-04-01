#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define FILE_NAME "input"
#else
#define FILE_NAME "test"
#endif

struct value {
    unsigned long i;
    unsigned long v;
    struct value *next;
};

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    struct value buckets[100000];
    memset(buckets, 0, sizeof(buckets));
    int mask[36];
    memset(mask, -1, sizeof(mask));
    while (getline(&line, &len, file) != EOF) {
        if (line[1] == 'a') {
            strsep(&line, " ");
            strsep(&line, " ");
            char *string_mask = strsep(&line, " ");
            string_mask[36] = '\0';
            for (int i = 0; i < strlen(string_mask); i++) {
                assert(35 - i >= 0);
                if (string_mask[35 - i] == '1') {
                    mask[i] = 1;
                } else if (string_mask[35 - i] == '0') {
                    mask[i] = 0;
                } else if (string_mask[35 - i] == 'X') {
                    mask[i] = -1;
                } else {
                    assert("bad char in string_mask" || false);
                }
            }
        } else {
            strsep(&line, "[]");
            unsigned long index = atol(strsep(&line, "[]"));
            strsep(&line, "=");
            unsigned long value = atol(line);

            unsigned long indexes[20000];
            memset(indexes, 0, sizeof(indexes));
            int indexes_len = 0;

            for (int i = 0; i < 36; i++)
                if (mask[i] == 1)
                    index |= (1UL << i);

            for (int i = 0; i < 36; i++) {
                if (mask[i] == 0 || mask[i] == 0)
                    continue;
                if (mask[i] == -1) {
                    if (indexes_len == 0) {
                        indexes[indexes_len] = index | (1UL << i);
                        indexes_len++;
                        indexes[indexes_len] = index & ~(1UL << i);
                        indexes_len++;
                        continue;
                    }
                    int indexes_len_copy = indexes_len;
                    for (int j = 0; j < indexes_len_copy; j++) {
                        indexes[indexes_len] = indexes[j] | (1UL << i);
                        indexes_len++;
                        assert(indexes_len <
                               sizeof(indexes) / sizeof(indexes[0]));
                        indexes[indexes_len] = indexes[j] & ~(1UL << i);
                        indexes_len++;
                        assert(indexes_len <
                               sizeof(indexes) / sizeof(indexes[0]));
                    }
                }
            }

            for (int i = 0; i < indexes_len; i++) {
                printf("index: %lu, value %lu\n", indexes[i], value);
                int possible_index =
                    indexes[i] % (sizeof(buckets) / sizeof(buckets[0]));
                if (buckets[possible_index].i == 0) {
                    buckets[possible_index].i = indexes[i];
                    buckets[possible_index].v = value;
                } else if (buckets[possible_index].i == indexes[i]) {
                    buckets[possible_index].v = value;
                } else if (buckets[possible_index].i != indexes[i]) {
                    printf("collision!\n");
                    struct value *curr = &buckets[possible_index];
                    bool found = false;
                    while (curr->next != NULL) {
                        if (curr->next->i == indexes[i]) {
                            curr->next->v = value;
                            found = true;
                            break;
                        }
                        curr = curr->next;
                    }
                    if (!found) {
                        curr->next = malloc(sizeof(struct value));
                        curr->next->i = indexes[i];
                        curr->next->v = value;
                        curr->next->next = NULL;
                    }
                }
            }
        }
        line = NULL;
    }

    unsigned long sum = 0;
    for (int i = 0; i < sizeof(buckets) / sizeof(buckets[0]); i++) {
        if (buckets[i].v == 0)
            continue;
        struct value *curr = buckets[i].next;
        while (curr != NULL) {
            sum += curr->v;
            curr = curr->next;
        }
        sum += buckets[i].v;
        printf("index:%lu value:%lu sum: %lu\n", buckets[i].i, buckets[i].v,
               sum);
    }
    printf("sum: %lu\n", sum);
}
