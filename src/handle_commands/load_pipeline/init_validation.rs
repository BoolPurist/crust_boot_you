use crate::file_management::NodeEntryMeta;
use crate::handle_commands::load_pipeline::{many_node_metas_to_ids, NodeId};
#[cfg(test)]
use crate::prelude::testing::*;
use crate::prelude::*;
use crate::{cli::InitKind, file_management::SourceTargetNode};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub enum InitAction {
    NotEmpty,
    Conflict(PathBuf),
    NoConflict(Vec<SourceTargetNode>),
    Purge(Vec<SourceTargetNode>),
}

pub fn determine_init_action(
    init_kind: InitKind,
    write_target: &Path,
    template_source: &Path,
    on_query_template: impl Fn(&Path) -> AppIoResult<Vec<NodeEntryMeta>>,
) -> AppIoResult<InitAction> {
    info!(
        "Determine if target for project initialization is valid for init kind {}",
        init_kind
    );

    let nodes_write_target = on_query_template(write_target)?;
    return match init_kind {
        InitKind::OnlyEmpty => {
            if !nodes_write_target.is_empty() {
                Ok(InitAction::NotEmpty)
            } else {
                let templates =
                    all_templates_paths(template_source, write_target, on_query_template)?;
                info!("Target folder is empty and is valid for project initialization.");
                Ok(InitAction::NoConflict(templates))
            }
        }
        InitKind::NoNameConflicts => {
            let templates = all_templates_paths(template_source, write_target, on_query_template)?;
            let target: Vec<NodeId> = templates
                .iter()
                .map(|next| (next.target_path(), next.node_type()))
                .collect();

            let nodes_write_target: Vec<NodeId> = many_node_metas_to_ids(&nodes_write_target);

            let to_return = match has_name_conflicts(&target, &nodes_write_target) {
                None => {
                    info!("No name conflict detected. Initialization of project should not override any files.");
                    InitAction::NoConflict(templates)
                }
                Some((conflict_path, _)) => InitAction::Conflict(conflict_path.to_path_buf()),
            };

            Ok(to_return)
        }
        InitKind::Override => {
            let templates = all_templates_paths(template_source, write_target, on_query_template)?;
            info!("Initialization of project could override some files.");
            Ok(InitAction::NoConflict(templates))
        }
        InitKind::Purge => {
            let templates = all_templates_paths(template_source, write_target, on_query_template)?;
            info!("Target folder is wiped out before the initialization of the project.");
            Ok(InitAction::Purge(templates))
        }
    };

    fn all_templates_paths(
        template_source: &Path,
        write_target: &Path,
        on_query_template: impl Fn(&Path) -> AppIoResult<Vec<NodeEntryMeta>>,
    ) -> AppIoResult<Vec<SourceTargetNode>> {
        let templates = on_query_template(template_source)?;

        prepare_for_return(template_source, write_target, templates)
    }

    fn prepare_for_return(
        template_source: &Path,
        write_target: &Path,
        templates: Vec<NodeEntryMeta>,
    ) -> AppIoResult<Vec<SourceTargetNode>> {
        SourceTargetNode::opt_many_from_many_sources(template_source, write_target, templates)
            .ok_or_else(|| {
                AppIoError::custom("Could not get retrieve all paths from template files")
            })
    }
}

fn produce_set_from_paths<'a>(for_set: &'a [NodeId<'a>]) -> Option<HashSet<NodeId<'a>>> {
    let mut to_return: HashSet<NodeId<'a>> = Default::default();

    for to_insert in for_set {
        if !to_return.insert(*to_insert) {
            return None;
        }
    }
    Some(to_return)
}

fn has_name_conflicts<'a>(
    source: &'a [NodeId<'a>],
    target: &'a [NodeId<'a>],
) -> Option<NodeId<'a>> {
    let source_set = produce_set_from_paths(source)?;
    let target_set = produce_set_from_paths(target)?;
    let mut no_interection = source_set.intersection(&target_set);
    no_interection.next().copied()
}

#[cfg(test)]
mod testing {
    use super::*;

    use crate::{cli::InitKind, file_management::FileKind};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestInputDetermineAction {
        template_source: PathBuf,
        write_target: PathBuf,
        content_of_write_target: Vec<NodeEntryMeta>,
        content_of_template_folder: Vec<NodeEntryMeta>,
    }

    #[test]
    fn test_produce_set_from_uniques_paths() {
        let input: Vec<NodeEntryMeta> = from_ron_input_file!("list_to_set_unique_paths.ron");
        let input = many_node_metas_to_ids(&input);

        let mut actual: Vec<NodeId> = produce_set_from_paths(&input)
            .unwrap()
            .into_iter()
            .collect();

        actual.sort();
        let actual: Vec<NodeEntryMeta> = many_ids_to_meta_nodes(&actual);
        insta::assert_ron_snapshot!(actual);
    }
    #[test]
    fn test_none_for_unique_key() {
        let input: Vec<NodeEntryMeta> = from_ron_input_file!("list_to_set_not_unique_paths.ron");
        let input = many_node_metas_to_ids(&input);
        let actual = produce_set_from_paths(&input);
        assert!(actual.is_none());
    }

    #[test]
    fn test_detect_no_paths_intersection() {
        let (left, right): (Vec<NodeEntryMeta>, Vec<NodeEntryMeta>) =
            from_ron_input_file!("interection_unique.ron");
        let (left, right) = (
            many_node_metas_to_ids(&left),
            many_node_metas_to_ids(&right),
        );
        let actual = has_name_conflicts(&left, &right);
        assert!(actual.is_none());
    }
    #[test]
    fn test_detect_paths_intersection() {
        let (left, right): (Vec<NodeEntryMeta>, Vec<NodeEntryMeta>) =
            from_ron_input_file!("interection_not_unique.ron");
        let (left, right) = (
            many_node_metas_to_ids(&left),
            many_node_metas_to_ids(&right),
        );
        let actual = has_name_conflicts(&left, &right).unwrap();
        let expected = NodeEntryMeta::new(FileKind::File, PathBuf::from("/some/target/z"));
        assert_eq!(expected, single_id_to_node_entry_meta(actual));
    }

    #[test]
    fn test_detect_name_conflict() {
        let init_kind = InitKind::NoNameConflicts;
        let input: TestInputDetermineAction = from_ron_input_file!("with_name_conflicts.ron");
        let callback = callback_on_query_content(&input);
        let actual = determine_init_action(
            init_kind,
            &input.write_target,
            &input.template_source,
            callback,
        )
        .unwrap();
        insta::assert_ron_snapshot!(actual);
    }
    #[test]
    fn test_return_without_name_conflict() {
        let init_kind = InitKind::NoNameConflicts;
        let input: TestInputDetermineAction = from_ron_input_file!("without_name_conflicts.ron");
        let callback = callback_on_query_content(&input);
        let actual = determine_init_action(
            init_kind,
            &input.write_target,
            &input.template_source,
            callback,
        )
        .unwrap();
        insta::assert_ron_snapshot!(actual);
    }

    #[test]
    fn test_err_return_not_empty() {
        let init_kind = InitKind::OnlyEmpty;
        let input: TestInputDetermineAction = from_ron_input_file!("err_return_not_empty.ron");
        let actual = determine_init_action(
            init_kind,
            &input.write_target,
            Path::new("/a/b"),
            |actual_path| {
                if actual_path == input.write_target {
                    Ok(input.content_of_write_target.clone())
                } else {
                    panic!("Should not ask for all paths within the template source if {} is set and write target is not empty", init_kind)
                }
            },
        );
        assert!(matches!(actual, Ok(InitAction::NotEmpty)));
    }

    #[test]
    fn test_return_all_templates_for_empty_target() {
        let init_kind = InitKind::OnlyEmpty;
        let input: TestInputDetermineAction = from_ron_input_file!("test_return_if_empty.ron");
        let callback = callback_on_query_content(&input);
        let actual = determine_init_action(
            init_kind,
            &input.write_target,
            &input.template_source,
            callback,
        )
        .unwrap();
        insta::assert_ron_snapshot!(actual);
    }

    #[test]
    fn test_return_for_purge() {
        let init_kind = InitKind::Purge;
        let input: TestInputDetermineAction = from_ron_input_file!("with_name_conflicts.ron");
        let callback = callback_on_query_content(&input);
        let actual = determine_init_action(
            init_kind,
            &input.write_target,
            &input.template_source,
            callback,
        )
        .unwrap();
        insta::assert_ron_snapshot!(actual);
    }

    #[test]
    fn test_return_for_override() {
        let init_kind = InitKind::Override;
        let input: TestInputDetermineAction = from_ron_input_file!("with_name_conflicts.ron");
        let callback = callback_on_query_content(&input);
        let actual = determine_init_action(
            init_kind,
            &input.write_target,
            &input.template_source,
            callback,
        )
        .unwrap();
        insta::assert_ron_snapshot!(actual);
    }

    fn callback_on_query_content(
        input: &TestInputDetermineAction,
    ) -> impl Fn(&Path) -> AppIoResult<Vec<NodeEntryMeta>> + '_ {
        |actual_path| {
            if actual_path == input.write_target {
                Ok(input.content_of_write_target.clone())
            } else if actual_path == input.template_source {
                Ok(input.content_of_template_folder.clone())
            } else {
                panic!("Actual path {:?} is not valid", actual_path);
            }
        }
    }

    fn single_id_to_node_entry_meta(id: NodeId) -> NodeEntryMeta {
        NodeEntryMeta::new(id.1, id.0.to_path_buf())
    }

    fn many_ids_to_meta_nodes(many: &[NodeId]) -> Vec<NodeEntryMeta> {
        many.iter()
            .map(|&e| single_id_to_node_entry_meta(e))
            .collect()
    }
}
