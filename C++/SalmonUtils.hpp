#include<iostream>

class SalmonUtils {
  public:
      void println(string argv[]) {
        for (int i; argv[]; i++) {
          std::cout << argv[i]; 
        }
        std::cout << std::endl;
      }
};