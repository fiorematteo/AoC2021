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

struct rule {
    char v;
    int l[2];
    int r[2];
};

bool match(char **message, struct rule *rules, int rule) {
    bool outl = false;
    bool outr = false;
    if (rules[rule].v != 0) {
        bool out = (rules[rule].v == **message);
        (*message)++;
        return out;
    }

    char *msg = malloc(strlen(*message) + 1);
    strcpy(msg, *message);

    if (rules[rule].l[0] != 0) {
        outl = match(message, rules, rules[rule].l[0]);
        if (rules[rule].l[1] != 0)
            outl &= match(message, rules, rules[rule].l[1]);
    }

    if (rules[rule].r[0] != 0) {
        outr = match(&msg, rules, rules[rule].r[0]);
        if (rules[rule].r[1] != 0)
            outr &= match(&msg, rules, rules[rule].r[1]);
    }

    return outr || outl;
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    struct rule rules[200];
    memset(rules, 0, sizeof(rules));
    int rules_len = 0;
    while (getline(&line, &len, file) != EOF) {
        if (strcmp(line, "\n") == 0)
            break;
        int index = atoi(strsep(&line, ":"));
        char *token = strsep(&line, " ");
        if (line[0] == '\"') {
            rules[index].v = line[1];
        } else {
            token = strsep(&line, " ");
            rules[index].l[0] = atoi(token);
            token = strsep(&line, " ");
            if (token)
                rules[index].l[1] = atoi(token);
            token = strsep(&line, " ");
            if (token && token[0] == '|') {
                token = strsep(&line, " ");
                rules[index].r[0] = atoi(token);
                token = strsep(&line, " ");
                if (token)
                    rules[index].r[1] = atoi(token);
            }
        }
        line = NULL;
    }

    line = NULL;
    int count = 0;
    while (getline(&line, &len, file) != EOF) {
        line[strlen(line) - 1] = '\0';
        printf("%s", line);
        if (match(&line, rules, 0)) {
            printf(" MATCH");
            count++;
        }
        printf("\n");
        line = NULL;
    }
    printf("%d\n", count);
}
