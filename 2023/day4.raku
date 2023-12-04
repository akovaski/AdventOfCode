use v6;

sub MAIN($input) {
    my $file = open $input;
    grammar Card {
        token TOP { 'Card' \s+ \d+ ':' \s+ <numbers> \s+ '|' \s+ <numbers> }
        token numbers { <number>+ % \s+ }
        token number { \d+ }
    }

    my $part-one-sum;
    my $part-two-sum;
    my @upcoming-copies;
    for $file.lines -> $line {
        Card.parse($line);
        my @winners = $/<numbers>[0]<number>>>.Int;
        my @haves = $/<numbers>[1]<number>>>.Int;
        my %matches = @winners ∩ @haves;
        $part-one-sum += 2 ** (%matches.elems - 1) if %matches.elems;

        my $current-count = (@upcoming-copies.shift // 0) + 1;
        @upcoming-copies[$_] += $current-count for ^%matches.elems;
        $part-two-sum += $current-count;
    }
    say "part 1: $part-one-sum";
    say "part 2: $part-two-sum";
}