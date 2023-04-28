#include<bits/stdc++.h>

using namespace std;

void split_at_mut(int *arr, int mid, int **a, int **b, int n){

    if(mid <= n){
        int i = 0;
        *a = (int *)malloc(mid * sizeof(int));
        *b = (int *)malloc((n - mid) * sizeof(int));
        for(i = 0; i < mid; i++){
            (*a)[i] = arr[i];
        }
        for(i = 0; i < n - mid; i++){
            (*b)[i] = arr[mid + i];
        }
    }
    else{
        int cc = 31;
        cout << "just for test use\n";
    }

}

int main(){

    int n = 0, mid = 0, i = 0;
    cin >> n;
    getchar();
    
    int *v = (int *)malloc(n * sizeof(int));
    int flag = 1;
    
    char line[100];
    cin.getline(line, 100);
    char * ptr = strtok(line, " ");
    while(ptr != NULL){
        if (i == n){
            flag = 0;
            break;
        }
        v[i++] = atoi(ptr);
        ptr = strtok(NULL, " ");
    }

    if (i != n || !flag){
        cout << "error" << endl;
        return 0;
    }

    scanf("%d", &mid);
    printf("split v into 2 slices at index %d\n", mid);

    int *a, *b;
    split_at_mut(v, mid, &a, &b, n);

    for(i = 0; i < mid; i++){
        printf("%d", a[i]);
        if(i != mid - 1){
            printf(", ");
        }
    }
    cout << endl;

    for(i = 0; i < n - mid; i++){
        printf("%d", b[i]);
        if(i != n - mid - 1){
            printf(", ");
        }
    }
    cout << endl;
}