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

int find_end_paren(char *s, int begin) {
    int depth = 0;
    for (int i = begin + 1; s[i]; i++) {
        if (s[i] == '(') {
            depth++;
        } else if (s[i] == ')') {
            depth--;
        }
        if (depth == 0) {
            return i;
        }
    }
    return -1;
}

int find_begin_paren(char *s, int end) {
    int depth = 0;
    for (int i = end - 1; i >= 0; i--) {
        if (s[i] == ')') {
            depth++;
        } else if (s[i] == '(') {
            depth--;
        }
        if (depth == 0) {
            return i;
        }
    }
    return -1;
}

void add_parens(char *s) {
    for (int i = 0; s[i]; i++) {
        if (s[i] == '+') {
            if (s[i - 1] == ')') {
                int index = find_begin_paren(s, i);
                memmove(s + index + 1, s + index, strlen(s));
                s[index] = '(';
            } else {
                memmove(s + i, s + i - 1, strlen(s));
                s[i - 1] = '(';
            }
            i++;
            if (s[i + 1] == '(') {
                int index = find_end_paren(s, i);
                memmove(s + index + 2, s + index + 1, strlen(s));
                s[index + 1] = ')';
            } else {
                memmove(s + i + 2, s + i + 1, strlen(s));
                s[i + 2] = ')';
            }
            i++;
        }
    }
}

void remove_spaces(char *s) {
    for (int i = 0; s[i]; i++) {
        if (s[i] == ' ') {
            memmove(s + i, s + i + 1, strlen(s + i + 1) + 1);
            i--;
        } else if (s[i] == '\n') {
            s[i] = '\0';
            break;
        }
    }
}

long eval(char **str) {
    long sum = 0;
    int op = 0;
    long temp = 0;

    while (**str != '\0') {
        switch (**str) {
        case ' ':
        case '\n':
            (*str)++;
            break;
        case '+':
            op = 1;
            (*str)++;
            break;
        case '*':
            op = 2;
            (*str)++;
            break;
        case '(':
            (*str)++;
            temp = eval(str);
            switch (op) {
            case 0:
                sum = temp;
                break;
            case 1:
                sum += temp;
                break;
            case 2:
                sum *= temp;
                break;
            }
            op = 0;
            break;
        case ')':
            (*str)++;
            return sum;
            break;
        default:
            assert(**str >= '0' && **str <= '9');
            temp = atoi(*str);
            (*str)++;
            switch (op) {
            case 0:
                sum = temp;
                break;
            case 1:
                sum += temp;
                break;
            case 2:
                sum *= temp;
                break;

                op = 0;
            }
            break;
        }
    }
    return sum;
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    long full_sum = 0;
    while (getline(&line, &len, file) != EOF) {
        remove_spaces(line);
        line = realloc(line, strlen(line) * 20);
        add_parens(line);
        printf("%s  ", line);

        long tmp = eval(&line);
        printf("%ld\n", tmp);
        full_sum += tmp;
        line = NULL;
    }
    printf("full_sum: %ld\n", full_sum);
}
