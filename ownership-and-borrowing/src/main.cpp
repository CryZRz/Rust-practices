#include <iostream>

int* refer_devol(){
    int numero = 2512;
    return &numero;
}

int main(){
    int num = 6;

    auto num3 = refer_devol();

    std::cout<<&num<<"\n";
    std::cout<<&num3<<"\n";
}