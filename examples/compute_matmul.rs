use std::{error::Error, fmt, io::IsTerminal, thread, time::Duration};

use metal::{
    MTLBuffer, MTLCommandBuffer, MTLCommandBufferExt, MTLCommandBufferStatus, MTLCommandEncoder, MTLCommandQueue,
    MTLComputeCommandEncoder, MTLDevice, MTLDeviceExt, MTLLibraryExt, MTLResourceOptions, MTLSize, MetalError,
};

const SIZE: usize = 64;
const COUNT: usize = SIZE * SIZE;
const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void matmul(device const float* input [[buffer(0)]],
                   device const float* weights [[buffer(1)]],
                   device float* output [[buffer(2)]],
                   constant uint& size [[buffer(3)]],
                   uint2 position [[thread_position_in_grid]]) {
    float sum = 0.0;
    for (uint inner = 0; inner < size; ++inner)
        sum += input[position.y * size + inner] * weights[inner * size + position.x];
    output[position.y * size + position.x] = sum;
}
"#;

#[derive(Debug)]
enum DemoError {
    Unavailable(&'static str),
    Metal(MetalError),
}

impl fmt::Display for DemoError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            Self::Unavailable(resource) => write!(formatter, "{resource} is unavailable"),
            Self::Metal(error) => error.fmt(formatter),
        }
    }
}

impl Error for DemoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Metal(error) => Some(error),
            Self::Unavailable(_) => None,
        }
    }
}

impl From<MetalError> for DemoError {
    fn from(error: MetalError) -> Self {
        Self::Metal(error)
    }
}

fn draw_matrices(
    matrices: &[Vec<f32>; 3],
    previous: Option<&[Vec<f32>; 3]>,
    stage: &str,
    device: &str,
    runs: usize,
) {
    let colors = ["38;2;125;211;252;48;2;18;42;55", "38;2;216;180;254;48;2;42;29;58", "38;2;134;239;172;48;2;17;48;35"];
    let border = "─".repeat(24);
    print!("\x1b[H\x1b[J");
    println!("\x1b[1;38;2;251;191;36mMTL-RS  /  METAL COMPUTE\x1b[0m");
    println!("\x1b[38;2;166;173;200m{device}  ·  one pipeline, four GPU dispatches\x1b[0m\n");
    println!("\x1b[1m{stage}\x1b[0m\n");
    println!("{:^26}     {:^26}     {:^26}", "INPUT X", "WEIGHTS W", "GPU RESULT Y");
    println!("\x1b[2m{:^26}     {:^26}     {:^26}\x1b[0m", "64×64 FP32", "64×64 FP32", "64×64 FP32");
    println!("\x1b[2m╭{border}╮     ╭{border}╮     ╭{border}╮\x1b[0m");
    for row in 0..4 {
        for (matrix_index, matrix) in matrices.iter().enumerate() {
            print!("\x1b[2m│\x1b[0m");
            for column in 0..4 {
                let index = row * SIZE + column;
                let changed = previous.is_some_and(|previous| previous[matrix_index][index] != matrix[index]);
                let style = if changed {
                    "38;2;253;230;138;48;2;77;48;19"
                } else if matrix[index] == 0.0 {
                    "38;2;77;86;106"
                } else {
                    colors[matrix_index]
                };
                print!("\x1b[{style}m{:>6.0}\x1b[0m", matrix[index]);
            }
            print!("\x1b[2m│\x1b[0m");
            match (matrix_index, row) {
                (0, 1) => print!("  ×  "),
                (1, 1) => print!("  =  "),
                (0 | 1, _) => print!("     "),
                _ => {},
            }
        }
        println!();
    }
    println!("\x1b[2m╰{border}╯     ╰{border}╯     ╰{border}╯\x1b[0m");
    println!("\x1b[2m4×4 previews  ·  amber cells changed since the last dispatch\x1b[0m\n");
    println!("\x1b[1;38;2;134;239;172m✓ All {COUNT} GPU values match the CPU reference\x1b[0m");
    println!("{runs} dispatches  ·  {} exact output checks", runs * COUNT);
}

fn main() -> Result<(), DemoError> {
    let interactive = std::io::stdout().is_terminal();
    let device = <dyn MTLDevice>::system_default().ok_or(DemoError::Unavailable("Metal device"))?;
    let queue = device.new_command_queue().ok_or(DemoError::Unavailable("command queue"))?;
    let library = device.new_library_with_source(SHADER, None)?;
    let function = library.new_function_with_name("matmul").expect("SHADER defines matmul");
    let pipeline = device.new_compute_pipeline_state_with_function(&function)?;
    let mut matrices = [
        (0..COUNT).map(|index| ((index + index / SIZE) % 9) as f32 - 4.0).collect::<Vec<_>>(),
        (0..COUNT).map(|index| ((index * 5 + index / SIZE * 3) % 7) as f32 - 3.0).collect(),
        vec![f32::NAN; COUNT],
    ];
    let buffers = matrices
        .iter()
        .map(|values| {
            device
                .new_buffer_with_data(bytemuck::cast_slice(values), MTLResourceOptions::STORAGE_MODE_SHARED)
                .ok_or(DemoError::Unavailable("matrix buffer"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let size_buffer = device
        .new_buffer_with_data(bytemuck::cast_slice(&[SIZE as u32]), MTLResourceOptions::STORAGE_MODE_SHARED)
        .ok_or(DemoError::Unavailable("size buffer"))?;
    let mut previous = None;
    let stages = [
        "01 / 04  Multiply on the GPU",
        "02 / 04  Edit one weight → a result column changes",
        "03 / 04  Edit an input → a result row changes",
        "04 / 04  Transpose the weights → recompute",
    ];
    for (step, stage) in stages.iter().enumerate() {
        match step {
            1 => matrices[1][SIZE + 1] = -matrices[1][SIZE + 1],
            2 => matrices[0][0] = -matrices[0][0],
            3 => {
                for row in 0..SIZE {
                    for column in 0..row {
                        matrices[1].swap(row * SIZE + column, column * SIZE + row);
                    }
                }
            },
            _ => {},
        }
        matrices[2].fill(f32::NAN);
        for (buffer, values) in buffers.iter().zip(&matrices) {
            // The preceding dispatch has completed before shared buffers are rewritten.
            unsafe {
                std::slice::from_raw_parts_mut(buffer.contents().as_ptr().cast::<f32>(), COUNT).copy_from_slice(values);
            }
        }
        let commands = queue.command_buffer().ok_or(DemoError::Unavailable("command buffer"))?;
        let encoder = commands.compute_command_encoder().ok_or(DemoError::Unavailable("compute encoder"))?;
        encoder.set_compute_pipeline_state(&pipeline);
        for (index, buffer) in buffers.iter().enumerate() {
            encoder.set_buffer(Some(buffer), 0, index);
        }
        encoder.set_buffer(Some(&size_buffer), 0, 3);
        encoder.dispatch_threads(
            MTLSize {
                width: SIZE,
                height: SIZE,
                depth: 1,
            },
            MTLSize {
                width: 8,
                height: 8,
                depth: 1,
            },
        );
        encoder.end_encoding();
        commands.commit();
        commands.wait_until_completed();
        if let Some(error) = commands.error() {
            return Err(error.into());
        }
        assert_eq!(commands.status(), MTLCommandBufferStatus::Completed);
        let result = unsafe { std::slice::from_raw_parts(buffers[2].contents().as_ptr().cast::<f32>(), COUNT) };
        matrices[2].copy_from_slice(result);
        for row in 0..SIZE {
            for column in 0..SIZE {
                let expected = (0..SIZE)
                    .map(|inner| matrices[0][row * SIZE + inner] * matrices[1][inner * SIZE + column])
                    .sum::<f32>();
                assert_eq!(
                    matrices[2][row * SIZE + column].to_bits(),
                    expected.to_bits(),
                    "{stage}: [{row}, {column}]"
                );
            }
        }
        if interactive {
            draw_matrices(&matrices, previous.as_ref(), stage, &device.name().to_string(), step + 1);
            thread::sleep(Duration::from_millis(1500));
            previous = Some(matrices.clone());
        } else {
            println!("{stage}: {COUNT} exact GPU output checks passed");
        }
    }
    println!("\n✓ {} exact GPU output checks passed", stages.len() * COUNT);
    Ok(())
}
