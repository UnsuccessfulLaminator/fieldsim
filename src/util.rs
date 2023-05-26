use nannou::geom::Vec2;
use crate::bodies::Body;



#[derive(PartialEq)]
enum AdaptStep {
    NoChange,
    Decrease,
    Increase
}

pub fn isopotential_points(
    body: &impl Body,
    r0: Vec2,
    min_dl: f32,
    max_dl: f32,
    max_err: f32,
    max_steps: usize
) -> Vec<Vec2> {
    let f = |r| body.e_field(r).normalize().perp();
    let stop = |r: Vec2, dl| (r-r0).length() < dl/2.;
    
    rk4_2d(f, r0, min_dl, max_dl, max_err, max_steps, stop)
}

fn rk4_2d(
    f: impl Fn(Vec2) -> Vec2,
    r0: Vec2,
    min_dl: f32,
    max_dl: f32,
    max_err: f32,
    max_steps: usize,
    stop: impl Fn(Vec2, f32) -> bool
) -> Vec<Vec2> {
    let mut points = Vec::new();
    let mut r = r0;
    let mut dl = max_dl;

    for _ in 0..max_steps {
        let (mut k1, mut k2, mut k3, mut k4);
        let mut adapt = AdaptStep::NoChange;

        loop {
            k1 = f(r);
            k2 = f(r+k1*0.5*dl);
            k3 = f(r+k2*0.5*dl);
            k4 = f(r+k3*dl);
            let err = (k1-4.*k2+2.*k3+k4).length()/6.;
            
            if err >= max_err {
                if dl/2. >= min_dl && adapt != AdaptStep::Increase {
                    dl /= 2.;
                    adapt = AdaptStep::Decrease;
                }
                else { break; }
            }
            else if err < max_err/10. {
                if dl*2. <= max_dl && adapt != AdaptStep::Decrease {
                    dl *= 2.;
                    adapt = AdaptStep::Increase;
                }
                else { break; }
            }
            else { break; }
        }
        
        r += (k1+2.*(k2+k3)+k4)*dl/6.;

        if r.is_finite() { points.push(r); }
        else { break; }
        
        if stop(r, dl) { break; }
    }

    points
}

// Find a series of points tracing out a field line from a given starting
// point, stopping when the line reaches a potential of too great a magnitude.
// Parameters:
//     bodies    - electric bodies producing the field lines
//     r0        - initial point
//     dl        - length step size between points
//     max_steps - line must terminate within this many steps of the start
// Returns:
//     A vector of points tracing out the field line, from lowest potential to
//     highest.
pub fn field_line_points(
    body: &impl Body,
    r0: Vec2,
    min_dl: f32,
    max_dl: f32,
    max_err: f32,
    max_steps: usize
) -> Vec<Vec2> {
    let f_forward = |r| body.e_field(r).normalize();
    let f_backward = |r| -body.e_field(r).normalize();
    let stop = |r, _dl| body.potential(r).abs() > 5e3;
    let mut points = rk4_2d(f_forward, r0, min_dl, max_dl, max_err, max_steps, stop);
    let backward_points = rk4_2d(f_backward, r0, min_dl, max_dl, max_err, max_steps, stop);
    
    points.reverse();
    points.push(r0);
    points.extend(backward_points.into_iter());

    points
}



/* THIS IS UNLIKELY TO BE USED BUT LEFT HERE IN CASE

// Find line segments corresponding to a level curve of a given surface.
// Parameters:
//     vals - 2D array of surface heights, x index first, y index second
//     xs   - x values that the x index corresponds to
//     ys   - y values that the y index corresponds to
//     z    - height of the level curve
// Returns:
//     A vector of line segments as (point, point) pairs, which together join
//     up to form a contour. The order of the line segments is not contiguous.
pub fn contour_lines(
    vals: &ArrayView2<f32>,
    xs: &ArrayView1<f32>,
    ys: &ArrayView1<f32>,
    z: f32
) -> Vec<(Vec2, Vec2)> {
    let mut lines = Vec::new();

    for i_x in 0..vals.nrows()-1 {
        for i_y in 0..vals.ncols()-1 {
            let (x0, y0) = (xs[i_x], ys[i_y]);
            let (x1, y1) = (xs[i_x+1], ys[i_y+1]);
            let r_00 = Vec3::new(x0, y0, vals[[i_x, i_y]]);
            let r_10 = Vec3::new(x1, y0, vals[[i_x+1, i_y]]);
            let r_01 = Vec3::new(x0, y1, vals[[i_x, i_y+1]]);
            let r_11 = Vec3::new(x1, y1, vals[[i_x+1, i_y+1]]);
            let r_avg = (r_00+r_10+r_01+r_11)/4.;

            let t0 = triangle_intersection(r_00, r_10, r_avg, z);
            let t1 = triangle_intersection(r_00, r_01, r_avg, z);
            let t2 = triangle_intersection(r_11, r_10, r_avg, z);
            let t3 = triangle_intersection(r_11, r_01, r_avg, z);

            lines.extend([t0, t1, t2, t3].into_iter().flatten());
        }
    }

    lines
}

// Helper function for contour_lines which finds the intersection of a level
// plane with an arbitrary triangle.
// Parameters:
//     r0, r1, r2 - triangle vertices
//     z          - height of the level plane
// Returns:
//     Optionally a line segment as a (point, point) pair, only if the plane
//     intersects the triangle.
fn triangle_intersection(r0: Vec3, r1: Vec3, r2: Vec3, z: f32) -> Option<(Vec2, Vec2)> {
    let inter01 = line_intersection(r0, r1, z);
    let inter12 = line_intersection(r1, r2, z);
    let inter02 = line_intersection(r0, r2, z);

    if let Some(inter01) = inter01 {
        if let Some(inter12) = inter12 {
            if inter02 == None { Some((inter01, inter12)) }
            else { None }
        }
        else {
            if let Some(inter02) = inter02 { Some((inter01, inter02)) }
            else { None }
        }
    }
    else {
        if let Some(inter12) = inter12 {
            if let Some(inter02) = inter02 { Some((inter12, inter02)) }
            else { None }
        }
        else { None }
    }
}

// Helper function for triangle_intersection which finds the intersection of a
// level plane with an arbitrary line.
// Parameters:
//     r0, r1 - ends of the line segment
//     z      - height of the level plane
// Returns:
//     Optionally the intersection point, if it exists.
fn line_intersection(r0: Vec3, r1: Vec3, z: f32) -> Option<Vec2> {
    if (r0[2] < z && r1[2] < z) | (r0[2] > z && r1[2] > z) { None }
    else if r0[2] == z && r1[2] == z { Some(r0.lerp(r1, 0.5).xy()) }
    else {
        let frac = (z-r0[2])/(r1[2]-r0[2]);
        
        Some(r0.lerp(r1, frac).xy())
    }
}*/
