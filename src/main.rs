use plotly::{layout::{Axis, Margin}, Bar, ImageFormat, Layout, Plot};
use rand_distr::{Binomial, Distribution};

fn main() {
    let bin = Binomial::new(10000, 0.01).unwrap();

    let mut testResult: Vec<i32> = vec![0; 10001];
    for successCount in bin.sample_iter(&mut rand::thread_rng()).take(10000) {
        testResult[successCount as usize] += 1;
    }

    let t = Bar::new((0..=10000).collect(), testResult);
    let mut plot=Plot::new();
    plot.add_trace(t);

    let layout=Layout::new()
    .title("Binomial")
    .x_axis(Axis::new().title("Success Count").auto_range(false).range(vec![60,140]))
    .y_axis(Axis::new().title("Count").auto_range(false).range(vec![0,500]));

    plot.set_layout(layout);

    plot.write_image("output.png", ImageFormat::PNG, 1280 , 720, 1.0);
}
