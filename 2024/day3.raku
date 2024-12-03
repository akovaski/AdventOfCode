sub MAIN($input) {
    grammar Muls {
        token TOP { .*? <mul>+%.*? .* }
        token mul { "mul(" <number> "," <number> ")" }
        token number { \d+ }
    }

    my $parsedMuls = Muls.parsefile($input);
    my @muls = $parsedMuls<mul>.map({.<number>».Int});
    my $part-one-solution = @muls.map({[*] $_.List}).sum;
    say "part 1: $part-one-solution";

    grammar DontMuls {
        token TOP { .*? (<dont> || <dodo> || <mul>)+%.*? .* }
        token mul { "mul(" <number> "," <number> ")" }
        token number { \d+ }
        token dont { "don't()" }
        token dodo { "do()" }
    }

    my $parsedDontMuls = DontMuls.parsefile($input);
    my $enabled = True;
    my $part-two-solution = 0;
    for $parsedDontMuls[0] {
        if .<mul> and $enabled {
            $part-two-solution += [*] .<mul><number>».Int;
        }
        elsif .<dont> {
            $enabled = False;
        }
        elsif .<dodo> {
            $enabled = True;
        }
    }
    say "part 2: $part-two-solution";
}