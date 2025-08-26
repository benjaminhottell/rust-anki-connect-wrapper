use std::borrow::Cow;
use crate::endpoints::request::Request;

#[derive(Default)]
pub struct ModelNames;

impl Request for ModelNames {
    type Response = Vec<String>;
    type Params = ();
    fn get_action(&self) -> &'static str { "modelNames" }
}

#[derive(Default)]
pub struct ModelNamesAndIds;

impl Request for ModelNamesAndIds {
    type Response = std::collections::HashMap<String, u64>;
    type Params = ();
    fn get_action(&self) -> &'static str { "modelNamesAndIds" }
}

// TODO findModelsById
// TODO findModelsByName
// TODO modelFieldNames
// TODO modelFieldDescriptions
// TODO modelFieldFonts
// TODO modelFieldsOnTemplates
// TODO createModel
// TODO modelTemplates

/// Corresponds to the `modelStyling` action
#[derive(serde::Serialize)]
pub struct ModelStyling<'a> {
    #[serde(rename = "modelName")]
    model_name: Cow<'a, str>,
}

impl<'a> ModelStyling<'a> {
    pub fn new(model_name: impl Into<Cow<'a, str>>) -> Self {
        ModelStyling {
            model_name: model_name.into(),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ModelStylingResponse {
    pub css: String,
}

impl<'a> Request for ModelStyling<'a> {
    type Response = ModelStylingResponse;
    type Params = Self;
    fn get_action(&self) -> &'static str { "modelStyling" }
    fn get_params(&self) -> Option<&Self::Params> { Some(self) }
}

// TODO updateModelTemplates
// TODO updateModelStyling
// TODO findAndReplaceInModels
// TODO modelTemplateRename
// TODO modelTemplateReposition
// TODO modelTemplateAdd
// TODO modelTemplateRemove
// TODO modelFieldRename
// TODO modelFieldSetFont
// TODO modelFieldSetFontSize
// TODO modelFieldSetDescription
// TODO modelFieldAdd
// TODO modelFieldRemove
// TODO modelFieldSetFont
// TODO modelFieldSetFontSize
// TODO modelFieldSetDescription
