const std = @import("std");

pub fn main() !void {
    const inputFile = try std.fs.cwd().openFile("input.txt", .{ .read = true });
    const input = inputFile.reader();
    var buf: [10]u8 = undefined;
    var increase_count: i32 = 0;
    var window: [3]i32 = undefined;
    var pos: usize = 0;
    while (try input.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const value: i32 = try std.fmt.parseInt(i32, line, 10);
        //std.log.info("input: {}", .{value});
        const i_a = pos % window.len;
        const i_b = (pos + 1) % window.len;
        const i_c = (pos + 2) % window.len;
        if (pos >= window.len) {
            const prev = window[0] + window[1] + window[2];
            const curr = value + window[i_b] + window[i_c];
            //std.log.info("curr: {}", .{curr});
            if (curr > prev) {
                increase_count += 1;
            }
        }
        window[i_a] = value;
        pos += 1;
    }
    std.log.info("increase count: {}", .{increase_count});
}
