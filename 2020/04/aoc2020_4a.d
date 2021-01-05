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
    return pp.byr != null &&
           pp.iyr != null &&
           pp.eyr != null &&
           pp.hgt != null &&
           pp.hcl != null &&
           pp.ecl != null &&
           pp.pid != null;
}
