use areas_volumes::*;

pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {
    let area = x * y;
    let required_area = match objects {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };

    (required_area * times as f64) <= area as f64
}
pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {
    let volume = x * y * z;
    let rectangle_volume = match objects {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::Pyramid => triangular_pyramid_volume(a as f64),
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
        };

        (rectangle_volume * times as f64) <= volume as f64
    }
