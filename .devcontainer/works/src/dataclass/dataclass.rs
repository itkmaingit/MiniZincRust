pub type Points<T> = Vec<Vec<T>>;
pub type Cells<T> = Vec<Vec<T>>;
pub type HorizonEdges<T> = Vec<Vec<T>>;
pub type VerticalEdges<T> = Vec<Vec<T>>;

#[derive(Debug)]
pub struct Edges<T> {
    pub h: HorizonEdges<T>,
    pub v: VerticalEdges<T>,
}

impl<T> Edges<T> {
    // Edgesの初期化関数、誤っていればその時点でエラーを吐かせる。
    pub fn new(h: HorizonEdges<T>, v: VerticalEdges<T>) -> Result<Edges<T>, &'static str> {
        if h.len() != v.len() + 1 || h[0].len() + 1 != v[0].len() {
            return Err(
                "(Hの行数) = (Vの行数) + 1 かつ (Hの列数) + 1 = (Vの列数)でないといけません！",
            );
        } else {
            Ok(Edges { h, v })
        }
    }

    pub fn h_size(&self) -> (usize, usize) {
        return (self.h.len(), self.h[0].len());
    }

    pub fn v_size(&self) -> (usize, usize) {
        return (self.v.len(), self.v[0].len());
    }
}

#[derive(PartialEq, Debug)]
pub enum Attribute {
    H,
    V,
    P,
    C,
}
#[derive(Debug)]
pub struct Variable(pub Attribute, pub usize, pub usize);
