pub(crate) struct ASTNode {
    op: Token,
    left: Option<ASTNode>,
    right: Option<ASTNode>,
}

impl ASTNode {
    pub fn new(op: Token) -> Self {
        Self {
            op,
            left: None,
            right: None
        }
    }

    pub fn left(mut self, n: ASTNode) -> Self {
        self.left = Some(n);
        self
    }

    pub fn right(mut self, n: ASTNode) -> Self {
        self.right = Some(n);
        self
    }
}