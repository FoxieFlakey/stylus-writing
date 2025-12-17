use taffy::TaffyTree;

pub struct UI {
  tree: TaffyTree
}

impl UI {
  pub fn new() -> Self {
    Self {
      tree: TaffyTree::new()
    }
  }
}


