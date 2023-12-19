use itertools::Itertools;
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle};
use svg::{Document, Node};

#[allow(unused)]
pub fn plot_svg(trench_points: &[(i32, i32)], areas: &[(i32, i32, usize, usize)]) {
    let (row, col) = trench_points[0];
    let (min_row, max_row) = trench_points
        .iter()
        .map(|(row, _)| *row)
        .minmax()
        .into_option()
        .unwrap();

    let (min_col, max_col) = trench_points
        .iter()
        .map(|(_, col)| *col)
        .minmax()
        .into_option()
        .unwrap();

    let scale = 1000.0;
    let width = (max_col - min_col) as f64;
    let height = (max_row - min_row) as f64;
    let size = width.max(height);
    let tx = |x| scale * (x - min_col) as f64 / size;
    let ty = |y| scale * (y - min_row) as f64 / size;
    let mut data = Data::new().move_to((tx(col), ty(row)));

    for (row, col) in trench_points.iter().skip(1) {
        data = data.line_to((tx(*col), ty(*row)));
    }
    data = data.close();

    let path = Path::new()
        .set("fill", "black")
        .set("stroke", "none")
        .set("d", data);

    let mut document = Document::new()
        .set(
            "viewBox",
            (scale * -0.1, scale * -0.1, scale * 1.2, scale * 1.2),
        )
        .add(path);

    for (row, col, rows, cols) in areas {
        let rect = Rectangle::new()
            .set("x", tx(*col))
            .set("y", ty(*row))
            .set("width", scale * (*cols as f64) / size)
            .set("height", scale * (*rows as f64) / size)
            .set("fill", "orange")
            .set("fill-opacity", "0.5");

        document.append(rect);
    }

    svg::save("day18plot.svg", &document).unwrap();
}
