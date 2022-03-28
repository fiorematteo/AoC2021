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

struct bag {
    int num;
    char *name;
    struct bag *children;
    int children_len;
};

void print_bag(struct bag *bag) {
    printf("%s %d children\n", bag->name, bag->children_len);
    for (int i = 0; i < bag->children_len; i++) {
        printf("    %d, %s\n", bag->children[i].num, bag->children[i].name);
    }
}

bool search_gold(struct bag *bags, int bags_len, struct bag bag) {
    for (int i = 0; i < bag.children_len; i++) {
        if (strcmp(bag.children[i].name, "shinygold") == 0)
            return true;
        for (int j = 0; j < bags_len; j++) {
            if (strcmp(bag.children[i].name, bags[j].name) == 0)
                if (search_gold(bags, bags_len, bags[j]))
                    return true;
        }
    }
    return false;
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    struct bag bags[700];
    for (int i = 0; i < sizeof(bags) / sizeof(struct bag); i++) {
        bags[i].name = malloc(sizeof(char) * 20);
        bags[i].children = malloc(sizeof(struct bag) * 100);
    }
    int bags_len = 0;
    char *line = NULL;
    size_t len = 0;
    while (getline(&line, &len, file) != EOF) {
        strcpy(bags[bags_len].name, strsep(&line, " "));
        strcat(bags[bags_len].name, strsep(&line, " "));
        strsep(&line, " ");
        strsep(&line, " ");
        int children_len = 0;
        while (true) {
            struct bag *child = bags[bags_len].children + children_len;
            child->name = malloc(sizeof(char) * 20);
            char *num = strsep(&line, " ");
            if (num == NULL)
                break;
            child->num = atoi(num);
            strcpy(child->name, strsep(&line, " "));
            strcat(child->name, strsep(&line, " "));
            strsep(&line, " ");
            children_len++;
        }
        bags[bags_len].children_len = children_len;
        if (children_len == 1 &&
            strcmp(bags[bags_len].children[0].name, "otherbags.\n") == 0)
            bags[bags_len].children_len = 0;
        bags_len++;
    }

    int sum = 0;
    for (int i = 0; i < bags_len; i++) {
        print_bag(&bags[i]);
        bool contains_gold = search_gold(bags, bags_len, bags[i]);
        sum += contains_gold;
        printf("contains gold %d\n\n", contains_gold);
    }
    printf("SOMMA: %d\n", sum);
}
