sub MAIN($input) {
    grammar Input {
        token TOP { <order>+%"\n" "\n\n" <update>+%"\n" "\n"* }
        token order { (\d+) "|" (\d+) }
        token update { (\d+)+%"," }
    }

    my $parsedMuls = Input.parsefile($input);
    my %orders;
    my %reverse-orders;
    for $parsedMuls<order> {
        my $first = .[0].Int;
        my $second = .[1].Int;

        %orders{$first} //= [];
        %orders{$first}.push($second);

        %reverse-orders{$second} //= [];
        %reverse-orders{$second}.push($first);
    }
    
    my @updates = $parsedMuls<update>.map({.[0]».Int.List});

    my $part-one-solution = 0;
    my $part-two-solution = 0;
    for @updates -> @update {
        my $ordered = pages-are-ordered(@update, %orders);
        if $ordered {
            $part-one-solution += @update[@update.elems div 2];
        } else {
            my $update-set = set @update;
            my %update-orders = %orders.grep({.key.Int (elem) $update-set}).map({.key => .value.grep({$_ (elem) $update-set}).List});
            my %update-reverse-orders = %reverse-orders.grep({.key.Int (elem) $update-set}).map({.key => .value.grep({$_ (elem) $update-set}).List});
            my @ordered-update = make-ordered-list(%update-orders, %update-reverse-orders);
            $part-two-solution += @ordered-update[@ordered-update.elems div 2];
        }
    }
    say "part 1: $part-one-solution";
    say "part 2: $part-two-solution";
}

sub pages-are-ordered(@update, %orders) {
    my $previous = ().SetHash;
    for @update -> $page {
        if $previous{%orders{$page}.any} {
            return False;
        }
        $previous{$page}++;
    }
    return True;
}

sub make-ordered-list(%orders, %reverse-orders) {
    my @ordered = [];
    my %all-pages = set %orders.keys, %orders.values>>.map({$_>>.Str})>>.List;
    my %reverse-counts = %all-pages.map({.key => (%reverse-orders{.key} // ()).List.elems});
    return %reverse-counts.sort(+*.value).map(*.key.Int);
}
