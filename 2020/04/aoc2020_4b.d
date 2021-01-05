import std.stdio;
import std.string;
import std.file;
import std.algorithm;
import std.array;
import std.conv;

struct passport {
    string byr = null;
    string iyr = null;
    string eyr = null;
    string hgt = null;
    string hcl = null;
    string ecl = null;
    string pid = null;
    string cid = null;
}

void main()
{
    auto input_file = readText("input.txt").splitLines();

    passport pp = {};
    int valid_count = 0;

    foreach (line; input_file) {
        if (line == "") {
            if (validPassport(pp)) {
                valid_count += 1;
            }
            pp = passport();
        } else {
            foreach (kv; line.split(' ')) {
                string[] kv_split = kv.split(':');
                string key = to!string(kv_split[0]);
                string value = to!string(kv_split[1]);
                pp = updatePassport(pp, key, value);
            }
        }
    }
    writefln("Valid passport count: %d", valid_count);
}

passport updatePassport(passport pp, string key, string value) {
    switch (key) {
        case "byr":
            pp.byr = value;
            break;
        case "iyr":
            pp.iyr = value;
            break;
        case "eyr":
            pp.eyr = value;
            break;
        case "hgt":
            pp.hgt = value;
            break;
        case "hcl":
            pp.hcl = value;
            break;
        case "ecl":
            pp.ecl = value;
            break;
        case "pid":
            pp.pid = value;
            break;
        case "cid":
            pp.cid = value;
            break;
        default:
            writefln("Invalid key: %s", key);
            assert(0);
    }
    return pp;
}


bool validPassport(passport pp) {
    return pp.byr != null && validIntRange(pp.byr, 1920, 2002) &&
           pp.iyr != null && validIntRange(pp.iyr, 2010, 2020) &&
           pp.eyr != null && validIntRange(pp.eyr, 2020, 2030) &&
           pp.hgt != null && validHeight(pp.hgt) &&
           pp.hcl != null && validHairColor(pp.hcl) &&
           pp.ecl != null && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].canFind(pp.ecl) &&
           pp.pid != null && pp.pid.length == 9 && pp.pid.validInt();
}

bool validInt(string s) {
    foreach(c; s) {
        if (c < '0' || c > '9') {
            return false;
        }
    }
    return true;
}

bool validIntRange(string s, int min, int max) {
    if (!validInt(s)) {
        return false;
    }
    int val = s.to!int();
    return val >= min && val <= max;
}

bool endsWith(string s, string suffix) {
    return s[s.length - suffix.length .. s.length] == suffix;
}

bool validHeight(string s) {
    if (s.endsWith("cm")) {
        return validIntRange(s[0 .. s.length - 2], 150, 193);
    } else if (s.endsWith("in")) {
        return validIntRange(s[0 .. s.length - 2], 59, 76);
    } else {
        return false;
    }
}

bool validHexChar(char c) {
    return ('0' <= c && c <= '9') || ('a' <= c && c <= 'f');
}
bool validHairColor(string s) {
    if (s.length != 7 || s[0] != '#') {
        return false;
    }
    foreach(c; s[1..7]) {
        if (!validHexChar(c)) {
            return false;
        }
    }
    return true;
}
