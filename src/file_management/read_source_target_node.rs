use super::SourceTargetNode;

pub struct ReadTargetNode {
    source_target: SourceTargetNode,
    content: String,
}

pub struct AugementedeTargetNode(ReadTargetNode);
