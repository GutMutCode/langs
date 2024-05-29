const std = @import("std");

pub fn main() void {
    var input_buffer: InputBuffer = new_input_buffer();
    while (true) {
        print_prompt();
        read_input(input_buffer);
    }
}

const InputBuffer = struct { buffer: []u8, buffer_length: usize, input_length: usize };

/// create new input buffer
fn new_input_buffer() InputBuffer {
    return InputBuffer{
        .buffer = undefined,
        .buffer_length = 0,
        .input_length = 0,
    };
}

fn print_prompt() !void {
    try std.io.getStdOut().writeAll("db > ");
}

fn read_input(input_buffer: *InputBuffer) !void {
    _ = input_buffer;
}
