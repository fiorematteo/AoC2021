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
        int byr = atoi(passports[i].byr);
        if (byr < 1920 || byr > 2002)
            continue;
        int iyr = atoi(passports[i].iyr);
        if (iyr < 2010 || iyr > 2020)
            continue;
        int eyr = atoi(passports[i].eyr);
        if (eyr < 2020 || eyr > 2030)
            continue;
        char *unit = malloc(sizeof(char) * 2);
        int hgt;
        sscanf(passports[i].hgt, "%d%c%c", &hgt, &unit[0], &unit[1]);
        if(strcmp(unit, "") == 0)
            continue;
        if (strcmp(unit, "cm") == 0 && (hgt < 150 || hgt > 193))
            continue;
        if (strcmp(unit, "in") == 0 && (hgt < 59 || hgt > 76))
            continue;

        if (passports[i].hcl[0] != '#')
            continue;
        if (strlen(passports[i].hcl) != 7)
            continue;

        bool should_continue = false;
        for (int j = 1; j < strlen(passports[i].hcl); j++) {
            if ((passports[i].hcl[j] < '0' || passports[i].hcl[j] > '9') &&
                (passports[i].hcl[j] < 'a' || passports[i].hcl[j] > 'f')) {
                should_continue = true;
                break;
            }
        }
        if (should_continue)
            continue;
        if (!(strcmp(passports[i].ecl, "amb") == 0 ||
              strcmp(passports[i].ecl, "blu") == 0 ||
              strcmp(passports[i].ecl, "brn") == 0 ||
              strcmp(passports[i].ecl, "gry") == 0 ||
              strcmp(passports[i].ecl, "grn") == 0 ||
              strcmp(passports[i].ecl, "hzl") == 0 ||
              strcmp(passports[i].ecl, "oth") == 0)) {
            continue;
        }
        if (strlen(passports[i].pid) != 9)
            continue;
        for (int j = 0; j < strlen(passports[j].pid); j++) {
            if (passports[i].pid[j] < '0' || passports[i].pid[j] > '9') {
                should_continue = true;
                break;
            }
        }
        if (should_continue)
            continue;

        printf("Passport %d is valid!\n", i);
        print_passport(&passports[i]);
        valid_passports++;
    }

    printf("Valid passports: %d\n", valid_passports);
}
