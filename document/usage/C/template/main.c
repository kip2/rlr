#include <stdio.h>
#include <string.h>

int get_input_int() {
    int n;
    scanf("%d", &n);
    return n;
}

char *get_input(char *buffer, int size) {
    if (fgets(buffer, size, stdin)) {
        buffer[strcspn(buffer, "\n")] = '\0';
    }
    return buffer;
}

void get_int_array(int *arr, int max_size) {
    for (int i = 0; i < max_size; i++) {
        scanf("%d", &arr[i]);
    }
}

void join_int_array(const int *arr, int length, char *out, const char *sep) {
    out[0] = '\0';

    for (int i = 0; i < length; i++) {
        char buffer[32];
        if (i > 0) {
            strcat(out, sep);
        }
        sprintf(buffer, "%d", arr[i]);
        strcat(out, buffer);
    }
}

int main(void) {
    int x = get_input_int();
    int y = get_input_int();

    printf("%d", x - y);

    return 0;
}
