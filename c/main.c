#include <stdio.h>  // standard in/out

int main(int argc, char const *argv[])
{
    // test to ensure build environment is working
    printf("hello world\n");

    // in order to solve the problems will need to make sure it can read files
    FILE *fptr;     // need a pointer of type file

    fptr = fopen("../../data/day01.txt","r");   // where are you calling the function from
            // add error handling 
    int c;
    if (fptr) {
    while ((c = getc(fptr)) != EOF)     // getc reads a character from a file
        putchar(c);     // write character to standard out
    }


    printf("%p\n",fptr);     // %d - int, %s - char, %p - pointer 
    fclose(fptr);

    return 0;   // return status anything but a 1 is bad
}
