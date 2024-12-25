sub MAIN($input) {
    grammar Input {
        token TOP { [<lock>|<key>]+%"\n"+ "\n"* }
        token lock { "#####\n" [<[.#]>+]+%"\n" }
        token key { [<[.#]>+]+%"\n" }
    }
    my $parsed = Input.parsefile($input);

    my @locks = $parsed<lock>.map(-> $lock-match {
        my @lock;
        for $lock-match.Str.lines -> $line {
            my @line = $line.comb;
            for ^@line.elems {
                @lock[$_]++ if @line[$_] eq "#"
            }
        }
        @lock.map(* - 1).List
    });
    my @keys = $parsed<key>.map(-> $key-match {
        my @key;
        for $key-match.Str.lines.reverse -> $line {
            my @line = $line.comb;
            for ^@line.elems {
                @key[$_]++ if @line[$_] eq "#"
            }
        }
        @key.map(* - 1).List
    });

    my $part1-solution = 0;
    for @keys -> @key {
        lock: for @locks -> @lock {
            for ^5 {
                next lock if @key[$_] + @lock[$_] > 5;
            }
            $part1-solution++;
        }
    }
    say "part1 solution: $part1-solution";
}