const layer = @import("layer.zig");
const nll = @import("nll.zig");
const mnist = @import("mnist.zig");
const relu = @import("relu.zig");

const std = @import("std");

const INPUT_SIZE: u32 = 784;
const OUTPUT_SIZE: u32 = 10;
const BATCH_SIZE: u32 = 32;
const EPOCHS: u32 = 25;

pub fn main() !void {
    var allocator = std.heap.page_allocator;

    // Get MNIST data
    const mnist_data = try mnist.readMnist(&allocator);

    // Prep loss function
    const loss_function = nll.NLL(OUTPUT_SIZE);

    // Prep NN
    var layer1 = try layer.Layer(INPUT_SIZE, 100).init(&allocator);
    var relu1 = relu.Relu.new();
    var layer2 = try layer.Layer(100, OUTPUT_SIZE).init(&allocator);

    // Do training
    var e: usize = 0;
    while (e < EPOCHS) : (e += 1) {
        // Do training
        var i: usize = 0;
        while (i < 60_000 / BATCH_SIZE) : (i += 1) {
            // Prep inputs and targets
            const inputs = mnist_data.train_images[i * INPUT_SIZE * BATCH_SIZE .. (i + 1) * INPUT_SIZE * BATCH_SIZE];
            const targets = mnist_data.train_labels[i * BATCH_SIZE .. (i + 1) * BATCH_SIZE];

            // Go forward and get loss
            const outputs1 = try layer1.forward(inputs, &allocator);
            const outputs2 = try relu1.forward(outputs1, &allocator);
            const outputs3 = try layer2.forward(outputs2, &allocator);
            const loss = try loss_function.nll(outputs3, targets, &allocator);

            // Update network
            const grads1 = try layer2.backwards(loss.input_grads, &allocator);
            const grads2 = try relu1.backwards(grads1.input_grads, &allocator);
            const grads3 = try layer1.backwards(grads2, &allocator);
            layer1.applyGradients(grads3.weight_grads);
            layer2.applyGradients(grads1.weight_grads);

            // Free memory
            allocator.free(outputs1);
            allocator.free(outputs2);
            allocator.free(outputs3);
            grads1.destruct(&allocator);
            allocator.free(grads2);
            grads3.destruct(&allocator);
            loss.destruct(&allocator);
        }

        // Do validation
        i = 0;
        var correct: f64 = 0;
        const outputs1 = try layer.forward(mnist_data.test_images, &allocator);
        const outputs2 = try relu1.forward(outputs1, &allocator);
        const outputs3 = try layer2.forward(outputs2, &allocator);
        var b: usize = 0;
        while (b < 10_000) : (b += 1) {
            var max_guess: f64 = outputs3[b * OUTPUT_SIZE];
            var guess_index: usize = 0;
            for (outputs3[b * OUTPUT_SIZE .. (b + 1) * OUTPUT_SIZE], 0..) |o, oi| {
                if (o > max_guess) {
                    max_guess = o;
                    guess_index = oi;
                }
            }
            if (guess_index == mnist_data.test_labels[b]) {
                correct += 1;
            }
        }

        // Free memory
        allocator.free(outputs1);
        allocator.free(outputs2);
        allocator.free(outputs3);

        correct = correct / 10_000;
        std.debug.print("Average Validation Accuracy: {}\n", .{correct});
    }

    layer1.destruct(&allocator);
    layer2.destruct(&allocator);
    mnist_data.destruct(&allocator);
}
