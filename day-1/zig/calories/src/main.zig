const std = @import("std");
const fs = std.fs;

pub fn main() anyerror!void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var file = try std.fs.cwd().openFile("inputs.txt", .{});
    defer file.close();

    const buffer_size = 20000;
    // TODO: Figure out how to read a line at a time rather than reading entire file to the buffer
    const file_buffer = try file.readToEndAlloc(allocator, buffer_size);
    defer allocator.free(file_buffer);

    // Split by "\n" and iterate through the resulting slices of "const []u8"
    var iter = std.mem.split(u8, file_buffer, "\n");
    var highest_1: i32 = 0;
    var highest_2: i32 = 0;
    var highest_3: i32 = 0;
    var sum: i32 = 0;

    const stdout = std.io.getStdOut().writer();
    while (iter.next()) |line| {
        if (line.len == 0) {
            if (sum > highest_1) {
                highest_3 = highest_2;
                highest_2 = highest_1;
                highest_1 = sum;
            } else if (sum > highest_2) {
                highest_3 = highest_2;
                highest_2 = sum;
            } else if (sum > highest_3) {
                highest_3 = sum;
            }
            sum = 0;
            continue;
        }
        var num = try std.fmt.parseInt(i32, line, 10);
        sum += num;
    }
    if (sum > highest_1) {
        highest_3 = highest_2;
        highest_2 = highest_1;
        highest_1 = sum;
    } else if (sum > highest_2) {
        highest_3 = highest_2;
        highest_2 = sum;
    } else if (sum > highest_3) {
        highest_3 = sum;
    }
    var total = highest_1 + highest_2 + highest_3;
    try stdout.print("Highest is: {d}\n", .{highest_1});
    try stdout.print("Total is: {d}\n", .{total});
}
