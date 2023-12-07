use v6;

sub MAIN($input) {
    my $file = open $input;

    grammar CamelCards {
        token TOP { <row>+%"\n" "\n"*}
        token row { <hand> " " <bid> }
        token hand { \S+ }
        token bid { \d+ }
    }

    my $camel-cards = CamelCards.parse($file.slurp);
    my @rows = $camel-cards<row>.map({ (.<hand>.Str, .<bid>.Int) });
    my @ranked-rows1 = @rows.sort({hand-strength($_[0], &hand-type-strength1, '23456789TJQKA'.comb)});
    my $part-one-solution = (@ranked-rows1»[1] Z* 1..*).sum;
    say "part 1: $part-one-solution";

    my @ranked-rows2 = @rows.sort({hand-strength($_[0], &hand-type-strength2, 'J23456789TQKA'.comb)});
    my $part-two-solution = (@ranked-rows2»[1] Z* 1..*).sum;
    say "part 2: $part-two-solution";
}

sub hand-strength($hand, &hand-type-strength, @card-strengths) {
    my $strength = &hand-type-strength($hand);
    for $hand.comb -> $card {
        $strength = $strength +< 8 + @card-strengths.first({ $_ eq $card }, :k);
    }
    return $strength;
}

sub hand-type-strength1($hand) {
    my @sorted = $hand.comb.sort;
    my @runs = [1];
    my $card = @sorted[0];
    for @sorted[1..*] -> $new-card {
        if $new-card eq $card {
            @runs.tail += 1;
        } else {
            @runs.push(1);
            $card = $new-card;
        }
    }
    return do given @runs.sort {
        when .[0] == 5 { 6 } # Five of a kind
        when .[1] == 4 { 5 } # Four of a kind
        when .[1] == 3 { 4 } # Full House
        when .[2] == 3 { 3 } # Three of a kind
        when .[1] == 2 { 2 } # Two pair
        when .[3] == 2 { 1 } # One pair
        default { 0 } # High card
    };
}

sub hand-type-strength2($hand) {
    my @sorted = $hand.comb.grep(none /J/).sort;
    if @sorted.elems == 0 {
        return 6;
    } else {
        my @runs = [1];
        my $card = @sorted[0];
        for @sorted[1..*] -> $new-card {
            if $new-card eq $card {
                @runs.tail += 1;
            } else {
                @runs.push(1);
                $card = $new-card;
            }
        }
        @runs.=sort;
        @runs.tail += 5 - @sorted.elems;
        return do given @runs {
            when .[0] == 5 { 6 } # Five of a kind
            when .[1] == 4 { 5 } # Four of a kind
            when .[1] == 3 { 4 } # Full House
            when .[2] == 3 { 3 } # Three of a kind
            when .[1] == 2 { 2 } # Two pair
            when .[3] == 2 { 1 } # One pair
            default { 0 } # High card
        };
    }
}