#include <stdio.h>
#include <stdlib.h>

#define DAYS 80
#define FILE_NAME "input"
//#define FILE_NAME "test"

typedef struct laternfish {
    int days;
    struct laternfish *next;
} laternfish;

int countFish = 0;

void add_fish(laternfish **head, int days) {
    laternfish *new_fish = malloc(sizeof(laternfish));
    new_fish->days = days;
    new_fish->next = *head;
    *head = new_fish;
    countFish++;
}

void print_fish(laternfish *head) {
    while (head != NULL) {
        printf("%d ", head->days);
        head = head->next;
    }
    printf("\n");
}

int main() {
    // read input file
    FILE *f = fopen(FILE_NAME, "r");
    laternfish *head = NULL;

    char c;
    while ((c = fgetc(f)) != EOF) {
        if (c == ',' || c == ' ' || c == '\n') {
            continue;
        }
        add_fish(&head, c - '0');
    }
    printf("initial fish: ");
    print_fish(head);

    int days = 0;
    while (days < DAYS) {
        laternfish *curr = head;
        while (curr != NULL) {
            curr->days--;
            if (curr->days == -1) {
                add_fish(&head, 8);
                curr->days = 6;
            }
            curr = curr->next;
        }
        days++;
    }
    printf("there are %d\n", countFish);
}
