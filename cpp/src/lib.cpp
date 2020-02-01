// Run a Rust function.
#include <vector>
#include <rustlib.h>
#include <cpplib.h>

void my_cpp_function() {
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
          int rust_num = my_exported_func(c_num);
     }
     my_exported_func(42);
     return;
}
