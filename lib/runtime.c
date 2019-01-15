#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int readInt() {
    int ret;
    scanf("%d\n", &ret);
    return ret;
}

char* readString() {
    char* ret = NULL;
    size_t n = 0;
    ssize_t len = getline(&ret, &n, stdin);
    ret[len-1] = 0;
    return ret;
}

void printInt(int i) {
    printf("%d\n", i);
}

void printString(char* s) {
    printf("%s\n", s);
}

char* concat(char* s1, char* s2) {
    size_t l1 = strlen(s1);
    size_t l2 = strlen(s2);
    char* res = (char*)malloc(sizeof(char)*(l1+l2+1));
    strcpy(res, s1);
    strcpy(res+l1, s2);
    return res;
}