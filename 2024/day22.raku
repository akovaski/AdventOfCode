sub MAIN($input) {
    my $file = open $input;
    my @initial-secrets := $file.slurp.trim.lines>>.Int.List;

    my $part1-solution = 0;
    for @initial-secrets -> $initial-secret {
        my $secret = $initial-secret;
        for ^2000 {
            $secret = advance-day($secret);
        }
        $part1-solution += $secret;
    }
    say "part1 solution: $part1-solution";


    my %change-lookup;
    for @initial-secrets -> $initial-secret {
        my $secret = $initial-secret;
        my %found-seq is SetHash;
        my @seq = [Nil, Nil, Nil, Nil];
        for ^2000 {
            my $new-secret = advance-day($secret);
            my $new-secret-price = $new-secret mod 10;
            my $change = $new-secret-price - ($secret mod 10);
            $secret = $new-secret;
            @seq.shift;
            @seq.push($change);
            if @seq[0].defined {
                my $seq-key = @seq.join(",");
                if %found-seq{$seq-key} {
                    next;
                }
                %found-seq{$seq-key}++;
                %change-lookup{$seq-key} = 0 if %change-lookup{$seq-key}:!exists;
                %change-lookup{$seq-key} += $new-secret-price;
            }
        }
    }
    my $part2-solution = %change-lookup.values.max;
    say "part2 solution: $part2-solution";
}

sub advance-day($secret) {
    my $step1 = ($secret * 64 +^ $secret) mod 16777216;
    my $step2 = ($step1 div 32 +^ $step1) mod 16777216;
    my $step3 = ($step2 * 2048 +^ $step2) mod 16777216;
    return $step3;
}