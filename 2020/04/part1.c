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

struct passport {
    char byr[100];
    char iyr[100];
    char eyr[100];
    char hgt[100];
    char hcl[100];
    char ecl[100];
    char pid[100];
    char cid[100];
};
enum state { key, value, end };

char *get_value(char *key, struct passport *passport) {
    switch (key[0]) {
    case 'b':
        return passport->byr;
    case 'i':
        return passport->iyr;
    case 'h':
        if (key[1] == 'g')
            return passport->hgt;
        else if (key[1] == 'c')
            return passport->hcl;
    case 'e':
        if (key[1] == 'c')
            return passport->ecl;
        else if (key[1] == 'y')
            return passport->eyr;
    case 'p':
        return passport->pid;
    case 'c':
        return passport->cid;
    default:
        assert(false);
    }
}

void print_passport(struct passport *passport) {
    printf("\n");
    printf("byr: %s\n", passport->byr);
    printf("iyr: %s\n", passport->iyr);
    printf("eyr: %s\n", passport->eyr);
    printf("hgt: %s\n", passport->hgt);
    printf("hcl: %s\n", passport->hcl);
    printf("ecl: %s\n", passport->ecl);
    printf("pid: %s\n", passport->pid);
    printf("cid: %s\n", passport->cid);
    printf("\n");
}

int main() {
    FILE *file = fopen(FILE_NAME, "r");
    if (file == NULL) {
        printf("Error opening file!\n");
        return 1;
    }

    struct passport passports[1280];
    char c;
    int passport_count = 0;
    enum state state = key;
    char *current_key = malloc(sizeof(char));
    char *current_value = malloc(sizeof(char));
    while ((c = fgetc(file)) != EOF) {
        switch (state) {
        case end:
            state = key;
            if (c == '\n') {
                print_passport(&passports[passport_count]);
                passport_count++;
                break;
            }
        case key:
            if (c == ':') {
                state = value;
            } else {
                current_key = realloc(current_key, (strlen(current_key) + 1));
                current_key[strlen(current_key)] = c;
            }
            break;
        case value:
            if (c == '\n' || c == ' ') {
                state = end;
                char *target =
                    get_value(current_key, &passports[passport_count]);
                strcpy(target, current_value);
                current_key = malloc(sizeof(char));
                current_value = malloc(sizeof(char));
            } else {
                current_value =
                    realloc(current_value, (strlen(current_value) + 1));
                current_value[strlen(current_value)] = c;
            }
            break;
        }
    }

    int valid_passports = 0;
    for (int i = 0; i < passport_count; i++) {
        if (strcmp(passports[i].byr, "") == 0 ||
            strcmp(passports[i].iyr, "") == 0 ||
            strcmp(passports[i].eyr, "") == 0 ||
            strcmp(passports[i].hgt, "") == 0 ||
            strcmp(passports[i].hcl, "") == 0 ||
            strcmp(passports[i].ecl, "") == 0 ||
            strcmp(passports[i].pid, "") == 0) {
            continue;
        }

        printf("Passport %d is valid!\n", i);
        valid_passports++;
    }

    printf("Valid passports: %d\n", valid_passports);
}
