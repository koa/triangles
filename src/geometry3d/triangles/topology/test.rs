use crate::geometry3d::point::bounding_box::BoundingBox3d;
use crate::prelude::{
    point_3d, IndexedTriangleList, Plane3d, Point3d, ReferencedTriangle, StaticLine3d,
    TriangleTopology,
};
use crate::test::load_schublade_as_triangles;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};
use stl_io::Vector;

#[test]
fn test_intersect() {
    let triangle_list: IndexedTriangleList<_> = load_schublade_as_triangles();
    let topolgy = TriangleTopology::new(&triangle_list).expect("Error on topology");
    let mut bbox: BoundingBox3d = Default::default();
    for p in triangle_list.points.iter() {
        bbox += p.coordinates();
    }
    dbg!(bbox);

    let line = StaticLine3d::new(point_3d(0.0, 100.0, 100.0), point_3d(0.0, -1.0, 0.0));
    let option = topolgy.find_first_intersection(&line);
    dbg!(option);
}

#[test]
fn test_find_planes() {
    let triangle_list: IndexedTriangleList<_> = load_schublade_as_triangles();
    let topology = TriangleTopology::new(&triangle_list).expect("Error on topology");
    let triangles = topology.triangles_of_plane();

    let grouped_planes = group_planes(triangles);
    println!("Group count: {}", grouped_planes.len());
}

fn group_planes<'a>(
    triangles: &'a HashMap<Plane3d, Vec<ReferencedTriangle<Vector<f32>>>>,
) -> HashMap<&'a Plane3d, Vec<&'a Plane3d>> {
    let mut planes = Vec::with_capacity(triangles.len());
    for first_plane in triangles.keys() {
        let mut density = 0.0;
        for second_plane in triangles.keys() {
            if first_plane == second_plane {
                continue;
            }
            let dist = first_plane.dist_square(second_plane);
            /*if dist > (100.0 * f32::EPSILON) as f64 {
                continue;
            }*/
            density += 1.0 / dist.0;
        }
        //println!("Plane: {first_plane:?}: {density}");
        planes.push((density, first_plane));
    }
    planes.sort_by_key(|(d, _)| OrderedFloat::from(-*d));

    let mut remaining_planes: HashSet<_> = triangles.keys().collect();
    let mut grouped_planes = HashMap::new();
    let dist_threshold = 0.1 * f64::EPSILON as f64;

    for (_, plane) in planes {
        if !remaining_planes.remove(plane) {
            continue;
        }
        let mut neighbors = vec![plane];
        println!("Plane: {plane:?}");
        let mut max_dist: f64 = 0.0;
        for candidate in remaining_planes.iter() {
            let x = plane.dist_square(candidate).0;
            if x < dist_threshold {
                println!("  {}: {candidate:?}: {x}", dist_threshold / x);
                max_dist = max_dist.max(x);
                neighbors.push(candidate);
            }
        }
        println!("Max Margin: {}", max_dist);
        for p in neighbors.iter() {
            remaining_planes.remove(p);
        }

        grouped_planes.insert(plane, neighbors);
    }
    grouped_planes
}
