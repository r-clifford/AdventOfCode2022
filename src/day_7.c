#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define INIT_CAP 20

void strip_newline(char* s) {
    size_t len = strlen(s);
    if (s[len-1] == '\n') {
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
typedef enum
{
    CMD_CD,
    CMD_LS,
} CMD;
file_t *create_fs()
{
    file_t *root = malloc(sizeof(*root));
    root->parent = NULL;
    root->name = "/";
    root->size = 0;
    root->child_size = 0;
    root->is_dir = 1;
    root->capacity = INIT_CAP;
    root->children = malloc(sizeof(*root) * root->capacity);
    return root;
}
void pad(size_t n) {
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
    pad(depth);
    printf("- %s (dir)\n", wd->name);
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
            total += calculate_size(wd->children[i]);
        }
        else
        {
            total += wd->size;
        }
    }
    return total;
}
char **get_children_names(file_t *wd)
{
    char **children = malloc(wd->child_size * sizeof(*children));
    for (size_t i = 0; i < wd->child_size; i++)
    {
        children[i] = wd->children[i]->name;
    }
    return children;
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
    get_root(wd->parent);
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
    CMD cmd = CMD_LS;
    while (token)
    {
        int is_cd = !strcmp(token, "cd");
        int is_ls = !strcmp(token, "ls\n");
        if (is_ls)
        {
            token = strtok(NULL, delim);
            cmd = CMD_LS;
        }
        else if (is_cd)
        {
            char *target = strtok(NULL, delim);
            wd = fs_cd(wd, target);
            cmd = CMD_CD;
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
int main()
{
    FILE *f = fopen("./data/test.txt", "r");
       char *line = malloc(sizeof(line) * 100);
    // char line[100];
    file_t *wd = create_fs();
    while (fgets(line, 100, f) != NULL)
    {
        wd = parse_cmd(wd, line);
    }
    wd = get_root(wd);
    print_postorder(wd, 0);
    //    char** children = get_children_names(wd);
    //    for (size_t i = 0; i < wd->child_size; i++) {
    //        printf("%s\n", children[i]);
    //    }
    return 0;
}
