#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#define INIT_CAP 20
#define THRESHOLD 100000
#define TOTAL_SPACE 70000000
#define REQ_SPACE 30000000
void strip_newline(char *s)
{
    size_t len = strlen(s);
    if (s[len - 1] == '\n')
    {
        s[len - 1] = '\0';
    }
}
struct File_T;
typedef struct File_T
{
    char *name;
    size_t capacity;
    size_t size;
    struct File_T *parent;
    struct File_T **children;
    size_t child_size;
    int is_dir;
} file_t;
int add_child(file_t *wd, char *target, size_t size, int is_dir);

file_t *create_fs()
{
    file_t *root = malloc(sizeof(*root));
    root->parent = NULL;
    root->name = malloc(2*sizeof(char));
    strcpy(root->name, "/");
    root->size = 0;
    root->child_size = 0;
    root->is_dir = 1;
    root->capacity = INIT_CAP;
    root->children = malloc(sizeof(*root) * root->capacity);
    return root;
}
void destroy_fs(file_t* wd) {
    if (wd->children == NULL) {
        free(wd->name);
        free(wd);
        return;
    }
    for (size_t i = 0; i < wd->child_size; i++)
    {
        destroy_fs(wd->children[i]);
    }
    free(wd->children);
    free(wd->name);
    free(wd);
}
void pad(size_t n)
{
    for (size_t i = 0; i < n; i++)
    {
        printf("\t");
    }
}
void print_postorder(file_t *wd, size_t depth)
{
    if (wd->children == NULL)
    {
        pad(depth);
        printf("- %s (file, size=%lu)\n", wd->name, wd->size);
        return;
    }
    char *bt = "*";
    if (wd->size > THRESHOLD)
    {
        bt = "";
    }
    pad(depth);
    printf("- %s (dir, size=%lu)[%s]\n", wd->name, wd->size, bt);
    depth++;
    for (size_t i = 0; i < wd->child_size; i++)
    {
        print_postorder(wd->children[i], depth);
    }
}
size_t calculate_size(file_t *wd)
{
    if (wd->child_size <= 0)
    {
        return wd->size;
    }
    size_t total = 0;
    for (size_t i = 0; i < wd->child_size; i++)
    {
        if (wd->children[i]->is_dir)
        {
            size_t size = calculate_size(wd->children[i]);
            total += size;
        }
        else
        {
            total += wd->children[i]->size;
        }
    }
    wd->size = total;
    return total;
}

file_t *get_child(file_t *wd, char *target)
{
    for (size_t i = 0; i < wd->child_size; i++)
    {
        if (strcmp(target, wd->children[i]->name) == 0)
        {
            return wd->children[i];
        }
    }
    return NULL;
}
file_t *get_root(file_t *wd)
{
    if (wd->parent == NULL)
    {
        return wd;
    }
    return get_root(wd->parent);
}
file_t *fs_cd(file_t *wd, char *target)
{
    file_t *ret = NULL;
    if (strcmp(target, "..\n") == 0)
    {
        return wd->parent;
    }
    else if (strcmp(target, "/\n") == 0)
    {
        file_t *root = get_root(wd);
        return root;
    }
    else
    {

        ret = get_child(wd, target);
        if (ret == NULL)
        {
            // create child dir
            add_child(wd, target, 0, 1);
            ret = get_child(wd, target);
        }
        return ret;
    }
}
int add_child(file_t *wd, char *target, size_t size, int is_dir)
{
    strip_newline(target);
    // return 0 if fail
    if (get_child(wd, target))
    {
        return 0;
    }
    file_t *child = malloc(sizeof(*child));
    child->child_size = 0;
    child->children = NULL;
    child->is_dir = is_dir;
    child->name = malloc(sizeof(char) * INIT_CAP);
    child->size = size;
    // int s = strlen(target);
    strcpy(child->name, target);
    child->parent = wd;
    if (is_dir)
    {
        child->capacity = INIT_CAP;
        child->children = malloc(sizeof(*child) * child->capacity);
    }

    if (wd->size >= wd->capacity)
    {
        // realloc
        wd->children = realloc(wd->children, sizeof(file_t) * wd->capacity * 2);
        wd->capacity *= 2;
    }
    wd->children[wd->child_size] = child;
    (wd->child_size)++;
    return 1;
}
void parse_ls(file_t *wd, char *name, int is_dir, size_t size)
{
    // XXXXX fname
    // dir dirname
    // s is output of ls
    // split lines
    if (is_dir)
    {

        add_child(wd, name, 0, 1);
    }
    else
    {
        add_child(wd, name, size, 0);
    }
}

int array_contains(size_t len, char *array[len], char *s)
{
    for (size_t i = 0; i < len; i++)
    {
        if (strcmp(array[i], s) == 0)
        {
            return 1;
        }
    }
    return 0;
}
file_t *parse_cmd(file_t *wd, char *s)
{
    const char *delim = " ";
    char *token = strtok(s, delim);
    if (strcmp(token, "$") == 0)
    {
        token = strtok(NULL, delim);
    }
    while (token)
    {
        int is_cd = !strcmp(token, "cd");
        int is_ls = !strcmp(token, "ls\n");
        if (is_ls)
        {
            token = strtok(NULL, delim);
        }
        else if (is_cd)
        {
            char *target = strtok(NULL, delim);
            wd = fs_cd(wd, target);
        }
        else
        {
            // cmd output
            int is_dir = 0;
            size_t size = 0;
            if (strcmp(token, "dir") == 0)
            {
                is_dir = 1;
            }
            else
            {
                size = atoi(token);
            }
            char *name = strtok(NULL, delim);
            parse_ls(wd, name, is_dir, size);
        }

        token = strtok(NULL, delim);
    }
    return wd;
}
size_t sum_gt_thresh(file_t *wd, size_t threshold)
{
    if (wd->children == NULL)
    {
        return 0;
    }
    size_t total = 0;
    if (wd->size <= threshold)
    {
        total = wd->size;
        printf("%s|%lu\n", wd->name, wd->size);
    }
    for (size_t i = 0; i < wd->child_size; i++)
    {
        size_t size = sum_gt_thresh(wd->children[i], threshold);
        total += size;
    }
    return total;
}
size_t search_helper(file_t *wd, size_t len, file_t *candidates[len], size_t idx, size_t threshold)
{
    if (wd->children == NULL)
    {
        return idx;
    }
    if (wd->size >= threshold)
    {
        candidates[idx++] = wd;
    }
    else
    {
        return idx;
    }
    for (size_t i = 0; i < wd->child_size; i++)
    {
        idx = search_helper(wd->children[i], len, candidates, idx, threshold);
    }
    return idx;
}
size_t min(size_t len, file_t *candidates[len])
{
    size_t minimum = SSIZE_MAX;
    size_t current;
    for (size_t i = 0; i < len; i++)
    {
        current = candidates[i]->size;
        printf("%lu/%lu\n", current, minimum);
        if (current < minimum)
        {
            minimum = current;
        }
    }
    free(candidates);
    return minimum;
}
size_t find_del(file_t *wd)
{
    // assume < 100
    file_t **candidates = malloc(sizeof(file_t) * 100);
    size_t unused = TOTAL_SPACE - wd->size;
    size_t threshold = REQ_SPACE - unused;
    printf("Req to free: %lu\n", threshold);
    size_t idx = search_helper(wd, 100, candidates, 0, threshold);
    return min(idx, candidates);
}
int main()
{
    FILE *f = fopen("./data/7.txt", "r");
    char *line = malloc(sizeof(line) * 100);
    // char line[100];
    file_t *wd = create_fs();
    while (fgets(line, 100, f) != NULL)
    {
        wd = parse_cmd(wd, line);
    }
    fclose(f);
    wd = get_root(wd);
    // print_postorder(wd, 0);
    calculate_size(wd);
    print_postorder(wd, 0);
    size_t total = sum_gt_thresh(wd, THRESHOLD);
    printf("%lu\n", total);
    size_t req = find_del(wd);
    printf("%lu\n", req);
    destroy_fs(wd);
    free(line);
    return 0;
}
