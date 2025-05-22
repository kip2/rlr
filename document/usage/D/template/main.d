import std.stdio;
import std.string;
import std.array;
import std.conv;

string solve(string input) {
    // todo: implement!
    return input;
}

string getInput() {
    return readln().strip();
}

int[] getInputOfIntArray() {
    return readln().split().map!(to!int).array;
}

void main() {
    // get input
    string result = solve(getInput());

    // print
    writeln(result);
}
