#include <stdio.h>
#include <string.h>
#include <ctype.h>

void capitalize(char *str) {
    for (int i = 0; str[i]; i++) {
        str[i] = toupper((unsigned char)str[i]);
    }
}

int main(int argc, char *argv[]) {
    char greeting[100] = "Hello, world!";
    int should_capitalize = 0;

    if (argc > 1) {
        snprintf(greeting, sizeof(greeting), "Hello, %s!", argv[1]);
    }

    if (argc > 2 && strcmp(argv[2], "--capitalize") == 0) {
        should_capitalize = 1;
    }

    if (should_capitalize) {
        capitalize(greeting);
    }

    printf("%s\n", greeting);

    return 0;
}
