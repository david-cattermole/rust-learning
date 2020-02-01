// Run a Rust function.
#include <iostream>
#include <vector>
#include <rustlib.h>
#include <cpplib.h>

int my_cpp_func(int num) {
    std::cout << "C++ Number is " << num << std::endl;
    return num + 1;
}


void create_stack() {
     std::vector<int> my_vec;
     my_vec.push_back(1);
     my_vec.push_back(2);
     my_vec.push_back(40);
     my_vec.push_back(41);
     my_vec.push_back(42);
     my_vec.push_back(43);
     std::vector<int>::iterator it;
     for (it = my_vec.begin(); it != my_vec.end(); ++it) {
          int c_num = *it;
          int rust_num = my_rust_func(c_num);
          int cpp_num = my_cpp_func(rust_num);
     }
     my_rust_func(42);
     return;
}
