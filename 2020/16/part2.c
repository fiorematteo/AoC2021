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

struct range {
    int start;
    int end;
    struct range *alternative;
};

bool validate(struct range r, int v) {
    if (r.start <= v && v <= r.end) {
        return true;
    }
    if (r.alternative != NULL) {
        return validate(*r.alternative, v);
    }
    return false;
}

bool validate_array(struct range r, int *vs, int v_len) {
    for (int i = 0; i < v_len; i++) {
        if (!validate(r, vs[i])) {
            return false;
        }
    }
    return true;
}

void print_ranges(struct range *rs, int ranges_len) {
    for (int i = 0; i < ranges_len; i++) {
        printf("%d-%d", rs[i].start, rs[i].end);
        if (rs[i].alternative != NULL) {
            printf("|");
            print_ranges(rs[i].alternative, 1);
        } else
            printf(" ");
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
    struct range *ranges = malloc(sizeof(struct range) * 30);
    int ranges_len = 0;
    while (getline(&line, &len, file) != EOF) {
        if (strcmp(line, "\n") == 0)
            break;
        strsep(&line, ":");
        int start1 = atoi(strsep(&line, "-"));
        int end1 = atoi(strsep(&line, " "));
        strsep(&line, "r");
        int start2 = atoi(strsep(&line, "-"));
        int end2 = atoi(strsep(&line, " "));
        ranges[ranges_len].start = start1;
        ranges[ranges_len].end = end1;
        ranges[ranges_len].alternative = malloc(sizeof(struct range));
        ranges[ranges_len].alternative->start = start2;
        ranges[ranges_len].alternative->end = end2;
        ranges[ranges_len].alternative->alternative = NULL;
        ranges_len++;
    }
    getline(&line, &len, file);
    getline(&line, &len, file);
    char *token;
    int field_index = 0;

    int my_ticket[ranges_len];
    while ((token = strsep(&line, ",")) != NULL) {
        my_ticket[field_index] = atoi(token);
        field_index++;
    }
    getline(&line, &len, file);
    getline(&line, &len, file);

    int tickets[ranges_len][200];
    int tickets_len = 0;
    memset(tickets, 0, sizeof(tickets));
    while (getline(&line, &len, file) != EOF) {
        int out_of_range = 0;
        field_index = 0;
        while ((token = strsep(&line, ",")) != NULL) {
            int value = atoi(token);
            out_of_range = 0;
            tickets[field_index][tickets_len] = value;
            field_index++;
            for (int i = 0; i < ranges_len; i++) {
                if (validate(ranges[i], value))
                    continue;
                out_of_range++;
            }
            if (out_of_range == ranges_len) {
                break;
            }
        }
        if (out_of_range != ranges_len)
            tickets_len++;
        assert(tickets_len < 200);
        line = NULL;
    }
    printf("parsing done\n");

    int strong_link[20];
    memset(strong_link, -1, sizeof(strong_link));
    int strong_link_len = 0;
    int inv_strong_link[20];
    memset(inv_strong_link, -1, sizeof(strong_link));

    while (strong_link_len < 20) {
        int valid_ranges[ranges_len][ranges_len];
        memset(valid_ranges, -1, sizeof(valid_ranges));
        int valid_ranges_len[20];
        memset(valid_ranges_len, 0, sizeof(valid_ranges_len));

        for (int i = 0; i < ranges_len; i++) {
            if (strong_link[i] != -1)
                continue;
            int valid_counter = 0;
            for (int j = 0; j < ranges_len; j++) {
                if (inv_strong_link[j] != -1)
                    continue;
                if (validate_array(ranges[i], tickets[j], tickets_len)) {
                    valid_ranges[i][valid_ranges_len[i]] = j;
                    valid_ranges_len[i]++;
                    valid_counter++;
                }
            }
            if (valid_counter == 1)
                printf("range %d has %d valid fields\n", i, valid_counter);
            if (valid_counter == 1) {
                strong_link[i] = valid_ranges[i][0];
                inv_strong_link[valid_ranges[i][0]] = i;
                strong_link_len++;
            }
        }
    }

    long total = 1;
    for (int i = 0; i < 6; i++) {
        total *= my_ticket[strong_link[i]];
    }
    printf("total: %ld\n", total);
}
