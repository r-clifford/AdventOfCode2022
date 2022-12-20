#include <stdlib.h>
#include <stdio.h>

int main() {
    char *path = "./src/data/1.txt";
    FILE *f = fopen(path, "r");
    if (f == NULL) {
        return -1;
    }
    char buf[10];
    int max_cal = 0;
    int current = 0;
    while (fgets(buf, 10, f) != NULL)
    {
        int cals;
        if (sscanf(buf, "%d", &cals) == 1) {
            current += cals;
        }
        else
        {
            if (current > max_cal) {
                max_cal = current;
            }
            current = 0;
        }
    }
    printf("%d\n", max_cal);
}