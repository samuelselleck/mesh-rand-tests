use std::f64::consts::PI;

use indicatif::ProgressIterator;
use mesh_rand::{SurfSample, UniformSurface};
use rand::distributions::Distribution;

use plotters::prelude::*;
const OUT_FILE_NAME: &'static str = "3d-plot.gif";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Verticies and faces for a non-regular tetrahedron:
    let verticies = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];

    let faces = [[1, 0, 2], [2, 0, 3], [0, 1, 3], [1, 2, 3]];
    let mesh_dist = UniformSurface::new(&verticies, &faces)?;
    let mut rng = rand::thread_rng();

    let points: Vec<_> = (0..300)
        .into_iter()
        .map(|_| {
            let SurfSample { position: p, .. } = mesh_dist.sample(&mut rng);
            (p[0] as f64, p[1] as f64, p[2] as f64)
        })
        .collect();

    let area = BitMapBackend::gif(OUT_FILE_NAME, (600, 400), 16)?.into_drawing_area();

    for i in (0..360).progress() {
        area.fill(&WHITE)?;

        let x_axis = (-1.0..2.0).step(0.1);
        let y_axis = (-1.0..2.0).step(0.1);
        let z_axis = (-1.0..2.0).step(0.1);

        let mut chart = ChartBuilder::on(&area)
            .caption(format!("3D Plot Test"), ("sans", 20))
            .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;

        chart.with_projection(|mut pb| {
            pb.yaw = i as f64 * PI / 180.0;
            pb.scale = 0.9;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()?;

        let point_series = points
            .iter()
            .map(|&(x, y, z)| Circle::new((x, y, z), 1.2, BLUE.filled()));

        chart
            .draw_series(point_series)?
            .label("Surface")
            .legend(|(x, y)| {
                Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
            });

        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    }
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}
#[test]
fn entry_point() {
    main().unwrap()
}
