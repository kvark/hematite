
pub type Vector3 = [f32, ..3];
pub type Vector4 = [f32, ..4];
/// A matrix in row major format.
///
/// Notice that row major is mathematical standard,
/// while OpenGL uses column major format.
pub type Matrix3x4 = [[f32, ..4], ..3];
pub type Matrix4 = [[f32, ..4], ..4];
/// A matrix in column major format.
///
/// This format is nice for storing vertices of a quad.
pub type Base4x3 = [[f32, ..3], ..4];

#[inline(always)]
pub fn vec3_sub(a: Vector3, b: Vector3) -> Vector3 {
    [
        a[0] - b[0],
        a[1] - b[1],
        a[2] - b[2],
    ]
}

#[inline(always)]
pub fn vec3_add(a: Vector3, b: Vector3) -> Vector3 {
    [
        a[0] + b[0],
        a[1] + b[1],
        a[2] + b[2]
    ]
}

#[inline(always)]
pub fn vec3_dot(a: Vector3, b: Vector3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline(always)]
pub fn vec3_cross(a: Vector3, b: Vector3) -> Vector3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0]
    ]
}

#[inline(always)]
pub fn vec3_mul(a: Vector3, b: f32) -> Vector3 {
    [
        a[0] * b,
        a[1] * b,
        a[2] * b
    ]
}

#[inline(always)]
pub fn vec3_len(a: Vector3) -> f32 {
    vec3_dot(a, a).sqrt()
}

#[inline(always)]
pub fn vec3_inv_len(a: Vector3) -> f32 {
    1.0 / vec3_len(a)
}

#[inline(always)]
pub fn vec3_normalized(a: Vector3) -> Vector3 {
    vec3_mul(a, vec3_inv_len(a))
}

#[inline(always)]
pub fn vec3_normalized_sub(a: Vector3, b: Vector3) -> Vector3 {
    vec3_normalized(vec3_sub(a, b))
}

#[inline(always)]
pub fn vec4_dot_vec(a: Vector4, b: Vector3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline(always)]
pub fn vec4_dot_pos(a: Vector4, b: Vector3) -> f32 {
    vec4_dot_vec(a, b) + a[3]
}

#[inline(always)]
pub fn base4x3_row(base: Base4x3, i: uint) -> Vector4 {
    [base[0][i], base[1][i], base[2][i], base[3][i]]
}

#[inline(always)]
pub fn base4x3_mat(base: Base4x3) -> Matrix3x4 {
    [
        base4x3_row(base, 0),
        base4x3_row(base, 1),
        base4x3_row(base, 2)
    ]
}

#[inline(always)]
pub fn mat3x4_col(mat: Matrix3x4, i: uint) -> Vector3 {
    [mat[0][i], mat[1][i], mat[2][i]]
}

#[inline(always)]
pub fn mat3x4_transform_pos(mat: Matrix3x4, a: Vector3) -> Vector3 {
    [
        vec4_dot_pos(mat[0], a),
        vec4_dot_pos(mat[1], a),
        vec4_dot_pos(mat[2], a),
    ]
}

#[inline(always)]
pub fn mat3x_transform_vec(mat: Matrix3x4, a: Vector3) -> Vector3 {
    [
        vec4_dot_vec(mat[0], a),
        vec4_dot_vec(mat[1], a),
        vec4_dot_vec(mat[2], a)
    ]
}

#[inline(always)]
pub fn base4x3_from_f32_12(a: [f32, ..12]) -> Base4x3 {
    [
        [a[0], a[1], a[2]],
        [a[3], a[4], a[5]],
        [a[6], a[7], a[8]],
        [a[9], a[10], a[11]]
    ]
}

#[inline(always)]
pub fn base4x3_to_f32_12(base: Base4x3) -> [f32, ..12] {
    [
        base[0][0], base[0][1], base[0][2],
        base[1][0], base[1][1], base[1][2],
        base[2][0], base[2][1], base[2][2],
        base[3][0], base[3][1], base[3][2]
    ]
}

#[inline(always)]
pub fn mat3x4_transform_quad(
    mat: Matrix3x4,
    quad: [f32, ..12]
) -> [f32, ..12] {
    let a = base4x3_from_f32_12(quad);
    base4x3_to_f32_12([
        mat3x4_transform_pos(mat, a[0]),
        mat3x4_transform_pos(mat, a[1]),
        mat3x4_transform_pos(mat, a[2]),
        mat3x4_transform_pos(mat, a[3])
    ])
}

