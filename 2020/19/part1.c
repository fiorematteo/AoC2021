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

typedef enum kind kind;
enum kind {
    LITERAL,
    BRANCH,
    SEQUENCE,
};

typedef struct rule rule;
struct rule {
    int sequence[4];
    char value;
    int left[4];
    int right[4];
    kind kind;
};

bool match(char **message, rule *rules, rule rule) {
    if (rule.kind == LITERAL) {
        bool out = **message == rule.value;
        (*message)++;
        return out;
    } else if (rule.kind == SEQUENCE) {
        bool out = true;
        for (int i = 0; rule.sequence[i] != 0; i++) {
            out &= match(message, rules, rules[rule.sequence[i]]);
        }
        return out;
    } else if (rule.kind == BRANCH) {
        // backtracking?
        char *old_line = *message;

        bool left = true;
        for (int i = 0; rule.left[i] != 0; i++) {
            left &= match(message, rules, rules[rule.left[i]]);
        }
        if (left)
            return true;
        message = &old_line;

        bool right = true;
        for (int i = 0; rule.right[i] != 0; i++) {
            right &= match(message, rules, rules[rule.right[i]]);
        }
        return right;
    } else {
        assert(false);
    }
}

void dump_rules(rule *rules, int len) {
    for (int i = 0; i < len; i++) {
        rule r = rules[i];
        if (r.value != 0) {
            printf("%d: %c\n", i, r.value);
        } else {
            if (r.kind == SEQUENCE) {
                printf("%d: ", i);
                for (int j = 0; j < 4; j++) {
                    if (r.sequence[j] != 0) {
                        printf("%d ", r.sequence[j]);
                    }
                }
                printf("\n");
            } else {
                printf("%d: ", i);
                for (int j = 0; j < 4; j++) {
                    if (r.left[j] != 0) {
                        printf("%d ", r.left[j]);
                    }
                }
                printf("| ");
                for (int j = 0; j < 4; j++) {
                    if (r.right[j] != 0) {
                        printf("%d ", r.right[j]);
                    }
                }
                printf("\n");
            }
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
    rule rules[200];
    memset(rules, 0, sizeof(rules));
    int rules_len = 0;
    while (getline(&line, &len, file) != EOF) {
        if (strcmp(line, "\n") == 0)
            break;
        rules_len++;
        int index = atoi(strsep(&line, ":"));
        line++;
        rule *rule = &rules[index];
        if (line[0] == '\"') {
            rule->value = line[1];
            rule->kind = LITERAL;
        } else {
            if (strstr(line, "|") == NULL) {
                int i = 0;
                char *token = strsep(&line, " ");
                while (token != NULL) {
                    assert(i < 4);
                    rule->sequence[i] = atoi(token);
                    token = strsep(&line, " ");
                    i++;
                }
                rule->kind = SEQUENCE;
            } else {
                char *left = strsep(&line, "|");
                int i = 0;
                char *token = strsep(&left, " ");
                while (token != NULL) {
                    assert(i < 4);
                    rule->left[i] = atoi(token);
                    token = strsep(&left, " ");
                    i++;
                }
                i = 0;
                line++;
                char *right = line;
                token = strsep(&right, " ");
                while (token != NULL) {
                    assert(i < 4);
                    rule->right[i] = atoi(token);
                    token = strsep(&right, " ");
                    i++;
                }
                rule->kind = BRANCH;
            }
        }
        line = NULL;
    }

    dump_rules(rules, rules_len);
    printf("\n");

    line = NULL;
    int count = 0;
    while (getline(&line, &len, file) != EOF) {
        line[strlen(line) - 1] = '\0';
        printf("%s", line);
        if (match(&line, rules, rules[0]) && *line == '\0') {
            printf(" MATCH");
            count++;
        }
        printf("\n");
        line = NULL;
    }
    printf("%d\n", count);
}
