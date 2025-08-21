use crate::client::Client;
use crate::error::Error;
use crate::models::RequestBody;

impl Client {

    // Model actions

    /// Invokes the `modelNames` action.
    pub async fn get_model_names(&self) -> Result<String, Error> {
        self.invoke(RequestBody::without_params("modelNames")).await
    }

    /// Invokes the `modelNamesAndIds` action.
    pub async fn get_model_names_and_ids(&self) -> Result<String, Error> {
        self.invoke(RequestBody::without_params("modelNamesAndIds")).await
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

}
