const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const args = try std.process.argsAlloc(allocator);
    const input_file_name = if (args.len >= 2) args[1] else "input.txt";

    const input_file = try std.fs.cwd().openFile(input_file_name, .{ .read = true });
    const input = input_file.reader();
    var buf: [300]u8 = undefined;
    var draw_buf: [100]u8 = undefined;

    const draw_line = try input.readUntilDelimiterOrEof(&buf, '\n');
    var num_draws: usize = 0;
    var draw_line_iter = std.mem.split(u8, draw_line.?, ",");
    while (draw_line_iter.next()) |draw| {
        draw_buf[num_draws] = try std.fmt.parseInt(u8, draw, 10);
        num_draws += 1;
    }
    const draws = draw_buf[0..num_draws];

    {
        const line = try input.readUntilDelimiterOrEof(&buf, '\n');
        std.debug.assert(line.?.len == 0);
    }

    const top_of_boards = try input_file.getPos();

    var num_lines: usize = 0;
    while ((try input.readUntilDelimiterOrEof(&buf, '\n')) != null) {
        num_lines += 1;
    }
    std.debug.assert(num_lines % 6 == 0);
    const num_boards = num_lines / 6;
    var boards = try allocator.alloc([25]u8, num_boards);

    try input_file.seekTo(top_of_boards);
    for (boards) |*board| {
        var board_pos: usize = 0;
        while (try input.readUntilDelimiterOrEof(&buf, '\n')) |line| {
            if (line.len == 0) {
                break;
            }
            std.debug.assert(line.len == 14);
            board[board_pos] = try parseIntRelax(line[0..2]);
            board_pos += 1;
            board[board_pos] = try parseIntRelax(line[3..5]);
            board_pos += 1;
            board[board_pos] = try parseIntRelax(line[6..8]);
            board_pos += 1;
            board[board_pos] = try parseIntRelax(line[9..11]);
            board_pos += 1;
            board[board_pos] = try parseIntRelax(line[12..14]);
            board_pos += 1;
        }
    }

    var bingos = try allocator.alloc(u128, num_boards * 12);
    const bingo_combinations = 10;
    for (boards) |*board, i| {
        // horizontal
        bingos[i * bingo_combinations + 0] = createBingo(board, .{ 0, 1, 2, 3, 4 });
        bingos[i * bingo_combinations + 1] = createBingo(board, .{ 5, 6, 7, 8, 9 });
        bingos[i * bingo_combinations + 2] = createBingo(board, .{ 10, 11, 12, 13, 14 });
        bingos[i * bingo_combinations + 3] = createBingo(board, .{ 15, 16, 17, 18, 19 });
        bingos[i * bingo_combinations + 4] = createBingo(board, .{ 20, 21, 22, 23, 24 });
        // vertical
        bingos[i * bingo_combinations + 5] = createBingo(board, .{ 0, 5, 10, 15, 20 });
        bingos[i * bingo_combinations + 6] = createBingo(board, .{ 1, 6, 11, 16, 21 });
        bingos[i * bingo_combinations + 7] = createBingo(board, .{ 2, 7, 12, 17, 22 });
        bingos[i * bingo_combinations + 8] = createBingo(board, .{ 3, 8, 13, 18, 23 });
        bingos[i * bingo_combinations + 9] = createBingo(board, .{ 4, 9, 14, 19, 24 });
    }

    var current_draws: u128 = 0;
    var bingo_board: ?usize = null;
    var last_draw: ?u8 = null;
    for (draws) |draw| {
        current_draws |= drawBit(draw);
        for (bingos) |bingo, i| {
            if (bingo & ~current_draws == 0) {
                bingo_board = i / bingo_combinations;
                last_draw = draw;
                std.log.info("BINGO! {}, {}", .{ i, bingo_board });
                break;
            }
        }
        if (bingo_board != null) {
            break;
        }
    }
    std.debug.assert(bingo_board != null);
    std.debug.assert(last_draw != null);

    const remaining = sumRemaining(current_draws, &boards[bingo_board.?]);
    std.log.info("remaining: {}, last draw: {}, final score: {}", .{ remaining, last_draw, remaining * last_draw.? });
}

fn parseIntRelax(s: []u8) !u8 {
    const number = if (s[0] == ' ') s[1..] else s;
    return try std.fmt.parseInt(u8, number, 10);
}

fn createBingo(board: *[25]u8, positions: [5]usize) u128 {
    var bingo: u128 = 0;
    for (positions) |pos| {
        bingo |= drawBit(board[pos]);
    }
    return bingo;
}

fn drawBit(draw: u8) u128 {
    return @as(u128, 1) << @intCast(u7, draw);
}

fn sumRemaining(current_draws: u128, board: *[25]u8) u64 {
    var board_bits: u128 = 0;
    for (board) |number| {
        board_bits |= drawBit(number);
    }
    var remaining_bits: u128 = board_bits & ~current_draws;
    var remaining_sum: u64 = 0;
    var i: u64 = 0;
    while (i < 128) {
        if (remaining_bits & 1 == 1) {
            remaining_sum += i;
        }
        remaining_bits >>= 1;
        i += 1;
    }
    return remaining_sum;
}
