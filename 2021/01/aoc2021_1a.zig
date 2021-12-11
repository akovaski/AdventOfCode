const std = @import("std");

pub fn main() !void {
    const inputFile = try std.fs.cwd().openFile("input.txt", .{ .read = true });
    const input = inputFile.reader();
    var buf: [10]u8 = undefined;
    var last: ?i32 = null;
    var increase_count: i32 = 0;
    while (try input.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        const value: i32 = try std.fmt.parseInt(i32, line, 10);
        //std.log.info("input: {}", .{value});
        if (last) |last_value| {
            if (value > last_value) {
                increase_count += 1;
                //std.log.info("increase", .{});
            }
        }
        last = value;
    }
    std.log.info("increase count: {}", .{increase_count});
}
