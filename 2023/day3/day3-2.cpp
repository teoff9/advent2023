#include <iostream>
#include <fstream>
#include <vector>
using namespace std;

int explore_around(vector<vector<bool>> &checked,vector<string> text, int i, int k);

int get_number(vector<vector<bool>> &checked, string line, int j, int k);

int main() {
    fstream input;
    vector<string> text;
    vector<vector<bool>> checked;
    string tmp;
    int i = 0;
    int sum = 0;

    input.open("input.txt");
    
    while (input >> tmp) {
        string point = ".";
        text.push_back(point.append(tmp).append("."));
    }


    for (string line : text) {
        vector<bool> tmp;
        for (char c : line) {
            tmp.push_back(false);
        }
        checked.push_back(tmp);
    }

    for (string line : text) {
        int k = 0;

        for (char c : line) {
            if (c== '*') {
                sum += explore_around(checked, text, i, k);
            }
            k++;
        }
        i++;
    }

    cout << sum << endl;

    return 0;
}


int explore_around(vector<vector<bool>> &checked,vector<string> text, int i, int k) {
    int s = 0;
    vector<int> numbers;

    for (int j :  {-1, 0, 1}) {
        for (int m : {-1, 0, 1}) {
            if (j != 0 or m != 0) {
                if (isdigit(text[i+j][m+k]) && checked[i+j][m+k] == false) {
                    numbers.push_back(get_number(checked, text[i+j],i+j, m+k));
                }
            }
        }
    }

    if (numbers.size() == 2) {
        s += numbers[0] * numbers[1];
    }

    return s;
}


int get_number(vector<vector<bool>> &checked, string line,int j, int k) {
    string number = "";
    bool indexes[5] = {0, 0 ,1, 0, 0};

    for (int i : {-2, -1, 0, 1, 2}) {
        if (k+i >= 0 && k+i < line.size() && isdigit(line[k+i])) {
            number.push_back(line[k+i]);
            indexes[i+2] = true;
        } else if (k+i >=0 && k+i < line.size() && !isdigit(line[k+i])) {
            if (i<0) {
                number = "";
                indexes[0] = false;
                indexes[1] = false;
            } else if (i>0) {
                break;
            }
        }
    }


    for (int i = 0; i < 5; i++) {
        if (indexes[i] == true) {
            checked[j][k-2+i] = true;
        }
    }

    return stoi(number);
}
