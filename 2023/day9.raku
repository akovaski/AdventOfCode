use v6;

sub MAIN($input) {
    my $file = open $input;

    grammar Oasis {
        token TOP { <history>+%"\n" "\n"* }
        token history { <val>+%\h+ }
        token val { '-'? \d+ }
    }

    class OasisActions {
        method TOP ($/) { make $<history>».made }
        method history ($/) { make $<val>».made }
        method val ($/) { make $/.Int }
    }

    my $oasis = Oasis.parse($file.slurp, actions => OasisActions.new);
    my @histories = $oasis.made;
    my $part-one-solution;
    my $part-two-solution;
    sub revdiff { $^b - $^a }
    for @histories -> @history {
        my @values = @history;
        my @rightmosts = [@values.tail];
        my @leftmosts = [@values.head];
        while @values.all != 0 {
            @values = @values.tail(*-1) Z- @values.head(*-1);
            @rightmosts.push(@values.tail);
            @leftmosts.push(@values.head);
        }
        $part-one-solution += [+] @rightmosts;
        $part-two-solution += [[&revdiff]] @leftmosts.reverse;
    }
    say "part 1: $part-one-solution";
    say "part 2: $part-two-solution";
}
