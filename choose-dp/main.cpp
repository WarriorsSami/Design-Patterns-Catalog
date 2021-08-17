#include <iostream>
#include <vector>
#include <string>
#include <experimental/random>
#include <chrono>
#include <thread>
#include <map>
#include <fstream>
#include <sstream>

using namespace std;
using namespace experimental;
using namespace chrono_literals;

ifstream buffer("store.txt");

vector<string> ProgrammingLanguages = {"Java", "C#", "C++", "Kotlin", "Golang", "Rust", "Python", "Typescript"};
vector<string> DesignPatterns = {"Singleton", "Builder", "Factory Method", "Abstract Factory", "Prototype",
                                 "Adapter", "Bridge", "Composite", "Decorator", "Facade", "Flyweight", "Proxy",
                                 "Chain of Responsibility", "Iterator", "Mediator", "Command", "Memento",
                                 "Observer", "State", "Strategy", "Template Method", "Visitor"};

map<pair<string, string>, bool> Store;
string line;

int main() {
    while (getline(buffer, line)) {
        istringstream read(line);
        string dp, lang;
        getline(read, dp, ',');
        getline(read, lang, ',');
        Store[make_pair(dp, lang)] = true;
        cout << dp << " " << lang << "\n";
    }

    cout << "This is a sample program to retrieve a random combination of a Design Pattern and a Programming "
            "Language from the list provided by you.\nYou are here to learn a brand new DP in a completely "
            "surprisingly manner.\nLet's see what we've got for you this time!";

    this_thread::sleep_for(2000ms);
    string dp, lang;

    cout << "\n----------------------------------\nHere is your job:\n";
    dp = DesignPatterns[randint(0, static_cast<int>(DesignPatterns.size() - 1))];
    lang = ProgrammingLanguages[randint(0, static_cast<int>(ProgrammingLanguages.size() - 1))];

    while (Store[make_pair(dp, lang)]) {
        cout << "-";
        this_thread::sleep_for(100ms);
        dp = DesignPatterns[randint(0, static_cast<int>(DesignPatterns.size() - 1))];
        lang = ProgrammingLanguages[randint(0, static_cast<int>(ProgrammingLanguages.size() - 1))];
    }

    cout << "\nYou have to implement the " << dp;
    cout << " Design Pattern in " << lang;
    cout << "\nGood luck!";

    return 0;
}
