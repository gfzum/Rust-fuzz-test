#include <iostream>
#include <string>

using namespace std;

void split_at_mut(string &s, int mid, string &a, string &b) {
    if (mid <= s.size()) {
        a = s.substr(0, mid);
        b = s.substr(mid);
    }
    else {
        throw out_of_range("mid too big!");
    }
}

int main() {
    int n;
    cin >> n;
    string s;
    cin >> s;
    int mid;
    cin >> mid;
    if (s.size() != n) {
        cout << "length not match" << endl;
    }
    else {
        string a, b;
        split_at_mut(s, mid, a, b);
        cout << a << endl;
        cout << b << endl;
    }
}