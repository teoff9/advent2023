//06.11.23 by Matteo Fava
//Advent of code Day 1 part 1

#include <fstream>
#include <iostream>
#include <vector>
using namespace std;

int main() {

    //variables
    ifstream input;
    string line;
    int first_n, last_n;
    bool is_first;
    int sum = 0;

    input.open("input.txt");
    if (input.fail()) { 
        cout << "Can't open" << endl;
        return -1;
    } else {
        while (input >> line) {
            is_first = true;
            first_n, last_n = 0;

            for (char tmp : line) {
                if (isdigit(tmp) && is_first) {
                    first_n = tmp -48;
                    last_n = first_n;
                    is_first = false;
                } else if (isdigit(tmp)) {
                    last_n = tmp - 48;
                }
            }

            sum += first_n * 10 + last_n;
        }
    }

    cout << sum << endl;

    return 0;
}