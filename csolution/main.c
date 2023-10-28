#include <stdio.h>


int main(){
    FILE * fp;
    fp = fopen("../data/day01.txt", "r");

    printf(fp);
    return(0);
}