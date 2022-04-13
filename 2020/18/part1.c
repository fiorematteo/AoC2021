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

long eval(char **str) {
    long sum = 0;
    int op = 0;
    long temp = 0;
    printf("%s\n", *str);

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
        long inptr = 0;
        long tmp = eval(&line);
        printf("%ld\n", tmp);
        full_sum += tmp;
        line = NULL;
    }
    printf("full_sum: %ld\n", full_sum);
}
