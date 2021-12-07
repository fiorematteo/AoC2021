#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

//#define FILE_NAME "test"
#define FILE_NAME "input"

struct list {
    int *data;
    int data_len;
};
typedef struct list list;

void append(list *l, int x) {
    l->data = realloc(l->data, (l->data_len + 1) * sizeof(int));
    l->data[l->data_len] = x;
    l->data_len++;
}

int costo(int n) {
    int sum = 0;
    for (int i = 0; i <= n; i++) {
        sum += i;
    }
    return sum;
}

int main() {
    // read input file
    FILE *f = fopen(FILE_NAME, "r");

    list crabs = {.data = malloc(sizeof(int)), .data_len = 0};

    char *str = NULL;
    size_t len = 1000;
    getline(&str, &len, f);

    char *token = strtok(str, ",");
    while (token != NULL) {
        append(&crabs, atoi(token));
        token = strtok(NULL, ",");
    }

    int max = 0;
    int min = 0;
    for (int i = 0; i < crabs.data_len; i++) {
        if (crabs.data[i] > max) {
            max = crabs.data[i];
        }
        if (crabs.data[i] < min) {
            min = crabs.data[i];
        }
    }

    int prev_fuel = 999999999;
    int min_i = 0;
    for (int i = min; i <= max; i++) {
        int total_fuel = 0;
        for (int j = 0; j < crabs.data_len; j++) {
            total_fuel += costo(abs(crabs.data[j] - i));
            if (i == 5)
                printf("%d -> %d: %d\n", crabs.data[j], i,
                       costo(abs(crabs.data[j] - i)));
        }
        if (total_fuel < prev_fuel) {
            prev_fuel = total_fuel;
            min_i = i;
        }
    }
    printf("minimo carburante %d\n", prev_fuel);
    printf("punto di allineamento %d\n", min_i);
}
