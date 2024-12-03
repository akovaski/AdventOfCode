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

    grammar EnabledMuls {
        token TOP { .*? [<.disabled> || <mul>]+%.*? .* }
        token mul { "mul(" <number> "," <number> ")" }
        token number { \d+ }
        token disabled { "don't()" .*? ["do()" || $] }
    }

    my $parsedEnabledMuls = EnabledMuls.parsefile($input);
    my @enabledMuls = $parsedEnabledMuls<mul>.map({.<number>».Int});
    my $part-two-solution = @enabledMuls.map({[*] $_.List}).sum;
    say "part 2: $part-two-solution";
}