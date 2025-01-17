// Copyright (c) 2017 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use crate::command_buffer::synced::CommandBufferState;
use crate::pipeline::layout::PipelineLayout;
use crate::VulkanObject;
use std::error;
use std::fmt;

/// Checks whether push constants are compatible with the pipeline.
pub(in super::super) fn check_push_constants_validity(
    current_state: CommandBufferState,
    pipeline_layout: &PipelineLayout,
) -> Result<(), CheckPushConstantsValidityError> {
    if pipeline_layout.push_constant_ranges().is_empty() {
        return Ok(());
    }

    let constants_pipeline_layout = match current_state.push_constants_pipeline_layout() {
        Some(x) => x,
        None => return Err(CheckPushConstantsValidityError::MissingPushConstants),
    };

    if pipeline_layout.internal_object() != constants_pipeline_layout.internal_object()
        && pipeline_layout.push_constant_ranges()
            != constants_pipeline_layout.push_constant_ranges()
    {
        return Err(CheckPushConstantsValidityError::IncompatiblePushConstants);
    }

    let set_bytes = current_state.push_constants();

    if !pipeline_layout
        .push_constant_ranges()
        .iter()
        .all(|pc_range| set_bytes.contains(pc_range.offset..pc_range.offset + pc_range.size))
    {
        return Err(CheckPushConstantsValidityError::MissingPushConstants);
    }

    Ok(())
}

/// Error that can happen when checking push constants validity.
#[derive(Debug, Copy, Clone)]
pub enum CheckPushConstantsValidityError {
    /// The push constants are incompatible with the pipeline layout.
    IncompatiblePushConstants,
    /// Not all push constants used by the pipeline have been set.
    MissingPushConstants,
}

impl error::Error for CheckPushConstantsValidityError {}

impl fmt::Display for CheckPushConstantsValidityError {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CheckPushConstantsValidityError::IncompatiblePushConstants => {
                write!(
                    fmt,
                    "the push constants are incompatible with the pipeline layout"
                )
            }
            CheckPushConstantsValidityError::MissingPushConstants => {
                write!(
                    fmt,
                    "not all push constants used by the pipeline have been set"
                )
            }
        }
    }
}
