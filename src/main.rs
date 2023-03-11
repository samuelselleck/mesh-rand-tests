#![feature(iter_array_chunks)]
use std::f64::consts::PI;

use indicatif::ProgressIterator;
use mesh_rand::PoissonDiskSurface;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use plotters::prelude::*;
const OUT_FILE_NAME: &str = "3d-plot.gif";
fn main() -> Result<()> {
    let (verticies, faces) = load_model("models/newell_teaset/teapot.obj")?;
    let [(sx, bx), (sy, by), (sz, bz)] = get_bounds(&verticies);
    let (cx, cy, cz) = ((bx + sx) / 2.0, (by + sy) / 2.0, (bz + sz) / 2.0);
    let max_dim = (bx - sx).max(by - sy).max(bz - sz) / 2.0;
    let x_axis = ((cx - max_dim)..(cx + max_dim)).step(max_dim / 10.0);
    let y_axis = ((cy - max_dim)..(cy + max_dim)).step(max_dim / 10.0);
    let z_axis = ((cz - max_dim)..(cz + max_dim)).step(max_dim / 10.0);

    let mesh_dist = PoissonDiskSurface::new(&verticies, &faces)?;
    let mut rng = rand::thread_rng();

    println!("verts: {}, faces: {}", verticies.len(), faces.len());

    let points: Vec<[f64; 3]> = mesh_dist
        .sample_naive(0.1, 10000, 10000, &mut rng)
        .iter()
        .map(|p| p.map(|v| v as f64))
        .collect();

    println!("point count: {}", points.len());
    let area = BitMapBackend::gif(OUT_FILE_NAME, (600, 400), 16)?.into_drawing_area();

    let mut chart = ChartBuilder::on(&area)
        .caption("mesh-rand test", ("sans", 20))
        .build_cartesian_3d(x_axis, y_axis, z_axis)?;

    let point_series = points
        .iter()
        .map(|&[x, y, z]| Circle::new((x, y, z), 1.2, BLUE.filled()));

    for i in (0..360).progress() {
        area.fill(&WHITE)?;

        chart.with_projection(|mut pb| {
            let s = i as f64 * PI / 180.0;
            pb.yaw = s;
            pb.pitch = s.sin() * 0.2 + 0.25;
            pb.scale = 0.9;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()?;

        chart.draw_series(point_series.clone())?;
        //     .label("Surface")
        //     .legend(|(x, y)| {
        //         Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
        //     });

        // chart.configure_series_labels().border_style(BLACK).draw()?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    }
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}

fn get_bounds(verts: &[[f32; 3]]) -> [(f64, f64); 3] {
    let mut bounds = [(f64::MAX, f64::MIN); 3];
    for i in [0, 1, 2] {
        for v in verts {
            bounds[i].0 = bounds[i].0.min(v[i] as f64);
            bounds[i].1 = bounds[i].1.max(v[i] as f64);
        }
    }
    bounds
}

type Verts = Vec<[f32; 3]>;
type Faces = Vec<[usize; 3]>;

fn load_model(src: &str) -> Result<(Verts, Faces)> {
    let (models, _) = tobj::load_obj(
        src,
        &tobj::LoadOptions {
            single_index: false, //nah
            triangulate: true,   //yes please
            ignore_points: true,
            ignore_lines: true,
        },
    )?;

    let mut verticies = Vec::new();
    let mut triangles = Vec::new();
    for tobj::Model { mesh, .. } in models {
        let offset = verticies.len();
        let verts = mesh.positions.iter().copied().array_chunks();
        let tris = mesh
            .indices
            .iter()
            .map(|&v| v as usize + offset)
            .array_chunks();
        verticies.extend(verts);
        triangles.extend(tris);
    }

    Ok((verticies, triangles))
}

#[allow(dead_code)]
fn load_example() -> Result<(Verts, Faces)> {
    // Verticies and faces for a non-regular tetrahedron:
    let verticies = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ]
    .into();

    let faces = [[1, 0, 2], [2, 0, 3], [0, 1, 3], [1, 2, 3]].into();
    Ok((verticies, faces))
}
