pub fn insert_new_head(body: &mut Vec<(i32, i32)>, new_point: (i32, i32)) -> Vec<(i32, i32)> {
    let mut new_body: Vec<(i32, i32)> = Vec::new();

    new_body.push((new_point.0, new_point.1));

    let mut old_body = body.clone();

    new_body.append(&mut old_body);

    return new_body;
}
