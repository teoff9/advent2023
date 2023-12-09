#include <iostream>
#include <fstream>
#include <vector>
using namespace std;

void print_checked(vector<vector<bool>> checked) {
for (int z = 0; z<checked.size(); z++) {
        for (int n = 0; n< checked[z].size(); n++) {
            cout << checked[z][n] << " ";
        }
        cout << "\n";
    }
}

int explore_around(vector<vector<bool>> &checked,vector<string> text, int i, int k, vector<int>& res);

int get_number(vector<vector<bool>> &checked, string line, int j, int k);

int main() {
    fstream input;
    vector<string> text;
    vector<vector<bool>> checked;
    string tmp;
    int i = 0;
    int sum = 0;
    ofstream out;
    vector<int> res;
    out.open("output.txt");

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
            if (c!= '.' and !isdigit(c)) {
                sum += explore_around(checked, text, i, k, res);
            }
            k++;
        }
        i++;
    }

    for (int a : res) {
        out << a << '\n';
    }

    cout << sum << endl;

    return 0;
}


int explore_around(vector<vector<bool>> &checked,vector<string> text, int i, int k, vector<int>& res) {
    int s = 0;

    for (int j :  {-1, 0, 1}) {
        for (int m : {-1, 0, 1}) {
            if (j != 0 or m != 0) {
                if (isdigit(text[i+j][m+k]) && checked[i+j][m+k] == false) {
                    int a = get_number(checked, text[i+j],i+j, m+k);
                    res.push_back(a);
                    s += a;
                }
            }
        }
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
