use crate::Piece;


pub fn for_each_tile<T, F>(board: &Vec<Vec<Option<Piece>>>, mut f: F) -> Result<Vec<T>, String>
where
    F: FnMut(usize, usize, &Option<Piece>) -> Result<T, String>,
{
    let mut result = Vec::new();
    let mut ir = 0;

    for row in board.iter() {
        let mut ic = 0;
        for tile in row.iter() {
            let res = f(ir, ic, tile)?;
            result.push(res);
            ic += 1;
        }
        ir += 1;
    }

    Ok(result)
}