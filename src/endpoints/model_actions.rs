use crate::endpoints::request::Request;

pub struct ModelNames;

impl Request for ModelNames {
    type Response = Vec<String>;
    type Params = ();
    fn get_action(&self) -> &'static str { "modelNames" }
}

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
// TODO modelStyling
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
