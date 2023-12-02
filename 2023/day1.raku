use v6;

sub MAIN($input) {
    my $file = open $input;

    grammar First-Last {
        regex TOP { <-digit>* [ <digit> .* <digit> | <digit> ] <-digit>* }
        regex digit { \d }
    }

    grammar First-Last-Wordy {
        regex TOP { <-digit>* [ <digit> .* <digit> | <digit> ] <-digit>* }
        regex digit { \d || 'one' || 'two' || 'three' || 'four' || 'five' || 'six' || 'seven' || 'eight' || 'nine' }
    }

    my %word_to_int =
        'one' => '1',
        'two' => '2',
        'three' => '3',
        'four' => '4',
        'five' => '5',
        'six' => '6',
        'seven' => '7',
        'eight' => '8',
        'nine' => '9'
    ;
    my $total = 0;
    my $wordy-total = 0;
    for $file.lines -> $line {
        next unless $line; # ignore any empty lines
        First-Last.parse($line);
        $total += +"{$/<digit>.head}{$/<digit>.tail}";

        First-Last-Wordy.parse($line);
        my $wordy-first = $/<digit>.head;
        my $wordy-final = $/<digit>.tail;
        my $first = %word_to_int{$wordy-first}:exists ?? %word_to_int{$wordy-first} !! $wordy-first;
        my $final = %word_to_int{$wordy-final}:exists ?? %word_to_int{$wordy-final} !! $wordy-final;
        $wordy-total += +"$first$final";
    }

    say "part 1: $total";
    say "part 2: $wordy-total";
}