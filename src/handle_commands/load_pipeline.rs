use std::borrow::Cow;

mod decoding;
mod init_validation;
use decoding::DecodedContent;

use crate::{
    cli::LoadTemplateArg,
    file_management::{
        write_transactions::{DirToEnsure, WriteTransactions},
        FileKind, NodeEntryMeta,
    },
    prelude::*,
    template_augmentation::TemplateAugmentor,
};

use self::{decoding::DecodedFile, init_validation::InitAction};

type NodeId<'a> = (&'a Path, FileKind);

type Text = Box<str>;
type ByteContent = Box<[u8]>;

type WithWarning = AppResult<Option<PathBuf>>;

pub fn init_project_with_template(
    file_manipulator: &impl FileManipulator,
    augmentor: &mut impl TemplateAugmentor,
    args: &LoadTemplateArg,
    write_target: &Path,
    template_source: &Path,
) -> WithWarning {
    let init_kind = *args.details().with();
    let action =
        init_validation::determine_init_action(init_kind, write_target, template_source, |path| {
            file_manipulator.all_nodes_inside(path)
        })?;

    match action {
        InitAction::NotEmpty => bail!(
            "Aborted initialization of project. Reason: target location at {:?} is not empty. This is not allowed for init kind {}", 
            write_target,
            init_kind
        ),
        InitAction::Conflict(conflict) => bail!(
            "Aborted initialization of project. Reason: found file/folder at {:?} prevents it for current init kind {}", 
            conflict,
            init_kind
        ),
        InitAction::NoConflict(data) => {
            let (files, dirs) = WriteTransactions::new(data).into();
            let decoded = decoding::decode_files(file_manipulator, files)?;
            let augmented = augment_loaded_files(augmentor, decoded, args)?;
            ensure_folders(file_manipulator, &dirs)?;
            write_loaded_files(file_manipulator, &augmented)
        }
        InitAction::Purge(data) => {
            let (files, dirs) = WriteTransactions::new(data).into();
            let decoded = decoding::decode_files(file_manipulator, files)?;
            let augmented = augment_loaded_files(augmentor, decoded, args)?;

            info!("Deleting target folder before project initialization because of purge. Location {:?}", write_target);
            file_manipulator.delete_whole_folder(write_target).with_context(|| {
               format!("Purge: failed to wipe out target folder before project initialization. Location: {:?}.", write_target) 
            })?;
            file_manipulator.ensure_dir(write_target).with_context(|| {
               format!("Purge: failed to create empty target folder after delteting it. Location: {:?}.", write_target) 
            })?;

            ensure_folders(file_manipulator, &dirs)?;
            write_loaded_files(file_manipulator, &augmented)
        }
    }
}

fn ensure_folders(file_manipulator: &impl FileManipulator, to_ensure: &[DirToEnsure]) -> AppResult {
    info!("Make sure all folders for project structure exits.");

    for next in to_ensure {
        let target = next.target();
        file_manipulator.ensure_dir(target).with_context(|| {
            format!(
                "Failed ensure the existence of folder at {:?} for initializing the project",
                target
            )
        })?;
    }

    Ok(())
}

fn augment_loaded_files(
    augmentor: &mut impl TemplateAugmentor,
    to_augment: Vec<DecodedFile>,
    args: &LoadTemplateArg,
) -> AppResult<Vec<DecodedFile>> {
    if *args.details().ignore_placeholders() {
        info!("Skipping replacing placeholders due the option ignore placeholders is activated",);
        return Ok(to_augment);
    }

    info!("Augmenting loaded files from template folders. Replacing placeholders");
    to_augment
        .into_iter()
        .map(|next| {
            let new_content: AppResult<Option<Box<str>>> = match next.content() {
                DecodedContent::Decoded(text) => {
                    let augmented = augmentor.try_replace(text).with_context(|| {
                        format!(
                            "Failed to augment file from ({:?}) for initializing the project",
                            next.source()
                        )
                    })?;
                    match augmented {
                        Cow::Borrowed(_) => Ok(None),
                        Cow::Owned(changed) => Ok(Some((*changed).into())),
                    }
                }
                DecodedContent::NotDecodedable(_bytes) => Ok(None),
            };

            let is_fine = match new_content? {
                Some(text) => next.new_content(text),
                None => next,
            };

            Ok(is_fine)
        })
        .collect()
}

fn write_loaded_files(
    file_manipulator: &impl FileManipulator,
    to_write: &[DecodedFile],
) -> WithWarning {
    info!("Write augmented files to the target location");
    let mut found_no_invalid_utf8: Option<PathBuf> = None;
    for next in to_write {
        let target = next.target();

        if found_no_invalid_utf8.is_none() && next.is_not_decodalbe() {
            found_no_invalid_utf8 = Some(next.source().to_path_buf());
        }

        file_manipulator
            .write_bytes(target, next.content().to_byte_ref())
            .with_context(|| format!("Failed to write file to location: {:?}", target))?;
    }

    Ok(found_no_invalid_utf8)
}

fn many_node_metas_to_ids(many: &[NodeEntryMeta]) -> Vec<NodeId> {
    many.iter().map(|e| e.into()).collect()
}
