import std.stdio;
import std.string;
import std.algorithm;
import std.array;
import std.conv;

int solve(int x, int y) {
    return x - y;
}

string getInput() {
    return readln().strip();
}

int[] getInputOfIntArray() {
    return readln().split().map!(to!int).array;
}

void main() {
    auto line = getInputOfIntArray();

    int x = line[0];
    int y = line[1];

    int result = solve(x, y);
    writeln(result);
}
