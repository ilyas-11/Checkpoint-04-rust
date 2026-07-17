
#[derive(Debug,PartialEq,Eq)]
pub struct Matrix(pub(i32,i32),pub(i32,i32));

pub fn multiply(m: Matrix, x: i32) -> Matrix {
    Matrix((m.0.0*x,m.0.1*x),(m.1.0*x,m.1.1*x))
}
