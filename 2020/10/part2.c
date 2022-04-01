#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if 1
#define FILE_NAME "input"
#else
#define FILE_NAME "test"
#endif

struct node {
    int val;
    struct node *first;
    struct node *second;
    struct node *third;
};

void sort(int *arr, int len) {
    int i, j, tmp;
    for (i = 0; i < len; i++) {
        for (j = i + 1; j < len; j++) {
            if (arr[i] > arr[j]) {
                tmp = arr[i];
                arr[i] = arr[j];
                arr[j] = tmp;
            }
        }
    }
}

long seen[200];
long count_children(struct node *node) {
    if (seen[node->val] != 0) {
        return seen[node->val];
    }
    long total = 0;
    if (node->first)
        total += count_children(node->first);
    if (node->second)
        total += count_children(node->second);
    if (node->third)
        total += count_children(node->third);
    if (total == 0) {
        seen[node->val] = 1;
        return 1;
    }
    seen[node->val] = total;
    return total;
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    int nums[200];
    memset(nums, 0, sizeof(nums));
    int nums_len = 0;
    nums[nums_len++] = 0;
    while (getline(&line, &len, file) != EOF) {
        nums[nums_len++] = atoi(line);
    }
    sort(nums, nums_len);

    struct node nodes[200];
    memset(nodes, 0, sizeof(nodes));
    for (int i = 0; i < nums_len; i++) {
        nodes[i] = (struct node){.val = nums[i], NULL, NULL, NULL};
    }

    for (int i = 0; i < nums_len; i++) {
        for (int j = 1; j < 4; j++) {
            if (i + j > nums_len)
                break;
            switch (nums[i + j] - nums[i]) {
            case 1:
                nodes[i].first = &nodes[i + j];
                break;
            case 2:
                nodes[i].second = &nodes[i + j];
                break;
            case 3:
                nodes[i].third = &nodes[i + j];
                break;
            }
        }
    }

    memset(seen, 0, sizeof(seen));
    long total = count_children(nodes);
    printf("total=%ld\n", total);
}
